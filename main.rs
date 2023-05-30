use std::io::{self, Read};

fn main() {
    let input_string = String::from("-[--->+<]>-.[---->+++++<]>-.+.++++++++++.+[---->+<]>+++.-[--->++<]>-.++++++++++.+[---->+<]>+++.[-->+++++++<]>.++.-------------.[--->+<]>---..+++++.-[---->+<]>++.+[->+++<]>.++++++++++++..---.[-->+<]>--------.");

    let char_array: Vec<char> = input_string.chars().collect();
    let bf_code: &[char] = &char_array;

    let mut memory: [u8; 30000] = [0; 30000];
    let mut mem_ptr = 0;
    let mut tok_ptr = 0;

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
            '.' => print!("{}", memory[mem_ptr] as char),
            ',' => {
                let mut input = [0u8; 1];
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
}
