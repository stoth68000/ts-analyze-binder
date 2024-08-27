
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

pub struct StreamStatistics
{
	verbose: bool,
	handle: Box<stream_statistics_s>,
}

impl Default for StreamStatistics {
	fn default() -> Self {
	    Self::new(false)
	}
}

impl std::fmt::Debug for StreamStatistics {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		f.debug_struct("StreamStatistics")
			.field("verbose", &self.verbose)
			.field("handle", &(&*self.handle as *const _)) // format as a pointer
			.finish()
	}
}

impl StreamStatistics {
	pub fn new(verbose: bool) -> StreamStatistics {
		let handle = unsafe {
		    let stats_layout = std::alloc::Layout::new::<stream_statistics_s>();
		    let stats_ptr = std::alloc::alloc(stats_layout);
		    std::ptr::write_bytes(stats_ptr, 0, stats_layout.size());
		    Box::from_raw(stats_ptr as *mut stream_statistics_s)
		};

		let mut ctx = Self { verbose, handle };
		ctx.reset();
		ctx
	}

	pub fn reset(&mut self) {
		if self.verbose {
			println!("StreamStatistics::reset()");
		}
		unsafe {
			pid_stats_reset(&mut *self.handle)
		}
	}
	#[allow(dead_code)]
	pub fn write(&mut self, pkt: *const c_uchar, packet_count: u32) {	
		if self.verbose {
			println!("StreamStatistics::write(?, ?, {})", packet_count);
		}
		unsafe {
			pid_stats_update(&mut *self.handle, pkt, packet_count);
		};
	}
	#[allow(dead_code)]
	pub fn dprintf(&mut self, fd: i32) {
		if self.verbose {
			println!("StreamStatistics::dprintf(?, {})", fd);
		}
		unsafe {
			pid_stats_dprintf(&mut *self.handle, fd);
		}
	}
}
