use core::ffi::c_void;

use super::inputtype::InputType;

#[derive(Debug)]
pub struct ToolContext
{
	pub verbose: u32,
	pub process_all: bool,
	pub input: String,
	pub input_type: InputType,
	pub handle: *mut c_void,
}
impl Default for ToolContext {
    fn default() -> Self {
        ToolContext {
            verbose: 0,
            process_all: false,
            input: String::new(),
			input_type: InputType::InputFile,
			handle: std::ptr::null_mut(),
        }
    }
}
