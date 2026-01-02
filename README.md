# Byte Lang

## Byte Lang currently supports only ARM64 on macOS. Support for x86 and other architectures will be implemented later.

**Byte Lang** is an programming language designed to combine the low-level control of assembly language with the readability and structure of high-level languages. Currently in its early stages.

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
void : term(i32 exit_code : [reg(x0)], i32 test_num : [stack]) {
    asm(format("
        mov x16, #1
        svc #0x80
    "));
}

void : main() {
    i32 var = 4;
    i16 var2 = 2;
    i8 var3 = 1;
    i32 exit_code = 0;

    bl(term, exit_code, var);
}
```
