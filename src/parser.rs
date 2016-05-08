/***********************************************************/
//
// Whitespace Parser
// =================
// This file contains routines for parsing a
// whitespace program.
//
// parse(program: String) -> Vec<Action>
// -- Parses a whitespace program, returning a list
//    of Actions
//
// reduce_labels(program: Vec<Action>) -> Vec<Action>
// -- Takes a program of Actions and rewrites the labels
//    from names to simple action index pointers
//
/***********************************************************/
use std::collections::HashMap;
use symbols::Action;
use symbols::Action::*;
use symbols::Token;
use symbols::Token::*;



/********************************************/
// Public Functions
/********************************************/
// Parse a whitespace program, returning a list of actions
pub fn parse(program: String) -> Vec<Action> {
   let     reversed_program = program.chars().rev().collect();
   let mut tokenizer        = Tokenizer::new(reversed_program);
   let     parsed_program   = _parse(&mut tokenizer);

   parsed_program
}


// Replace the labels in a whitespace program with simple index pointers
pub fn reduce_labels(program: Vec<Action>) -> Vec<Action> {
   let mut reduced_program = Vec::new();
   let mut labels = HashMap::new();

   // Get all the labels in the program
   let mut program_pointer = 0;
   for action in program.iter() {
      if let &Label(label) = action {
         labels.insert(label, program_pointer);
      } else {
         program_pointer += 1;
      }
   }

   // Convert label names in flow control actions to program pointers
   for action in program {
      match action {
         // Drop label statements
         Label(_) => {},

         // Rewrite named labels
         Call          (ref label)  => reduced_program.push(Call          (*labels.get(label).unwrap())),
         Jump          (ref label)  => reduced_program.push(Jump          (*labels.get(label).unwrap())),
         JumpIfZero    (ref label)  => reduced_program.push(JumpIfZero    (*labels.get(label).unwrap())),
         JumpIfNegative(ref label)  => reduced_program.push(JumpIfNegative(*labels.get(label).unwrap())),

         // Ignore other actions
         other_action => reduced_program.push(other_action),
      }
   }

   reduced_program
}



/*****************************************/
// Structures
/*****************************************/
// Tokenizes the whitespace program, removing all non-whitespace
struct Tokenizer {
   program: String
}

impl Tokenizer {
   // Constructor, create a tokenizer
   fn new(program: String) -> Tokenizer {
      Tokenizer {
         program: program,
      }
   }

   // Get the next token
   fn next(&mut self, matching: &'static str) -> Token {
      'search:loop {
         match self.program.pop().unwrap_or_else(|| panic!("Program ended while trying to match: {}", matching)) {
            ' '  => return Space,
            '\n' => return Return,
            '\t' => return Tab,
            _ => continue 'search,
         }
      }
   }

   // Check if there are more tokens to get
   fn more(&self) -> bool {
      for character in self.program.chars() {
         if character == ' ' || character == '\t' || character == '\n' {
            return true
         }
      }

      return false
   }
}



/*****************************************/
// Private Functions
/*****************************************/
// Parse a token stream into a list of Actions
fn _parse(tokens: &mut Tokenizer) -> Vec<Action> {
   let mut actions = Vec::new();

   while tokens.more() {
      actions.push(parse_token(tokens));
   }

   actions
}


// Parse a single whitespace token, returning it as an action
fn parse_token(tokens: &mut Tokenizer) -> Action {
   match tokens.next("Stack Manipulation, Flow Control, or {Arithmetic, Heap, I/O}") {
      /*** Stack Manipulation ***/
      Space  => match tokens.next("Stack Manipulation") {
         Tab    => Error("Unexpected Tab"),
         Space  => StackPush(consume_number(tokens)),
         Return => match tokens.next("Stack Manipulation: StackDuplicate, StackSwap, StackDiscard") {
            Space  => StackDuplicate,
            Tab    => StackSwap,
            Return => StackDiscard,
         },
      },

      /*** Flow Control ***/
      Return => match tokens.next("Flow Control") {
         Space  => match tokens.next("Flow Control: Call, Label, Jump") {
            Tab    => Call (consume_label(tokens)),
            Space  => Label(consume_label(tokens)),
            Return => Jump (consume_label(tokens)),
         },

         Tab    => match tokens.next("Flow Control: JumpIfZero, JumpIfNegative, EndSubroutine") {
            Space  => JumpIfZero    (consume_label(tokens)),
            Tab    => JumpIfNegative(consume_label(tokens)),
            Return => EndSubroutine,
         },

         Return => match tokens.next("Flow Control: Halt") {
            Space  => Error("Unexpected Space"),
            Tab    => Error("Unexpected Tab"),
            Return => Halt,
         }
      },

      /*** Arithmetic, Heap, I/O ***/
      Tab    => match tokens.next("Arithmetic, Heap, I/O") {
         /*** Arithmetic ***/
         Space => match tokens.next("Arithmetic") {
            Return => Error("Unexpected Return"),
            Space  => match tokens.next("Arithmetic: Add, Subtract, Multiply") {
               Space  => Add,
               Tab    => Subtract,
               Return => Multiply,
            },
            Tab    => match tokens.next("Arithmetic: Divide, Modulo") {
               Return => Error("Unexpected Return"),
               Space  => Divide,
               Tab    => Modulo,
            }
         },

         /*** Heap Manipulation ***/
         Tab   => match tokens.next("Heap Manipulation: HeapStore, HeapRetrieve") {
            Return => Error("Unexpected Return"),
            Space  => HeapStore,
            Tab    => HeapRetrieve,
         },

         /*** I/O ***/
         Return => match tokens.next("I/O") {
            Return => Error("Unexpected Return"),
            Space  => match tokens.next("I/O: OutputChar, OutputNumber") {
               Return => Error("Unexpected Return"),
               Space  => OutputChar,
               Tab    => OutputNumber,
            },
            Tab    => match tokens.next("I/O: ReadChar, ReadNumber") {
               Return => Error("Unexpected Return"),
               Space  => ReadChar,
               Tab    => ReadNumber,
            }
         }
      }
   }
}


// Match a return terminated number
fn consume_number(program: &mut Tokenizer) -> i64 {
   let mut number: u64 = 0;

   let negative = match program.next("Number: Positive/Negative") {
      Token::Space  => false,
      Token::Tab    => true,
      Token::Return => panic!("Unexpected Return in Number definition. Expected space or tab representing sign."),
   };

   loop {
      match program.next("Number: 0/1 (Space/Tab)") {
         Token::Space  => {number <<= 1;},
         Token::Tab    => {number <<= 1; number |= 0b1;},
         Token::Return => {break;},
      }
   }

   if negative {
      -(number as i64)
   } else {
      number as i64
   }
}


// Match a return terminated label
fn consume_label(program: &mut Tokenizer) -> u64 {
   let mut label: u64 = 1;

   loop {
      match program.next("Label") {
         Token::Space  => {label <<= 1; label |= 0b1;},
         Token::Tab    => {label <<= 1; label |= 0b0;},
         Token::Return => break,
      }
   }

   label
}