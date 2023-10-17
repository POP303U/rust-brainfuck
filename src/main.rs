use std::{
    fs::File,
    io::{self, Read, Write},
};

macro_rules! fprint {
    ($($arg:tt)*) => {{
        print!($($arg)*);
        io::stdout().flush().expect("ERROR: Failed to flush stdout");
    }};
}

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
    loop {
        fancy_screen();
        fprint!("Choose mode: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("couldn't read input");

        match input.trim() {
            "interpreter" => loop {
                mode_interpreter()
            },
            "filereader" => loop {
                mode_filereader().expect("Can't find file");
            },
            "help" => loop {
                mode_help();
            },
            _ => return,
        }
    }
}

fn mode_filereader() -> Result<(), io::Error> {
    fprint!("Enter file path: ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("ERROR: Couldn't read stdin");

    if user_input.trim() == "exit" {
        main();
    }

    let file_path = user_input.trim();
    let mut file = File::open(file_path)?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content)?;

    let mut bf = Brainfuck::new(file_content);
    bf.run();

    println!("");
    Ok(())
}

fn mode_interpreter() {
    let mut input = String::new();
    fprint!("\n>>> ");
    io::stdin()
        .read_line(&mut input)
        .expect("couldn't read var:input");

    let mut bf = Brainfuck::new(input.clone().into_bytes());
    bf.run();

    if input.trim() == "exit" {
        main();
    }
}

fn mode_help() {
    print!("\x1B[2J\x1B[1;1H");
    println!("\nEnter any of the 8 Brainfuck instructions to get interpreted");
    println!("+ - < > . , [ ]");
    println!("Memory is flushed after a command is interpreted\n");

    let mut user_input = String::new();
    fprint!("Press Enter to exit: ");
    io::stdin()
        .read_line(&mut user_input)
        .expect("couldn't read var:user_input");
    match user_input {
        _ => main(), // the most hacky code ever written
    }
}

fn fancy_screen() {
    print!("\x1B[2J\x1B[1;1H");
    println!("#-----------------------------------------------------------#");
    println!("|               rust-brainfuck [version 2.0]                |");
    println!("| choose mode to enter: 'interpreter', 'filereader', 'help' |");
    println!("|      enter the command 'exit' to return to this menu      |");
    println!("#-----------------------------------------------------------#");
}
