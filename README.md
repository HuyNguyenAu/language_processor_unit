# Cognitive Processor Unit

Imagine a basic processor that has the following components:

- Memory
- Registers
- Semantic Logic Unit (SLU)
- Control Unit (CU)

Instead of a traditional Arithmetic Logic Unit (ALU) that performs arithmetic and logical operations on binary data, this processor has a Semantic Logic Unit (SLU) that performs operations based prompts and embeddings.

In this frame, we could think of prompts as micro-code instructions that guide the SLU on how to process the data, while embeddings represent the data in a high-dimensional space that can be used to compare with other data.

Previously code operates on data types like integers, floats, and strings. In contrast, this processor has the ability to operate multi-modal data types such as images, text, and audio.

## Why?

The goal of this project was to try to explore the following idea:

> What would it be like to code in a language that treats multi-model data as if it were primitive data types?

Imagine being able to write code like this:

```
        LI X1, "cat.jpg"                ; Load the input image of a cat into register X1.
        LI X2, "dog.jpg"                ; Load the target image of a dog into register X2.
        LI X3, 80                       ; Initialize register X3 with the similarity threshold value of 80.
        LI X4, "Not a dog."             ; Load the output message into register X4.

        SIM X5, X1, X2                  ; Compare X2 to X1 and store the similarity score in X5.

        BLT X3, X5, END                 ; If similarity score in X5 is less than 80, jump to END.
        LI X4, "Is a dog."              ; Change  the output message.

END:    OUT X4                          ; Output the result.
```

# Requirements

- [Rust](https://rust-lang.org/) minimum version 1.93.0
- [LLama.cpp](https://github.com/ggml-org/llama.cpp) server with minimum release tag b7843

# Instruction Terminology

- `rd` - destination register
- `rs` - source register
- `value` - can be an immediate value (e.g., a string or number)

# Instruction Set

Available instructions in the assembly language:

| Instruction | Description                     | Use                        |
| ----------- | ------------------------------- | -------------------------- |
| LI          | Load Immediate                  | `li rd, imm`               |
| LF          | Load File                       | `lf rd, "file_path"`       |
| MV          | Copy Register                   | `mv rd, rs`                |
| ADD         | Add                             | `add rd, rs1, rs2`         |
| SUB         | Subtract                        | `sub rd, rs1, rs2`         |
| MUL         | Multiply                        | `mul rd, rs1, rs2`         |
| DIV         | Divide                          | `div rd, rs1, rs2`         |
| SIM         | Similarity                      | `sim rd, rs1, rs2`         |
| LABEL       | Label                           | `label_name:`              |
| BEQ         | Branch if Equal                 | `beq rs1, rs2, label_name` |
| BLT         | Branch if Less Than             | `blt rs1, rs2, label_name` |
| BLE         | Branch if Less Than or Equal    | `ble rs1, rs2, label_name` |
| BGT         | Branch if Greater Than          | `bgt rs1, rs2, label_name` |
| BGE         | Branch if Greater Than or Equal | `bge rs1, rs2, label_name` |
| OUT         | Output                          | `out rs`                   |

# Registers

The processor has 8 general-purpose registers, named X0 to X7. These registers can hold text, images, or audio data.

# Quick Start

Clone the repository:
```bash
git clone https://github.com/HuyNguyenAu/cognitive_processor_unit.git
cd cognitive_processor_unit
```

Start the LLama.cpp server:
```bash
./llama-server --embeddings --pooling mean -m path/to/your/model.bin
```

Build the example program:
```bash
cargo run build examples/simple.aism
```

Run the example program:

```bash
cargo run run data/build/simple.caism
```

# Acknowledgements

This project was inspired by the following works:

- [Crafting Interpreters](https://craftinginterpreters.com/) by Bob Nystrom
- [Andrej Karpathy](https://karpathy.ai/) LLM OS and Software 2.0 ideas
