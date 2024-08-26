
use libltntstools_sys::*;
//use std::ffi::c_void;
//use std::ffi::c_int;
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

#[derive(Debug)]
pub struct StreamStatistics
{
	verbose: bool,
	handle: *mut stream_statistics_s,
}
impl Default for StreamStatistics {
    fn default() -> Self {
        StreamStatistics {
            verbose: false,
			handle: std::ptr::null_mut(),
        }
    }
}
impl Drop for StreamStatistics {
	fn drop(&mut self) {
		// ctx.handle should automatically be released
	}
}
impl StreamStatistics {
	pub fn new(verbose: bool) -> StreamStatistics {
		let mut ctx = StreamStatistics::default();
		ctx.verbose = verbose;

		unsafe {
			let mut stats = {
				let stats_layout = std::alloc::Layout::new::<stream_statistics_s>();
				let stats_ptr = std::alloc::alloc(stats_layout);
				std::ptr::write_bytes(stats_ptr, 0, stats_layout.size());
				Box::from_raw(stats_ptr as *mut stream_statistics_s)
			};
			ctx.handle = stats.as_mut();
		};

		ctx.reset();

		return ctx;
	}
	#[allow(dead_code)]
	pub fn reset(&mut self) {
		if self.verbose {
			println!("StreamStatistics::reset() pre");
		}
		unsafe {
			pid_stats_reset(self.handle);
		}
		if self.verbose {
			println!("StreamStatistics::reset() post");
		}
	}
	#[allow(dead_code)]
	pub fn write(&mut self, pkt: *const c_uchar, packet_count: u32) {	
		if self.verbose {
			println!("StreamStatistics::write(?, ?, {})", packet_count);
		}
		unsafe {
			pid_stats_update(self.handle, pkt, packet_count);
		};
	}
	#[allow(dead_code)]
	pub fn dprintf(&mut self, fd: i32) {
		if self.verbose {
			println!("StreamStatistics::dprintf(?, {})", fd);
		}
		unsafe {
			pid_stats_dprintf(self.handle, fd);
		}
	}
}
