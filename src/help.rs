use kagero::printer::{Colors, Printer};
use std::env::consts::{ARCH, OS};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const TARGET: &str = env!("TARGET");
const CARGO_VERSION: &str = env!("CARGO_VERSION");
const RUSTC_VERSION: &str = env!("RUSTC_VERSION");
static GIT_BRANCH_TAG: Option<&str> = option_env!("GIT_BRANCH_TAG");
static GIT_HASH: Option<&str> = option_env!("GIT_HASH");

/// Help Message
/// 
/// Displays a help message for using this tool.
pub(crate) fn help_message(p: &mut Printer) {
	// Usage
	p.print("USAGE: ", Colors::RedBright).print(NAME, Colors::Red).println(" <FILE>", Colors::Red)
	.writeln("")

	// Args
	.println("ARGS:", Colors::MagentaBright)
	.print("FILE: ", Colors::Magenta).writeln("Path to the file whose duplicates are to be deleted")
	.writeln("")

	// Flags
	.println("FLAGS:", Colors::CyanBright)
	.print("-V, --version: ", Colors::Cyan).writeln("Prints metadata of this tool")
	.print("-h, --help:", Colors::Cyan).writeln("Prints this help message");

}

/// Version Message
/// 
/// Prints metadata of the program.
pub(crate) fn version(p: &mut Printer) {
	// Name and Description
	p.print(NAME, Colors::RedBright)
	.print(" - ", Colors::RedBright)
	.println(DESCRIPTION, Colors::RedBright)

	// Version
	.print("Version: ", Colors::Magenta)
	.write("v").write(VERSION).write("@").write(OS).write("-").writeln(ARCH); // format!("v{VERSION}@{OS}-{ARCH}")

	// Repository and Git
	p.print("Repository: ", Colors::Magenta).writeln(REPOSITORY)
	.print("Build: ", Colors::Magenta);
	if GIT_BRANCH_TAG.is_some() && GIT_HASH.is_some() {
		p.write(GIT_HASH.unwrap())
		.write("@")
		.writeln(GIT_BRANCH_TAG.unwrap());
	} else {
		p.write(VERSION).writeln("@crates.io");
	}

	// Rust
	p.print("Target: ", Colors::Blue).writeln(TARGET)
	.print("Rust: ", Colors::Blue).writeln(RUSTC_VERSION)
	.print("Cargo: ", Colors::Blue).writeln(CARGO_VERSION);
}