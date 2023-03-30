# Brainfuck compiler
An actual brainfuck compiler written in rust

### Installation:

``` shell
cargo install --git "https://github.com/Pranjal-Patel/bfc-rs"
```

### Usage:

``` shell
$ bfc
An actual brainfuck compiler

Usage: bfc [OPTIONS] <COMMAND>

Commands:
  compile, -c, --compile      Compile brainfuck code, default output file name: `output` and `output.asm` (will be deleted)
  interpret, -i, --interpret  Interpret brainfuck code
  help                        Print this message or the help of the given subcommand(s)

Options:
      --cellcount <cell count>  [default: 2000]
  -h, --help                    Print help
  -V, --version                 Print version
```

<hr>

Here's a brainfuck code to print hello world with a new line
<br>
File: hello.bf

``` brainfuck
>+++++++[<++++++++++>-]<++.
>>++++++++++[<++++++++++>-]<+.
>>++++++++++[<++++++++++>-]<++++++++.
>>++++++++++[<++++++++++>-]<++++++++.
>>+++++++++++[<++++++++++>-]<+.
>>+++[<++++++++++>-]<++.
>>++++++++[<++++++++++>-]<+++++++.
>>+++++++++++[<++++++++++>-]<+.
>>+++++++++++[<++++++++++>-]<++++.
>>++++++++++[<++++++++++>-]<++++++++.
>>++++++++++[<++++++++++>-]<.>

>++++++++++.
```

#### Compiler:

``` shell
$ bfc -cp hello.bf -o hello
$ ./hello
Hello World

# If you want to see the assembly output
$ bfc -cp hello.bf -s
$ cat output.asm
# Assembly output
...
```

#### Interpreter:

``` shell
$ bfc -ip hello.bf
Hello World
```