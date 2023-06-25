use std::env;
use std::path::Path;
use std::process::ExitCode;
use kagero::printer::{Printer, Colors};
mod hash;
mod help;

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
			// Convert to Path
			let filepath = Path::new(file);
			if !filepath.exists() || !filepath.is_file() {
				p.error(file, Colors::RedBright).errorln(" does not exist or is not a file!", Colors::Red);
				return ExitCode::FAILURE;
			}

			println!("{:#?}", hash::hasher(filepath)); // TODO: Do more stuff. This is just a test atm.

			ExitCode::SUCCESS
		}
	}
}