use std::env;
use std::process::exit;
use std::io::Read;
use std::path::Path;

mod copyright;
mod inputtype;
mod streammodel;
mod toolcontext;
mod pesextractor;
mod streamstatistics;

// use module:Type
use toolcontext::ToolContext;
use inputtype::InputType;
use streammodel::*;
use pesextractor::*;
use streamstatistics::*;

fn show_usage_si_streammodel()
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

#[allow(dead_code)]
fn main_si_streammodel() {

	let mut ctx = ToolContext::default();

	if env::args().count() < 2 {
		show_usage_si_streammodel();
		exit(1);
	}

	let mut args = env::args().skip(1);

	while let Some(arg) = args.next() {
		match &arg[..] {
			"-a" | "--all" => {
				ctx.process_all = true;
			},
			"-h" | "--help" => {
				show_usage_si_streammodel();
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
		show_usage_si_streammodel();
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

/* --------------------------------------------------- */

fn show_usage_pes_extractor()
{
	println!("{}", copyright::LTN_COPYRIGHT);
	println!("Version: v1.31.1"); // Drive this from the cargo.toml?
	println!("A tool to display PES structures from file.");
	println!("Usage:");
	println!("    -i, --input <filename>");
	println!("    -a don't terminate after the first model is obtained");
	println!("    -v Increase level of verbosity (enable descriptor dumping).");
	println!("    -h, --help Display command line help.");
	println!("    -P, --pid 0xnnnn PID containing the program elementary stream [def: 0x31]");
	println!("    -S, --streamid PES Stream Id. Eg. 0xe0 or 0xc0 [def: 0xe0]");
}

fn main_pes_extractor() {

	let mut ctx = ToolContext::default();

	if env::args().count() < 2 {
		show_usage_pes_extractor();
		exit(1);
	}

	let mut args = env::args().skip(1);

	while let Some(arg) = args.next() {
		match &arg[..] {
			"-a" | "--all" => {
				ctx.process_all = true;
			},
			"-h" | "--help" => {
				show_usage_pes_extractor();
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
			"-P" | "--pid" => {
				/* Convert 0xSOMETHING to internal var, or a pid in decimal */
				if let Some(arg_config) = args.next() {
					if arg_config.contains("0x") {
						let without_prefix = arg_config.trim_start_matches("0x");
						let n = u16::from_str_radix(without_prefix, 16);
						ctx.pid = n.unwrap();
					} else {
						ctx.pid = arg_config.parse::<u16>().unwrap();
					}
				} else {
					panic!("No value specified for parameter -P, --pid, aborting.");
				}
			},
			"-S" | "--streamid" => {
				/* Convert 0xSOMETHING to internal var, or a streamid in decimal */
				if let Some(arg_config) = args.next() {
					if arg_config.contains("0x") {
						let without_prefix = arg_config.trim_start_matches("0x");
						let n = u8::from_str_radix(without_prefix, 16);
						ctx.streamid = n.unwrap();
					} else {
						ctx.streamid = arg_config.parse::<u8>().unwrap();
					}
				} else {
					panic!("No value specified for parameter -S, --streamid, aborting.");
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
		show_usage_pes_extractor();
		eprintln!("\n-i, --input is mandatory, aborting");
		exit(1);
	}
	println!("args: {ctx:?}");
	println!();

	// Why does this have to be mutable?
	let mut pe = PesExtractor::new(false, ctx.pid, ctx.streamid);

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
        	println!("PES Extractor - Read {} / {} bytes", nbytes, processed);
		}

        let b: i32 = nbytes.try_into().unwrap();
		pe.write(&buffer[0], b / 188);
	}

}

/* --------------------------------------------------- */

fn show_usage_stream_statistics()
{
	println!("{}", copyright::LTN_COPYRIGHT);
	println!("Version: v1.31.1"); // Drive this from the cargo.toml?
	println!("A tool to display Transport Statistics from file.");
	println!("Usage:");
	println!("    -i, --input <filename>");
	println!("    -v Increase level of verbosity (enable descriptor dumping).");
	println!("    -h, --help Display command line help.");
}

fn main_stream_statistics() {

	let mut ctx = ToolContext::default();

	if env::args().count() < 2 {
		show_usage_stream_statistics();
		exit(1);
	}

	let mut args = env::args().skip(1);

	while let Some(arg) = args.next() {
		match &arg[..] {
			"-h" | "--help" => {
				show_usage_stream_statistics();
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
		show_usage_stream_statistics();
		eprintln!("\n-i, --input is mandatory, aborting");
		exit(1);
	}
	println!("args: {ctx:?}");
	println!();

	// Why does this have to be mutable?
	let mut ss = StreamStatistics::new(true);

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
        	println!("StreamStatistics - Read {} / {} bytes", nbytes, processed);
		}

        let b: u32 = nbytes.try_into().unwrap();
		ss.write(&buffer[0], b / 188);
	}

	ss.dprintf(1);

}

/* --------------------------------------------------- */

fn main() {

	let args: Vec<String> = env::args().collect();

	let path = Path::new(&args[0]);
	let tn = path.file_name().unwrap();
	let toolname = tn.to_str().unwrap();

	println!("arg[0]: {}", toolname);

	match toolname {
		"ts-analyze-binder" => main_stream_statistics(),
		"tsrust_si_streammodel" => main_si_streammodel(),
		"tsrust_pes_extractor" => main_pes_extractor(),
		"tsrust_stream_statistics" => main_stream_statistics(),
        _ => assert!(false),
	}
}
