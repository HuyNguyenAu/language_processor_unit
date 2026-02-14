# Cognitive Processor Unit

Imagine a simple processor that has the following components:

- Memory
- Registers
- Semantic Logic Unit (SLU)
- Control Unit (CU)

Instead of a traditional Arithmetic Logic Unit (ALU) that performs arithmetic and logical operations on binary data, this processor uses an Large Language Model (LLM) as its SLU to process multi-modal data such as text, images, and audio.

We shift from processing values to processing meanings.

In this frame, we could think of prompts as micro-code instructions that guide the SLU on how to process the data. The SLU would take the input data from the registers, process it according to the instructions, and then store the output back in the registers or memory.

The instruction set is closely inspired by RISC-V assembly language, but with a focus on operations that are relevant to multi-modal data processing. For example, instead of traditional arithmetic operations, we have a `SIM` instruction that computes the similarity between two pieces of data. This greatly simplifies the code of the assembler and processor, which allows us to focus on the fun parts!

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

        BGE X5, X3, END                 ; If similarity score in X5 is greater than or equal to 80, jump to END.
        LI X4, "Is a dog."              ; Change the output message.

END:    OUT X4                          ; Output the result.
```

This is an experimental project to explore the idea of a cognitive processor unit that treats multi-modal data as primitive data types, which could potentially lead to new ways of programming and interacting with data.

# Requirements

- [Rust](https://rust-lang.org/) minimum version 1.93.0
- [LLama.cpp](https://github.com/ggml-org/llama.cpp) server with minimum release tag b7843

# Instruction Terminology

- `rd` - destination register
- `rs` - source register
- `value` - can be an immediate value (e.g., a string or number)

# Semantic Instruction Set

Available instructions in the assembly language:

| Instruction | Description                     | Use                        |
| ----------- | ------------------------------- | -------------------------- |
| LI          | Load Immediate                  | `li rd, imm`               |
| LF          | Load File                       | `lf rd, "file_path"`       |
| MV          | Copy Register                   | `mv rd, rs`                |
| ADD         | Add                             | `add rd, rs1, rs2`         |
| SUB         | Subtract                        | `sub rd, rs1, rs2`         |
| INF         | Inference                       | `inf rd, rs1, rs2`         |
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

The processor has 32 general-purpose registers, named X1 to X32. These registers can hold text, images, or audio data.

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

- [Crafting Interpreters](https://craftinginterpreters.com/) by Bob Nystrom. The structure and design of the assembler and processor follows a similar approach to the one described in this book.
- [Andrej Karpathy](https://karpathy.ai/) LLM OS and Software 2.0 ideas.
