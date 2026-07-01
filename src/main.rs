use std::env;
use std::fs;

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
    pos: usize,
    instructions: Vec<char>,
}

impl Program {
    fn new() -> Program {
        Program { 
            pos: 0, 
            instructions: Vec::new() 
        }
    }

    fn eof(&self) -> bool { self.pos >= self.instructions.len() }
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
        prog.pos += 1;
    }
}

fn check_syntax(x: char) -> bool {
    /* 
    The syntax is simple, so we can do this method to be faster, 
    since this is faster than in a loop (don't ask me why tho).
    */

    return 
        x == '+' || x == '-' || 
        x == '>' || x == '<' || 
        x == '[' || x == ']' ||
        x == ',' || x == '.'
} 