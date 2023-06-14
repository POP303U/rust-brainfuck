use std::fs::File;
use std::io::{self, Read, Write};
use std::process::exit;

// | --------------- |
// |    MAIN LOOP    |
// | --------------- |

fn main() {
    loop {
        fancy_screen();
        stdout_print(String::from("choose mode: "));
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("couldn't read input");
        match input.trim() {
             "interpreter" => loop {
                 if interpreter() {
                     break;
                 }
             },
             "filereader" => loop {
                 if filereader() {
                     break;
                 };
             },
             "exit" => exit(0),
             _ => return,
        }
    }
}

fn fancy_screen() {
    print!("\x1B[2J\x1B[1;1H");
    println!("#---------------------------------------------------#");
    println!("|            rust-brainfuck [version 1.1]           |");
    println!("| choose mode to enter: 'interpreter', 'filereader' |");
    println!("| enter the command 'exit' to return to this menu   |");
    println!("#---------------------------------------------------#");
}

//  | ---------------- |
//  |    FILEREADER    |
//  | ---------------- |

fn filereader() -> bool {
    stdout_print(String::from("Enter file name: "));

    let mut input = String::from("");
    io::stdin()
        .read_line(&mut input).expect("couldn't read input");
    if input.trim() == "exit" {
        return true; 
    }

    let file_path = input.trim();
    let file_result = File::open(file_path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => {
            println!("Error opening file! {}", error);
            return false;
        }
    };

    file.read_to_string(&mut input).expect("Failed to read the file for some reason");
    println!("output: {}", parse_tokens(input));
    false
}

//  | ----------------- |
//  |    INTERPRETER    |
//  | ----------------- |

fn interpreter() -> bool {
    let mut input = String::from("");
    stdout_print(String::from(">>> "));
    io::stdin()
        .read_line(&mut input)
        .expect("couldn't read input");
    if input.trim() == "exit" {
        return true;
    }
    println!("output: {}", parse_tokens(input));
    false
}

//  | ---------------- |
//  |   TOKEN PARSER   |
//  | ---------------- |

fn parse_tokens(input_string: String) -> String {
    let bf_code: Vec<char> = input_string.chars().collect();

    let mut memory: [u8; 30000] = [0; 30000];
    let mut mem_ptr = 0;
    let mut tok_ptr = 0;
    let mut output = String::from("");

    while tok_ptr < bf_code.len() {
        match bf_code[tok_ptr] {
            '>' => {
                if mem_ptr == 29999 {
                    mem_ptr = 0;
                } else {
                    mem_ptr += 1;
                }
            }
            '<' => {
                if mem_ptr == 0 {
                    mem_ptr = 29999;
                } else {
                    mem_ptr -= 1;
                }
            }
            '+' => {
                if memory[mem_ptr] == 255 {
                    memory[mem_ptr] = 0;
                } else {
                    memory[mem_ptr] += 1;
                }
            }
            '-' => {
                if memory[mem_ptr] == 0 {
                    memory[mem_ptr] = 255;
                } else {
                    memory[mem_ptr] -= 1;
                }
            }
            '.' => {
                output += (memory[mem_ptr] as char).to_string().trim();
            }
            ',' => {
                let mut input = [0u8; 1];
                stdout_print(String::from("input: "));
                io::stdin()
                    .read_exact(&mut input)
                    .expect("Failed to read input");
                memory[mem_ptr] = input[0];
            }
            '[' => {
                if memory[mem_ptr] == 0 {
                    let mut layers = 0;
                    loop {
                        if bf_code[tok_ptr] == ']' {
                            if layers == 0 {
                                break;
                            }
                            layers -= 1
                        }
                        tok_ptr += 1;
                        if bf_code[tok_ptr] == '[' {
                            layers += 1
                        }
                    }
                }
            }
            ']' => {
                if memory[mem_ptr] != 0 {
                    let mut layers = 0;
                    loop {
                        if bf_code[tok_ptr] == '[' {
                            if layers == 0 {
                                break;
                            }
                            layers -= 1
                        }
                        tok_ptr -= 1;
                        if bf_code[tok_ptr] == ']' {
                            layers += 1
                        }
                    }
                }
            }
            _ => (),
        }
        tok_ptr += 1;
    }
    output
}

// Hacky workaround for io::stdin being called first
fn stdout_print(input: String) {
    print!("{}", input);
    io::stdout().flush().expect("Failed to flush buffer");
}

/* dont need this
fn help() {
    println!("Enter any of the 8 Brainfuck instructions to get interpreted");
    println!("+ - < > . , [ ]");
    println!("Memory is flushed after a command is interpreted");
}

println!("Type 'help' or 'version' for more information.");

fn version() {
    println!("rust-brainfuck [version 1.0]");
}

    if input == String::from("help\n") {
        help();
    } else if input == String::from("version\n") {
        version();
    } else {

    }
    */
