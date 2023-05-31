use std::io::{self, Read, Write};
use std::fs::File;

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    println!("#---------------------------------------------------#");
    println!("|            rust-brainfuck [version 1.1]           |");
    println!("| choose mode to enter: 'interpreter', 'filereader' |");
    println!("| enter the command 'exit' to return to this menu   |");
    println!("#---------------------------------------------------#");
    stdout_print(String::from("choose mode: "));
    choose_mode();
    
}

// | --------------- |
// |    MAIN LOOP    |
// | --------------- |

fn choose_mode() {
    let mut input = String::from("");
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("couldn't read input");
        if input == String::from("interpreter\n") {
            loop {
                interpreter();
            }
        } else if input == String::from("filereader\n") {
            loop {
                filereader();
            }
    }   else {
            println!("Invalid option");
        }    
    }
    
}

//  | ---------------- |
//  |    FILEREADER    |
//  | ---------------- | 

fn filereader() {
    stdout_print(String::from("Enter file name: "));
    let mut input = String::from("");
    io::stdin().read_line(&mut input).expect("couldn't read input");
    if input == String::from("exit\n") {
        main();
    }
    let file_path = input.trim_end_matches('\n');
    let mut file = File::open(file_path);

    file.expect("File not found").read_to_string(&mut input);

    println!("output: {}",parse_tokens(input));
}

//  | ----------------- |
//  |    INTERPRETER    |
//  | ----------------- | 

fn interpreter() {
    let mut input = String::from("");
    stdout_print(String::from(">>> "));
    io::stdin().read_line(&mut input).expect("couldn't read input");
    if input == String::from("exit\n") {
        main();
    }
    println!("output: {}",parse_tokens(input));
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
                if mem_ptr == 30000 {
                    mem_ptr = 0;
                } else {
                    mem_ptr += 1;
                } 
            }
            '<' => {
                if mem_ptr == 0 {
                    mem_ptr = 30000;
                } else {
                    mem_ptr -= 1;
                }
            }
            '+' => {
                if memory[mem_ptr] == 255 {
                    memory[mem_ptr] = 0;
                } else {
                    memory[mem_ptr] = memory[mem_ptr] + 1;
                }
            }
            '-' => {
                if memory[mem_ptr] == 0 {
                    memory[mem_ptr] = 255;
                } else {
                    memory[mem_ptr] = memory[mem_ptr] - 1;
                }
            }
            '.' => {
                output += &(memory[mem_ptr] as char).to_string().trim_end_matches('\n');
            }
            ',' => {
                let mut input = [0u8; 1];
                stdout_print(String::from("input: "));
                io::stdin().read_exact(&mut input).expect("Failed to read input");
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
    return output;
}

// Just a better way to print, normal way sucks
fn stdout_print(input: String) {
    print!("{}",input);
    io::stdout().flush();
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
