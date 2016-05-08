/***********************************************************/
//
// Symbols
// =======
// Contains symbols defining the Whitespace language.
//
// enum Token <- The three tokens of the language (Return, Space, Tab)
//
// enum Action <- All of the operations in the language
//
/***********************************************************/

// The three whitespace tokens
#[derive(Debug, Copy, Clone)]
pub enum Token {
	Return,
	Space,
	Tab,
}

// Every built-in method
#[derive(Debug, Copy, Clone)]
pub enum Action {
	/** Stack Manipulation **/
	StackPush(i64), // Push the i64 value onto the stack
	StackDuplicate, // Duplicate the top value of the stack
	StackSwap,      // Swap the top two values on the stack
	StackDiscard,   // Discard the top value of the stack

	/** Arithmetic **/
	Add,       // Add the top two values on the stack
	Subtract,  // Subtract the top two values of the stack
	Multiply,  // Multiply the top two values of the stack
	Divide,    // Divide the top two values of the stack
	Modulo,    // Get the remainder after dividing the top two values on the stack

	/** Heap **/
	HeapStore,    // Store the second value on the stack at the address indicated by the first value on the stack
	HeapRetrieve, // Retrieve the value at the address indicated by the top value on the stack

	/** Flow Control **/
	Label(u64),          // Create the label u64 at the current location
	Call(u64),           // Call the subroutine indicated by u64
	Jump(u64),           // Unconditionally jump to the label u64
	JumpIfZero(u64),     // Jump to the label u64 if the top of the stack is zero
	JumpIfNegative(u64), // Jump to the label u64 if the top of the stack is negative
	EndSubroutine,       // End the current subroutine
	Halt,                // Halt the execution of the program

	/** I/O **/
	OutputChar,   // Output the top value of the stack as a character
	OutputNumber, // Output the top value of the stack as a number
	ReadChar,     // Read a character onto the stack
	ReadNumber,   // Read a number onto the stack

	Error(&'static str), // Unrecognized token
}