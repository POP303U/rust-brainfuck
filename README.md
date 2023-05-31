# rust-brainfuck version 1.1
rust-brainfuck is a fast brainfuck interpreter written in rust
it works like terminal Python3, however memory is flushed after entering a command.

**Usage**
+ **Choose Mode**: Choose between the Interpreter and Filereader.
+ **Interpreter**: The console takes input and interprets it. (takes input if necessary)
+ **Filereader**: The application reads files from its current directory. For Example: bf.txt

You can exit these modes by using the 'exit' subcommand.

**Bugs**
+ 1: Rust crashes if no file to read exists.
+ 2: Input still looks very wonky but works
