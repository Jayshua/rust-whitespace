/**************************************************/
//
// Whitespace Interpreter
// ======================
// This program reads a whitespace program from
// the disk, parses it, and either prints out the
// parsed program, or executes it depending on
// the users choice.
//
/**************************************************/
use std::fs::File;
use std::io::Read;

mod parser;
mod symbols;
mod virtual_machine;
use virtual_machine::WhitespaceVM;


/*******************************/
// Macros
/*******************************/
// Return the value of a result, or print the error message
macro_rules! handle_err {
    ($expression:expr) => (match $expression {
    	Ok(val)  => val,
    	Err(err) => {println!("Error: {:?}", err.to_string()); return;},
    })
}


/*******************************/
// Functions
/*******************************/
// Main function
fn main() {
	// Get the command-line arguments and validate them
	let arguments: Vec<String> = std::env::args().collect();

	if arguments.len() > 3 || arguments.len() < 2 {
		print_usage();
		return;
	}

	if arguments.len() == 3 && arguments[1] != "list" {
		print_usage();
		return;
	}

	// Get the path to the whitespace file
	let file_path = if arguments.len() == 2 {
		arguments[1].clone()
	} else {
		arguments[2].clone()
	};

	// Open the file and read it into a string
	let mut file = handle_err!(File::open(file_path));
	let mut program = String::new();
	handle_err!(file.read_to_string(&mut program));

	// Parse the program
	let parsed = parser::parse(program);

	// List the program, or execute it
	if arguments.len() == 3 && arguments[1] == "list" {
		for action in parsed {
			println!("{:?}", action);
		}
	} else {
		// Reduce the routine labels to program pointers
		let reduced = parser::reduce_labels(parsed);

		// Create the vm and execute the program
		let mut vm = WhitespaceVM::new(reduced);
		vm.execute();
	}
}


// Print the program's usage instructions
fn print_usage() {
	println!("Usage:    whitespace [command] <file>");
	println!("Commands: run   - (default) run the program");
	println!("          list  - list the commands that the file contains");
	println!("\n");
}

