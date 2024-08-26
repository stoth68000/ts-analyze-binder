use core::ffi::c_void;

use super::inputtype::InputType;

#[derive(Debug)]
pub struct ToolContext
{
	/* si_streammodel */
	pub verbose: u32,
	pub process_all: bool,
	pub input: String,
	pub input_type: InputType,
	pub handle: *mut c_void,

	/* pes_extractor */
	pub pid: u16,
	pub streamid: u8,
}
impl Default for ToolContext {
    fn default() -> Self {
        ToolContext {
			/* si_streammodel */
            verbose: 0,
            process_all: false,
            input: String::new(),
			input_type: InputType::InputFile,
			handle: std::ptr::null_mut(),
			
			/* pes_extractor */
			pid: 0,
			streamid: 0,
        }
    }
}
