
use libltntstools_sys::*;
use std::ffi::c_void;
use std::ffi::c_int;
use std::ffi::c_uchar;
//use std::ops::Deref;

/*
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
*/

pub extern "C" fn basic_pe_callback(_user_context: *mut c_void, pes: *mut ltn_pes_packet_s)
{
    unsafe {
        
        //println!("PTS = {}", (*pes).PTS);
        let s: i8 = 0;
        ltn_pes_packet_dump(pes, &s);

		/* Pass the PES via a rust callback and let rust own it */
        ltn_pes_packet_free(pes);
    };
}

#[derive(Debug)]
pub struct PesExtractor
{
	verbose: bool,
	handle: *mut c_void,
	pid: u16,
	streamid: u8,
}
impl Default for PesExtractor {
    fn default() -> Self {
        PesExtractor {
            verbose: false,
			handle: std::ptr::null_mut(),
			pid: 0,
			streamid: 0,
        }
    }
}
impl Drop for PesExtractor {
	fn drop(&mut self) {
		unsafe {
			pes_extractor_free(self.handle);
		};
	}
}
impl PesExtractor {
	pub fn new(verbose: bool, pid: u16, streamid: u8) -> PesExtractor {
		let mut ctx = PesExtractor::default();
		ctx.verbose = verbose;
		ctx.pid = pid;
		ctx.streamid = streamid;
		unsafe {
			pes_extractor_alloc(&mut ctx.handle as _, ctx.pid, ctx.streamid, Some(basic_pe_callback), std::ptr::null_mut());
		};

		return ctx;
	}
	pub fn write(&mut self, pkt: *const c_uchar, packet_count: c_int) {
	
		unsafe {
			pes_extractor_write(self.handle, pkt, packet_count);
		};
	
	}
}
