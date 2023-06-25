use std::env;
use std::process::ExitCode;
use kagero::printer::Printer;
mod help;

fn main() -> ExitCode {
	let mut prnt = Printer::default();
	let args: Vec<String> = env::args().collect();

	// Get file via command-line arguments
	let Some(file) = args.get(1) else {
		help::help_message(&mut prnt);
		return ExitCode::FAILURE;
	};

	// Flag handle
	return match file.as_str() {
		"-h" | "--help" => {
			help::help_message(&mut prnt);
			ExitCode::SUCCESS
		},
		"-V" | "--version" => {
			help::version(&mut prnt);
			ExitCode::SUCCESS
		},
		_ => ExitCode::SUCCESS // TODO: Replace this with function call that does the acutal logic.
	}
}
