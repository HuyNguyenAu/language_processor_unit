# Language Processor Unit

Let's start out with an idea _because I'm bored_: **What if a processor had an ALU (Arithmetic Logic Unit) that was an LLM?**

Basically, we shifted from having a processor that is exact and deterministic to one that is probabilistic and generative. This new paradigm called "Soft Computing" allows us to work with data that is unstructured, messy, or subjective in a way that traditional computing struggles with. In short, we can handle ambiguity and fuzzy logic, which is becoming more common in real‑world applications.

This project explores the idea of implementing a simple proccessor that has memory (RAM), a control unit, registers, and an ALU that is powered by a language model. The instruction set is designed to allow us to write code that can interact with the language model in a structured way, while still allowing for the flexibility and creativity of natural language prompts.

Think of this assembly language as a middle ground between traditional programming languages and natural language prompts, where we can write code that is more structured and modular than natural language prompts, but still allows us to work with multi-modal data in a way that is more intuitive and flexible than traditional programming languages.

## Why?

I really wanted to imagin a future where we can write code where we don't have to worry about edge cases or complex logic to handle unstructured data. Instead, we can just write code that describes what we want to achieve, and let the language model handle the complexity of how to achieve it. In short, **we can write code that is more focused on the "what" rather than the "how"**.

Here's an example of what a program written in this assembly language might looks like:

```
; Program: Room Comfort Adjustment System
; Objective: Adjust the room's temperature and lighting based on sensor data to achieve optimal physical comfort.
; Output: Adjusted temperature and lighting settings.

; Load sensor data and user feedback.
LF  X1, "examples/data/room_sensor_data.json"
LS  X2, "It's too dark to read and I am sweating."

; Sense: Brief description of the current state of the room based on sensor data.
LS  X3, "A sentence that describes the current state of the room."
MRF X4, X1, X3

RETRY:
; Think: Analyze the sensed state and determine necessary adjustments for physical comfort.
LS  X3, "New temperature and light adjustments values for physical comfort."
DST X5, X4, X3

; Guardrails: Ensure that the adjustments are within safe and reasonable limits.
LS  X3, "{ \"temp_c\": number, \"light%\": number }"
MRF X6, X5, X3

LS  X3, "The temperature must be between 18 and 24 degrees Celsius."
AUD X7, X6, X3

LI X3, 0
BEQ X7, X3, RETRY

LS X3, "The light intensity must be between 0% and 100%."
AUD X8, X6, X3

LI X3, 0
BEQ X8, X3, RETRY

; Act: Implement the adjustments to achieve the desired physical comfort.
OUT X6
```

## Instruction Terminology

- `rd` - destination register
- `rs` - source register
- `imm` - immediate value can be a string or a number
- `label_name` - a label used for branching

## Instruction Set

The instruction set is closely inspired by RISC-V assembly language:

| Instruction | Description                                                                  | Use                        |
| ----------- | ---------------------------------------------------------------------------- | -------------------------- |
| LS          | Load string into rd                                                          | `ls rd, "example"`         |
| LI          | Load immediate into rd                                                       | `li rd, imm`               |
| LF          | Load file into rd                                                            | `lf rd, "file_path"`       |
| MV          | Copy rs into rd                                                              | `mv rd, rs`                |
| BEQ         | Go to label if rs1 = rs2                                                     | `beq rs1, rs2, label_name` |
| BLT         | Go to label if rs1 < rs2                                                     | `blt rs1, rs2, label_name` |
| BLE         | Go to label if rs1 <= rs2                                                    | `ble rs1, rs2, label_name` |
| BGT         | Go to label if rs1 > rs2                                                     | `bgt rs1, rs2, label_name` |
| BGE         | Go to label if rs1 >= rs2                                                    | `bge rs1, rs2, label_name` |
| MRF         | Change the shape of rs1 into the form of rs2 and store in rd                 | `mrf rd, rs1, rs2`         |
| PRJ         | Predict the next step given rs1 when rs2 occurs and store in rd              | `prj rd, rs1, rs2`         |
| DST         | Boiling rs1 down to the essence of rs2 and store in rd                       | `dst rd, rs1, rs2`         |
| COR         | Find the link, difference, or similarity between rs1 and rs2 and store in rd | `cor rd, rs1, rs2`         |
| AUD         | Check if rs1 complies with rs2 and store 100 if compliant, 0 otherwise in rd | `aud rd, rs1, rs2`         |
| SIM         | Cosine similarity between rs1 and rs2 and store in rd (0 - 100)              | `sim rd, rs1, rs2`         |
| LABEL       | Define a label. Required for branching instructions                          | `label_name:`              |
| OUT         | Print the value of rs                                                        | `out rs\|imm`              |
| EXIT        | Exit the program                                                             | `exit`                     |

## Registers

There are 32 general-purpose registers, named X1 to X32. These registers can hold text and positive numbers (currently working on support images and audio).

## Quick Start

Clone the repository:

```bash
git clone https://github.com/HuyNguyenAu/cognitive_processor_unit.git
cd cognitive_processor_unit
```

Install [llama.cpp](https://github.com/ggml-org/llama.cpp).

Download [LFM2 2.6B model](https://huggingface.co/LiquidAI/LFM2-2.6B-GGUF).

Start the LLama.cpp server:

```bash
./llama-server --embeddings --pooling mean -m C:\llama\models\LFM2-2.6B-Q5_K_M.gguf
```

Build the example program:

```bash
cargo run build examples/room-comfort.aasm
```

Run the example program:

```bash
cargo run run data/build/room-comfort.lpu
```

## Acknowledgements

This project was inspired by the following works:

- [Crafting Interpreters](https://craftinginterpreters.com/) by Bob Nystrom. The structure and design of the assembler and processor follows a similar approach to the one described in this book.
- [Andrej Karpathy](https://karpathy.ai/) LLM OS and Software 2.0 ideas.
