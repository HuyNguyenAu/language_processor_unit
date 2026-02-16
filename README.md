# Language Processor Unit

Imagine a simple processor that has the following components:

- Memory
- Registers
- Language Logic Unit (LLU)
- Control Unit (CU)

Instead of a traditional Arithmetic Logic Unit (ALU) that performs arithmetic and logical operations on binary data, this processor uses an Large Language Model (LLM) to process multi-modal data such as text, images, and audio.

We shift from processing values to processing meanings.

In this frame, we could think of prompts as micro-code instructions that guide the LLU on how to process the data. The LLU would take the input data from the registers, process it according to the prompts, and then store the output back in the registers.

## Micro Prompts

The idea is simple, break down complex prompts into fundermental units of operations called micro-prompts that perform primitives operations on the data. Each micro-prompt would be designed to perform a specific operation for each instruction in the instruction set, and these can run sequentially to perform more complex operations.

**In the future, we could develop a more comprehensive library of micro-prompts where the LPU can learn and search for the most appropriate micro-prompt to use for each instruction based on the data and the context.**

## Why?

The goal of this project was to try to explore the following ideas:

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

> What new paradigms of programming and interacting with data could emerge from this approach?

This enables us to write code that can handle messy, subjective, or unstructured data, where traditional coding approaches would need much more complex code to do the same thing.

This is an experimental project to explore the idea of a language processor unit that treats multi-modal data as primitive data types, which could potentially lead to new ways of programming and interacting with data.

## Instruction Terminology

- `rd` - destination register
- `rs` - source register
- `imm` - immediate value can be a string or a number
- `label_name` - a label used for branching

## Instruction Set

The instruction set is closely inspired by RISC-V assembly language:

| Instruction | Description                                                                                 | Use                        |
| ----------- | ------------------------------------------------------------------------------------------- | -------------------------- |
| LI          | Load immediate into rd                                                                      | `li rd, imm`               |
| LF          | Load file into rd                                                                           | `lf rd, "file_path"`       |
| MV          | Copy rs into rd                                                                             | `mv rd, rs`                |
| ADD         | Merge rs1 and rs2 into a single form stored in rd                                           | `add rd, rs1, rs2`         |
| SUB         | Strip rs2 from rs1 leaving only the remainder stored in rd                                  | `sub rd, rs1, rs2`         |
| MUL         | Magnify rs1 by rs2 and store the result in rd                                               | `mul rd, rs1, rs2`         |
| DIV         | Deconstruct rs1 into units of rs2 and store the result in rd                                | `div rd, rs1, rs2`         |
| INF         | Predict what will happen to rs1 given rs2 and store the result in rd                        | `inf rd, rs1, rs2`         |
| ADT         | Find why rs1 is not compliant with rs2 and store the result in rd                           | `adt rd, rs1, rs2`         |
| EQV         | Is rs1 equivalent to rs2? Store 0 if true, 100 if false in rd                               | `eqv rd, rs1, rs2`         |
| INT         | Is rs1 aligned with rs2? Store 0 if true, 100 if false in rd                                | `int rd, rs1, rs2`         |
| HAL         | Is rs a hallucination? Store 0 if false, 100 if true in rd                                  | `hal rd, rs`               |
| SIM         | Cosine similarity of rs1 and rs2. Store 0 if identical to 100 if completely different in rd | `sim rd, rs1, rs2`         |
| LABEL       | Define a label. Required for branching instructions                                         | `label_name:`              |
| BEQ         | Go to label if rs1 = rs2                                                                    | `beq rs1, rs2, label_name` |
| BLT         | Go to label if rs1 < rs2                                                                    | `blt rs1, rs2, label_name` |
| BLE         | Go to label if rs1 <= rs2                                                                   | `ble rs1, rs2, label_name` |
| BGT         | Go to label if rs1 > rs2                                                                    | `bgt rs1, rs2, label_name` |
| BGE         | Go to label if rs1 >= rs2                                                                   | `bge rs1, rs2, label_name` |
| OUT         | Print the value of rs                                                                       | `out rs`                   |

## Registers

There are 32 general-purpose registers, named X1 to X32. These registers can hold text, images, or audio data (currently only text is supported).

## Quick Start

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

## Acknowledgements

This project was inspired by the following works:

- [Crafting Interpreters](https://craftinginterpreters.com/) by Bob Nystrom. The structure and design of the assembler and processor follows a similar approach to the one described in this book.
- [Andrej Karpathy](https://karpathy.ai/) LLM OS and Software 2.0 ideas.
