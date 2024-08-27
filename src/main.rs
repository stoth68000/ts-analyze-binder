use std::env;
use std::process::exit;
use std::io::Read;
use std::path::Path;
use std::net::{UdpSocket, Ipv4Addr};
use url::Url;

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

fn process_file(ctx: &mut ToolContext, sm: &mut StreamModel)
{
	println!("process_file {:?}", ctx.input);

	let mut file_in = std::fs::File::open(ctx.input.as_str()).unwrap();
	let mut buffer = [0u8; 16 * 188];
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
				return;
			}
		}
	}
}

fn process_udp_socket(ctx: &mut ToolContext, sm: &mut StreamModel) -> Result<(), Box<dyn std::error::Error>>
{
	println!("process_udp_socket1 {:?}", ctx.input);

	// Parse the URL
	let url = match Url::parse(&ctx.input) {
		Ok(url) => url,
		Err(e) => {
			eprintln!("Error parsing URL: {:?}", e);
			return Err(Box::new(e));
		}
	};

    if url.scheme() != "udp" {
        eprintln!("Invalid URL scheme. Expected 'udp'.");
        return Err("Bad URL")?;
    }

    let host = match url.host_str() {
        Some(host) => host,
        None => {
            eprintln!("No host found in URL.");
            return Err("Bad host")?;
        }
    };

    let port = match url.port() {
        Some(port) => port,
        None => {
            eprintln!("No port found in URL.");
            return Err("Bad port")?;
        }
    };

    // Create a UDP socket bound to the port
    // Bind the socket to the multicast port
	let without_prefix = format!("{}:{}", host, port);
    let socket = UdpSocket::bind(without_prefix)?;

	// Join the multicast group
	let multicast_addr: Ipv4Addr = host.parse().expect("Invalid multicast address");
	socket.join_multicast_v4(&multicast_addr, &Ipv4Addr::new(0, 0, 0, 0))?;
    socket.set_multicast_loop_v4(true)?;
  
	let mut buffer = [0u8; 7 * 188];
	let mut processed = 0;
	
    loop {
		let (nbytes, src) = socket.recv_from(&mut buffer)?;
        processed += nbytes;

		if ctx.verbose > 0 {
        	println!("StreamModel - Read {} / {} bytes, src {:?}", nbytes, processed, src);
		}

        let b: i32 = nbytes.try_into().unwrap();

		let mut complete = false;
		sm.write(&buffer[0], b / 188, &mut complete);

		if complete == true {
			let pat = sm.query_model();
			pat.print();

			if ctx.verbose > 0 {
				println!("PAT.program_count = {}", pat.program_count);
			}

			if ctx.process_all == false {
				return Ok(());
			}
		}
	}
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
					if arg_config.to_lowercase().contains("udp://") {
						ctx.input_type = InputType::InputUDPSocket;
						ctx.input = arg_config;
					} else {
						ctx.input = arg_config;
						ctx.input_type = InputType::InputFile;
					}
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

	// Why does this have to be mutable?
	let mut sm = StreamModel::new(false);

	match ctx.input_type {
		InputType::InputUnknown => (),
		InputType::InputUDPSocket => {
			let _ = process_udp_socket(&mut ctx, &mut sm);
		},
		InputType::InputFile => {
			process_file(&mut ctx, &mut sm);
		},
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
	let mut ss = StreamStatistics::new(false);
	println!("StreamStatistics: {:?}", ss);

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
		"ts-analyze-binder" => main_si_streammodel(),
		"tsrust_si_streammodel" => main_si_streammodel(),
		"tsrust_pes_extractor" => main_pes_extractor(),
		"tsrust_stream_statistics" => main_stream_statistics(),
        _ => assert!(false),
	}
}
