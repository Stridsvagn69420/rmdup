use std::env;
use std::fs;
use std::io::ErrorKind::{NotFound, PermissionDenied};
use std::path::Path;
use std::process::ExitCode;
use kagero::printer::{Printer, Colors};
mod hash;
mod help;
mod dir;

fn main() -> ExitCode {
	let mut p = Printer::default();
	let args: Vec<String> = env::args().collect();

	// Get file via command-line argument
	let Some(file) = args.get(1) else {
		help::help_message(&mut p);
		return ExitCode::FAILURE;
	};

	// Flag handler
	match file.as_str() {
		"-h" | "--help" => {
			help::help_message(&mut p);
			ExitCode::SUCCESS
		},
		"-V" | "--version" => {
			help::version(&mut p);
			ExitCode::SUCCESS
		},
		_ => {
			// Check File Path
			let filepath = Path::new(file);
			if !filepath.is_file() {
				p.error(file, Colors::RedBright).errorln(" does not exist or is not a file!", Colors::Red);
				return ExitCode::FAILURE;
			}

			// List Files in CWD
			let files = match dir::list_cwd() {
				Ok(res) => res,
				Err(ref e) if vec![NotFound, PermissionDenied].contains(&e.kind()) => {
					p.errorln("Could not access directory! Either the current directory does not exist anymore or is unreadable.", Colors::RedBright);
					return ExitCode::FAILURE;
				},
				Err(e) => {
					p.error("An unexpected error occured: ", Colors::Red)
					.errorln(e.kind().to_string().as_str(), Colors::RedBright);
					return ExitCode::FAILURE;
				}
			};

			// Create pseudo-path of input file in CWD and get hash of input file
			let pseudo_input = files.0.join(filepath.file_name().unwrap());
			let exists_in_cwd = pseudo_input.is_file();
			let Ok(ihash) = hash::hasher(filepath) else {
				p.error("Could not calculate BLAKE3 hash of ", Colors::Red)
				.errorln(file, Colors::RedBright)
				.errorln("The file is probably unreadable.", Colors::Red);
				return ExitCode::FAILURE;
			};

			// Iterate over CWD files
			p.print("Scanning ", Colors::Red)
			.print(files.1.len().to_string().as_str(), Colors::RedBright)
			.println(" files for possible duplicates...", Colors::Red);

			for dupfile in files.1 {
				p.println("----------------------------------------------", Colors::CyanBright);
				let dupfile_name = String::from(dupfile.file_name().unwrap().to_string_lossy());
				p.print("File: ", Colors::Blue)
				.println(&dupfile_name, Colors::BlueBright);

				// Get hash of file
				let Ok(duphash) = hash::hasher(&dupfile) else {
					p.errorln("Could not get BLAKE3 hash of file!", Colors::Red);
					continue;
				};

				// Compare BLAKE3 hashes
				if ihash == duphash {
					if exists_in_cwd && pseudo_input == dupfile {
						p.println("File is input file. Skipping...", Colors::Blue);
					} else {
						// Remove duplicate file
						match fs::remove_file(dupfile) {
							Ok(()) => p.println("Successfully removed duplicate!", Colors::MagentaBright),
							Err(_) => p.println("Could not remove duplicate file!", Colors::RedBright)
						};
					}
				} else {
					p.println("Hash does not match. Skipping...", Colors::Cyan);
				}
			}

			p.writeln("")
			.println("All duplicate files have been removed!", Colors::Red);
			ExitCode::SUCCESS
		}
	}
}