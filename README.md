# Byte Lang

## Byte Lang currently supports only ARM64 on macOS. Support for x86 and other architectures will be implemented later.

**Byte Lang** is an programming language designed to combine the low-level control of assembly language with the readability and structure of high-level languages. Currently in its early stages.

## Contributing

Contributions are very welcome. This project is still early and needs help

- Feel free to open a GitHub issue for **anything**: bug reports, feature requests, questions, or even just ideas.
- No question is too simple to ask. Don't hesitate to request explanations of files, functions, or literally anything else.
- Every contribution, no matter how small, is greatly appreciated.
- If you'd like to work on something, comment on an issue and tag me to get it assigned. Feel free to ask questions.

## Project Goals

- This programming language is designed to behave **exactly as written**. Zero optimizations.
- At the same time, it aims to be highly performant through full manual control (with more effort required).
- Many compile time functions that allow the user to write complex and performant assembly wrappers.

It's particularly suited for:
- Operating systems and kernels
- Low-level libraries
- Performance-critical code

## Features

- **Early Development**: Byte Lang is still evolving with a limited set of commands and features.
- **Low-Level Control**: Directly manipulate hardware and system resources with assembly-like syntax.
- **Readable Syntax**: A structured and human-friendly approach to low-level programming.

## Getting Started

### Installation

Since Byte Lang is in its early stages, the installation process involves cloning the repository and building from source:

1. **Clone**:
   ```bash
   git clone https://github.com/morcules/byte-lang
   ```

2. **Navigate to the Directory**:
   ```bash
   cd byte-lang
   ```

3. **Run your first program**:
   ```bash
   ./byte-lang (command)
   ```

### Commands
* run (file location like example.byte)
* build (file location like example.byte)

### Example
```bash
void : term(i64 exit_code : [reg(x0)]) {
    asm(format("mov x1, #{}\nmov x16, #1\nsvc #0x80\n", 10));
}

void : test(i32 test_var : [stack], i64 test_var_two : [stack]) {

}

void : main() {
    i32 var = 5;
    i16 var2 = 2;
    i8 var3 = 1;
    i64 exit_code = 30;
    i64 exit_code_clone = exit_code;
    u64 test = 10;
    u8 something = 30;
    u8 something_clone = something;
    i64 exit_code_success = 0;

    bl(test, var, exit_code_clone);
    
    bl(term, exit_code_success);
}
```
