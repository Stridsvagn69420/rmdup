use std::env;
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



            // ---- TODO: Do more stuff. This is just a test atm. ----
            // List files
            println!("{:#?}", files);

            let input = files.0.join(filepath.file_name().unwrap()); // Create path as if file was in cwd (I test with build.rs as the input file)
            let current = files.1.get(1).unwrap().to_owned(); // Get a file's absolute path (build.rs is my 2nd file in the listing)
            let ihash = hash::hasher(&input).unwrap();
            let chash = hash::hasher(&current).unwrap();
            println!("{:#?}", input);
            println!("{:#?}", current);
            println!("{:#?}", ihash);
            println!("{:#?}", chash);
            println!("{:#?}", current == input); // Compare the files' path.
            println!("{:#?}", ihash == chash); // Also compare their hash value along with their path, like it has to do with every other file.
            // This is used so that if the current file matches the input file, it won't be removed. The system only works like this for the current functionality!

            // Hash Test
            println!("{:#?}", hash::hasher(filepath));

			ExitCode::SUCCESS
		}
	}
}