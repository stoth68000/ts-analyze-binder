use std::env;
use std::process::exit;
use std::io::Read;

mod copyright;
mod inputtype;
mod streammodel;
mod toolcontext;

// use module:Type
use toolcontext::ToolContext;
use inputtype::InputType;
use streammodel::*;

fn show_usage()
{
	println!("{}", copyright::LTN_COPYRIGHT);
	println!("Version: v1.31.1"); // Drive this from the cargo.toml?
	println!("A tool to display the PAT/PMT transport tree structures from file.");
	println!("The first PAT and first set of PMTs are displayed, then the program terminates.");
	println!("Usage:");
	println!("    -i, --input <filename>");
	println!("    -a don't terminate after the first model is obtained");
	println!("    -v Increase level of verbosity (enable descriptor dumping).");
	println!("    -h, --help Display command line help.");
}

fn main() {

	let mut ctx = ToolContext::default();

	if env::args().count() < 2 {
		show_usage();
		exit(1);
	}

	let mut args = env::args().skip(1);

	while let Some(arg) = args.next() {
		match &arg[..] {
			"-a" | "--all" => {
				ctx.process_all = true;
			},
			"-h" | "--help" => {
				show_usage();
				exit(1);
			},
			"-i" | "--input" => {
				if let Some(arg_config) = args.next() {
					ctx.input = arg_config;
					ctx.input_type = InputType::InputFile;
				} else {
					panic!("No value specified for parameter -i, --input, aborting.");
				}
			},
			"-v" | "--verbose" => {
				ctx.verbose += 1;
			},
			_ => {
			},
		}
	}

	if ctx.input.chars().count() < 1 {
		show_usage();
		eprintln!("\n-i, --input is mandatory, aborting");
		exit(1);
	}
	println!("args: {ctx:?}");
	println!();

	// Why does this have to be mutable?
	let mut sm = StreamModel::new(false);

	let mut file_in = std::fs::File::open(ctx.input).unwrap();
	let mut buffer = [0u8; 128 * 188];
	let mut processed = 0;
	
    loop {
        let nbytes = file_in.read(&mut buffer).unwrap();
        if nbytes < buffer.len() {
            break;
        }
        processed += nbytes;

		if ctx.verbose > 0 {
        	println!("StreamModel - Read {} / {} bytes", nbytes, processed);
		}

        let b: i32 = nbytes.try_into().unwrap();

		let mut complete = false;

		sm.write(&buffer[0], b / 188, &mut complete);

		if complete == true {
			let pat = sm.query_model();
			pat.print();

			println!("PAT.program_count = {}", pat.program_count);

			if ctx.process_all == false {
				break;
			}
		}
	}

}

