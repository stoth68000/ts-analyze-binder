
use libltntstools_sys::*;
use std::ffi::c_void;
use std::ffi::c_int;
use std::ffi::c_uchar;
use std::ops::Deref;

pub struct Pat(*mut pat_s);
impl Drop for Pat {
	fn drop(&mut self) {
		unsafe {
			pat_free(self.0);
		}
	}
}
impl Deref for Pat {
	type Target = pat_s;
	fn deref(&self) -> &Self::Target {
		unsafe {
			&*self.0 /* this is the value of the block, it returns */
		}
	}
}
impl Pat {
	#[allow(dead_code)]
	pub fn print(&self) {
		unsafe {
			pat_dprintf(self.0, 1);
		}
	}
}

#[derive(Debug)]
pub struct StreamModel
{
	verbose: bool,
	handle: *mut c_void,
	last_write_did_complete: bool,
}
impl Default for StreamModel {
    fn default() -> Self {
        StreamModel {
            verbose: false,
			handle: std::ptr::null_mut(),
			last_write_did_complete: false,
        }
    }
}
impl Drop for StreamModel {
	fn drop(&mut self) {
		unsafe {
			streammodel_free(self.handle);
		};
	}
}
impl StreamModel {
	pub fn new(verbose: bool) -> StreamModel {
		let mut ctx = StreamModel::default();
		ctx.verbose = verbose;
		unsafe {
			streammodel_alloc(&mut ctx.handle /* as _ */, std::ptr::null_mut());
		};

		return ctx;
	}
	pub fn write(&mut self, pkt: *const c_uchar, packet_count: c_int, complete: &mut bool) {
		unsafe {
			
			let mut val: i32 = 0;
			let val_ptr = &mut val as *mut c_int;
			streammodel_write(self.handle, pkt, packet_count, val_ptr);

			if val == 1 {
				let mut pat = std::ptr::null_mut();
				streammodel_query_model(self.handle, &mut pat as _);

				/* Display the entire pat, pmt and descriptor model to console */
				pat_dprintf(pat, 1);
				pat_free(pat);
				self.last_write_did_complete = true;
				(*complete) = true;
			} else {
				self.last_write_did_complete = false;
				(*complete) = false;
			}
		};
	}
	pub fn query_model(&mut self) -> Pat {

		if self.last_write_did_complete == false {
			//return None; /* What to do where? - We need to return a none or something */
		}
		/* The promise is, the last _write call resulted in a complete == 1, don't
		 * call this until then.
		 */
		unsafe {
			let mut pat = std::ptr::null_mut();
			streammodel_query_model(self.handle, &mut pat as _);

			/* Display the entire pat, pmt and descriptor model to console */
			if self.verbose {
				pat_dprintf(pat, 1);
				//pat_free(pat); /* caller owns the lifespan now */
			}

			return Pat(pat);
		};

	}
}
