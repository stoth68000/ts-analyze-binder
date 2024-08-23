//use std::fmt::{self, Display, Formatter};

#[derive(Default,Debug)]
pub enum InputType {
	#[default]
    InputUnknown,
    InputFile,
    //InputAvio,
    //InputPcap,
}
/* The #[derive(Default,Debug)] above implements this by default.
impl Display for InputType {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::InputUnknown => write!(f, "InputUnknown"),
			Self::InputFile => write!(f, "InputFile"),
			Self::InputAvio => write!(f, "InputAvio"),
			Self::InputPcap => write!(f, "InputPcap"),
		}
	}
}
*/