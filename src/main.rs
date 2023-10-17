use std::{
    env,
    fs::File,
    io::{self, Read},
    path::Path,
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
            memory: vec![0; 30000],
        }
    }

    fn plus(&mut self) {
        if self.memory[self.pointer as usize] == 255 {
            self.memory[self.pointer as usize] = 0;
        } else {
            self.memory[self.pointer as usize] += 1;
        }
    }

    fn minus(&mut self) {
        if self.memory[self.pointer as usize] == 0 {
            self.memory[self.pointer as usize] = 255;
        } else {
            self.memory[self.pointer as usize] -= 1;
        }
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
        if self.pointer == 29999 {
            self.pointer = 0;
        }
        self.pointer += 1;
    }

    fn move_left(&mut self) {
        if self.pointer == 0 {
            self.pointer = 29999;
        } else {
            self.pointer -= 1;
        }
    }

    fn sanitize_code(&mut self) {
        self.code.retain(|&c| {
            c == b','
                || c == b'+'
                || c == b'-'
                || c == b'.'
                || c == b'['
                || c == b']'
                || c == b'<'
                || c == b'>'
        });
    }

    fn run(&mut self) {
        self.sanitize_code();
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
                    if self.memory[self.pointer as usize] == 0 {
                        let mut layers = 0;
                        loop {
                            if opcodes[i] == b']' {
                                if layers == 0 {
                                    break;
                                }
                                layers -= 1
                            }
                            i += 1;
                            if opcodes[i] == b'[' {
                                layers += 1
                            }
                        }
                    }
                }
                b']' => {
                    if self.memory[self.pointer as usize] != 0 {
                        let mut layers = 0;
                        loop {
                            if opcodes[i] == b'[' {
                                if layers == 0 {
                                    break;
                                }
                                layers -= 1
                            }
                            i -= 1;
                            if opcodes[i] == b']' {
                                layers += 1
                            }
                        }
                    }
                }
                _ => eprintln!("ERROR: tried to execute invalid token this should NEVER happen"),
            }
            i += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("ERROR: No arguments provided\nCorrect usage: cargo run -- <input.bf>");
    }
    let mut file = File::open(Path::new(&args[1])).expect("ERROR: File not found");
    let mut file_content = Vec::new();

    match file.read_to_end(&mut file_content) {
        Err(why) => eprintln!("ERROR: Couldn't read file, why: {why}"),
        Ok(_) => println!("SUCCESS: Successfully read file, executing..."),
    };

    let mut bf = Brainfuck::new(file_content);
    bf.run();
}
