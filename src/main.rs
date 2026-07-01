use std::env;
use std::fs;
use std::io::{self, Write};

const MAX_MEMORY_BYTES: usize = 30_000;

const INC    : char = '+';
const DEC    : char = '-';
const PTR_INC: char = '>';
const PTR_DEC: char = '<';
const OP_LOOP: char = '[';
const CL_LOOP: char = ']';
const OUT    : char = '.';
const IN     : char = ',';

struct Parser {
    pos: usize,
    intput: String,
}

impl Parser {
    fn new(input: String) -> Parser { 
        Parser {pos: 0, intput: input} 
    }

    fn get_char(&self) -> char {
        self.intput[self.pos..].chars().next()
            .expect("File format not recogonized")
    }

    fn next_pos(&mut self) { self.pos += 1; }
    fn eof(&self) -> bool { self.pos >= self.intput.len() }
}

struct Program {
    inst_pos: usize,
    instructions: Vec<char>,
    memory_ptr: usize,
    memory: Vec<u8>,
    loops_idx: Vec<usize>
}

impl Program {
    fn new() -> Program {
        Program { 
            inst_pos: 0, 
            instructions: Vec::new(),
            memory_ptr: 0,
            memory: Vec::with_capacity(0),
            loops_idx: Vec::with_capacity(0),
        }
    }

    fn eof(&self) -> bool { self.inst_pos >= self.instructions.len() }

    fn update_memory(&mut self) {
        if self.memory_ptr + 1 < self.memory.len() { return; }
        for _ in 0..( (self.memory_ptr + 1) - self.memory.len() ) {
            self.memory.push(0);
        }
    }

    fn get_current_memory(&self) -> u8 {
        self.memory[self.memory_ptr]
    }

    fn get_current_inst(&self) -> char {
        self.instructions[self.inst_pos]
    }

    fn get_last_loop_idx(&self) -> usize {
        self.loops_idx[self.loops_idx.len()-1]
    }

    fn increase(&mut self) {
        if self.get_current_memory() < 255 {
            self.memory[self.memory_ptr] += 1;
        }
    }

    fn decrease(&mut self) {
        if self.get_current_memory() > 0 {
            self.memory[self.memory_ptr] -= 1;
        }
    }

    fn ptr_increase(&mut self) {
        self.memory_ptr += 1;
        if self.memory_ptr > MAX_MEMORY_BYTES { self.memory_ptr = 0; }
    }

    fn ptr_decrease(&mut self) {
        if self.memory_ptr == 0 { self.memory_ptr = MAX_MEMORY_BYTES; }
        else { self.memory_ptr -= 1; }
    }

    fn open_loops(&mut self) {
        if self.get_current_memory() != 0 && 
            (self.loops_idx.len() == 0 || self.get_last_loop_idx() != self.inst_pos) {
            self.loops_idx.push(self.inst_pos);
            return;
        }

        while !self.eof() && self.get_current_inst() == ']' {
            self.inst_pos += 1;
        }
        self.inst_pos += 1;
    }

    fn close_loops(&mut self) {
        assert!(self.loops_idx.len() != 0);
        if self.get_current_memory() == 0 { self.loops_idx.pop(); }
        else { self.inst_pos = self.get_last_loop_idx(); }
    }

    fn out(&self) {
        custom_print(&format!(
            "{}", self.get_current_memory() as char));
    }

    fn input(&self) {

    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1);
    let path = &args[1];

    let contents = 
        fs::read_to_string(path)
        .expect("No such file directory");

    let mut parser = Parser::new(contents);
    let mut prog = Program::new();

    while !parser.eof() {
        let c = parser.get_char();
        if check_syntax(c) {
            prog.instructions.push(c);
        }
        parser.next_pos();
    }

    while !prog.eof() {
        prog.update_memory();
        let operation = prog.instructions[prog.inst_pos];
        
        if operation == INC     { prog.increase();     }
        if operation == DEC     { prog.decrease();     }
        if operation == PTR_INC { prog.ptr_increase(); }
        if operation == PTR_DEC { prog.ptr_decrease(); }
        if operation == OP_LOOP { prog.open_loops();   }
        if operation == CL_LOOP { prog.close_loops();  }
        if operation == OUT     { prog.out();          }
        if operation == IN      { prog.input();        }

        prog.inst_pos += 1;
    }
}

fn check_syntax(x: char) -> bool {
    /* 
    The syntax is simple, so we can do this method to be faster, 
    since this is faster than in a loop (don't ask me why tho).
    */

    return 
        x == INC     || x == DEC     || 
        x == PTR_INC || x == PTR_DEC || 
        x == OP_LOOP || x == CL_LOOP ||
        x == IN      || x == OUT
} 

fn custom_print(s: &str) {
    print!("{s}");
    io::stdout().flush().unwrap();
}