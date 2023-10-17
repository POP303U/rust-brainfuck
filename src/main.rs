use std::{
    env,
    fs::File,
    io::{self, Read},
    path::Path,
    process::exit,
};

struct Brainfuck {
    code: Vec<u8>,
    memory: Vec<u8>,
    pointer: i32,
}

impl Brainfuck {
    fn new(code: Vec<u8>) -> Brainfuck {
        Brainfuck {
            code,
            pointer: 0,
            memory: vec![0],
        }
    }

    fn plus(&mut self) {
        if self.memory[self.pointer as usize] == 255 {
            self.memory[self.pointer as usize] = 0;
            return;
        }
        self.memory[self.pointer as usize] += 1;
    }

    fn minus(&mut self) {
        if self.memory[self.pointer as usize] == 0 {
            self.memory[self.pointer as usize] = 255;
            return;
        }
        self.memory[self.pointer as usize] -= 1;
    }

    fn input(&mut self) {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("ERROR: Failed to read input");
        self.memory[self.pointer as usize] = input.as_bytes()[0];
    }

    fn output(&mut self) {
        print!("{}", self.memory[self.pointer as usize] as u8 as char);
    }

    fn move_right(&mut self) {
        self.pointer += 1;
        if self.memory.len() < self.pointer as usize + 1 {
            self.memory.push(0);
        }
    }

    fn move_left(&mut self) {
        if self.pointer != 0 {
            self.pointer -= 1;
        }
    }

    fn sanitize_code(&mut self) {
        self.code.retain(|&c| {
            c == b'+'
                || c == b'-'
                || c == b'.'
                || c == b'!'
                || c == b','
                || c == b'['
                || c == b']'
                || c == b'<'
                || c == b'>'
        });
    }

    fn run(&mut self) {
        self.sanitize_code();
        let mut bracket_vec: Vec<i32> = Vec::new();
        let mut i: usize = 0;
        let opcodes = self.code.clone();
        while i < opcodes.len() {
            match opcodes[i] {
                b'+' => self.plus(),
                b'-' => self.minus(),
                b'>' => self.move_right(),
                b'<' => self.move_left(),
                b'.' => self.output(),
                b',' => self.input(),
                b'[' => {
                    if self.memory[self.pointer as usize] != 0 {
                        bracket_vec.push(i as i32);
                    } else {
                        bracket_vec.pop();
                    }
                }
                b']' => {
                    if self.memory[self.pointer as usize] != 0 {
                        match bracket_vec.last() {
                            Some(&index) => {
                                i = index as usize;
                            }
                            None => println!(""),
                        };
                    } else {
                        bracket_vec.pop();
                    }
                }
                _ => panic!("ERROR: Invalid token ran in function: Brainfuck::run()"),
            }
            i += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("ERROR: No arguments provided\nCorrect usage: cargo run -- <input.bf>");
        exit(1);
    }
    let mut file = File::open(Path::new(&args[1])).expect("ERROR: File not found");
    let mut file_content = Vec::new();

    match file.read_to_end(&mut file_content) {
        Err(why) => eprintln!("ERROR: Couldn't read file, why: {why}"),
        Ok(_) => print!("{}", String::from_utf8_lossy(&file_content)),
    };

    let mut bf = Brainfuck::new(file_content);
    bf.run();
}
