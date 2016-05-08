/***********************************************************/
//
// WhitespaceVM
// ============
// Defines a virtual machine that executes a whitespace
// program given as a vector of Actions
//
/***********************************************************/
use std::collections::HashMap;
use symbols::Action;
use symbols::Action::*;
use std::io;
use std::io::{Read, Write};


/********************************/
// Macros
/********************************/
// Pop a value off a vector, returning the value
// or panicking with an error if the stack is empty
macro_rules! pop {
    ($stack:expr) => (
    	match $stack.pop() {
    		Some(value) => value as i64,
    		None => panic!("Runtime Error: Tried to pop the stack, but it was empty."),
    	}
    )
}


/********************************/
// Public Structure
/********************************/
// A virtual machine that executes whitespace programs
#[derive(Debug)]
pub struct WhitespaceVM {
	heap:            HashMap<i64, i64>,
	stack:           Vec<i64>,
	call_stack:      Vec<usize>,
	program:         Vec<Action>,
	program_pointer: usize,
}

impl WhitespaceVM {
	// Constructor, create a WhitespaceVM
	pub fn new(program: Vec<Action>) -> WhitespaceVM {
		WhitespaceVM {
			heap:            HashMap::new(),
			stack:           Vec::new(),
			call_stack:      Vec::new(),
			program:         program,
			program_pointer: 0,
		}
	}

	// Execute the program
	pub fn execute(&mut self) {
		// Loop processing actions until a Halt is encountered
		loop {
			// Get the index of the highest element on the stack
			let stack_end = if self.stack.len() > 0 {self.stack.len() - 1} else {0};

			// Execute the current action
			match self.program[self.program_pointer] {
				/**************************/
				// Stack Operations
				/**************************/
				// Push the i64 value onto the stack
				StackPush(value) => self.stack.push(value),
	
				// Duplicate the top value of the stack
				StackDuplicate => {
					let value = self.stack[stack_end];
					self.stack.push(value);
				}
				
				// Swap the top two values on the stack
				StackSwap => {
					let temporary_value       = self.stack[stack_end];
					self.stack[stack_end]     = self.stack[stack_end - 1];
					self.stack[stack_end - 1] = temporary_value;
				}
				
				// Discard the top value of the stack
				StackDiscard => {
					pop!(self.stack);
				},


				/**************************/
				// Arithmetic Operations
				/**************************/
				// Add the top two values on the stack
				Add => {
					let right = pop!(self.stack);
					let left  = pop!(self.stack);
					let sum   = left + right;
					self.stack.push(sum);
				},

				// Subtract the top two values of the stack
				Subtract => {
					let right      = pop!(self.stack);
					let left       = pop!(self.stack);
					let difference = left - right;
					self.stack.push(difference);
				},

				// Multiply the top two values of the stack
				Multiply => {
					let right   = pop!(self.stack);
					let left    = pop!(self.stack);
					let product = left * right;
					self.stack.push(product);
				},

				// Divide the top two values of the stack
				Divide => {
					let right    = pop!(self.stack);
					let left     = pop!(self.stack);
					let quotient = left / right;
					self.stack.push(quotient);
				},

				// Get the remainder after dividing the top two values on the stack
				Modulo => {
					let right     = pop!(self.stack);
					let left      = pop!(self.stack);
					let remainder = left % right;
					self.stack.push(remainder);
				},

				
				/**************************/
				// Heap Operations
				/**************************/
				// Store the second value on the stack at the address indicated by the first value on the stack
				HeapStore => {
					let value   = pop!(self.stack);
					let address = pop!(self.stack);
					self.heap.insert(address, value);
				}

				// Retrieve the value at the address indicated by the top value on the stack
				HeapRetrieve => {
					let address = pop!(self.stack);
					let value = match self.heap.get(&address) {
						Some(value) => value,
						None => panic!("Tried to get a value from the heap, but no value was found at address: {}", address),
					};
					self.stack.push(*value);
				}


				/**************************/
				// Flow Control Operations
				/**************************/
				// Call the subroutine indicated by u64
				Call(location) => {
					self.call_stack.push(self.program_pointer);
					self.program_pointer = (location - 1u64) as usize; // Program Pointer will still be incremented this loop
				},

				// Unconditionally jump to the label u64
				Jump(location) => {
					self.program_pointer = (location - 1u64) as usize; // Program pointer will still be incremented this loop
				},

				// Jump to the label u64 if the top of the stack is zero
				JumpIfZero(location) => {
					if pop!(self.stack) == 0 {
						self.program_pointer = (location - 1u64) as usize;
					}
				}, 

				// Jump to the label u64 if the top of the stack is negative
				JumpIfNegative(location) => {
					if pop!(self.stack) < 0 {
						self.program_pointer = (location - 1u64) as usize;
					}
				},

				// End the current subroutine
				EndSubroutine => {
					self.program_pointer = self.call_stack.pop().expect("Tried to return from a procedure, but no procedure call was made.");
				},

				// Halt the execution of the program
				Halt => {
					break;
				},


				/**************************/
				// Flow Control Operations
				/**************************/
				// Output the top value of the stack as a character
				OutputChar => {
					let character = (pop!(self.stack) as u8) as char;
					print!("{}", character);
					io::stdout().flush().expect("Unable to flush standard output.");
				},

				// Output the top value of the stack as a number
				OutputNumber => {
					let number = pop!(self.stack);
					print!("{}", number);
					io::stdout().flush().expect("Runtime Error: Unable to flush standard output.");
				},

				// Read a character onto the stack
				ReadChar => {
					let destination = pop!(self.stack);
					let mut buffer = [0u8; 1];
					io::stdin().read_exact(&mut buffer).expect("Unable to read a character.");
					self.heap.insert(destination, buffer[0] as i64);
				},

				// Read a number onto the stack
				ReadNumber => {
					let destination = pop!(self.stack);
					let number;
					
					loop {
						let mut buffer = String::new();
						io::stdin().read_line(&mut buffer).expect("Unable to read from standard input.");

						match buffer.trim().parse() {
							Ok(val) => {
								number = val;
								break;
							},
							Err(error) => {
								println!("Unable to parse number: {}", error);
								continue;
							}
						}
					}

					self.heap.insert(destination, number);
				},


				/*****************************************************************/
				// These shouldn't happen since they are processed during parsing
				/*****************************************************************/
				Label(label) => panic!("Found a label instruction. This should have been parsed! Label was: {}", label), // Can't happen
				Error(error) => panic!("Found a parsing error while executing the program. This should have generated a Parse-Error! Error was: {}", error),
			}

			// Increment the program counter
			self.program_pointer += 1;
		}
	}
}

