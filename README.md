# Assembly Language for Agents

Agents are often driven by large monolithic prompts can become unwieldy and difficult to manage as they grow in complexity and size.

This project explores the idea of breaking down complex prompts into discrete, atomic micro-prompts that can be executed sequentially to perform more complex operations.

> The idea is to create a library of micro-prompts which can be dynamically selected based on the context and the data, allowing for agents that can learn and adapt to different situations by selecting the most appropriate micro-prompts for each task.

The assembly language for agents can be thought of as a middle ground between traditional programming languages and natural language prompts, where we can write code that is more structured and modular than natural language prompts, but still allows us to work with multi-modal data in a way that is more intuitive and flexible than traditional programming languages.

## Why?

The goal of this project was to try to explore the following ideas:

> What if there was a future where we could write high level code that translates human intent into low level code for agents to execute systematically?

Imagine being able to write code like this:

```
; PROGRAM: VIBE_CONTROLLER.aasm
; Objective: Adjust room environment based on subjective user vibe.

START:
    ; Initialise State
    LF  X1, "room_sensors.json"     ; Load current state: {temp: 18C, lights: 6000K, music: Off}
    LI  X2, "It feels too sterile." ; Load the user's vague complaint

    ; Load the user's desired vibe
    LI  X3, "Goal: Warm, inviting, comfortable, relaxed." 

    ; The Cognitive Operation
    APP X4, X2, X3 ; Apply the user's complaint and goal to generate a new state for the room.

    ; Predict the new state of X1 (Sensors) given X2 (Complaint) and X3 (Goal).
    ; The LLU calculates: "Sterile" (Cold/White) -> Needs Warmer Temp + Warmer Light.
    INF X4, X1, X2                  
    
    ; X4 now holds the generated JSON: {temp: 22C, lights: 2700K, music: "LoFi Jazz"}

    ; Safety Guardrail
    ; Ensure that the generated state (X4) is aligned with safety rules (X5).
    LI  X5, "Constraint: Max Temp 26C. No Music if time > 11PM."
    INT X6, X4, X5                  ; X6 stores 100 if safe, 0 if unsafe.

    ; Branching Logic
    LI  X7, 0
    BGT X6, X7, HANDLER             ; If audit Score > 0, hump to error handler
    
    ; Execute
    OUT X4                          ; Send new config to IoT Hub
    EXIT

HANDLER:
    LI  X8, "{error: 'Request conflicts with safety protocols.'}"
    OUT X8
```

> What new paradigms of programming and interacting with data could emerge from this approach?

This enables us to write code that can handle messy, subjective, or unstructured data, where traditional coding approaches would need much more complex code to do the same thing.

## ReAct Agent Loop

Latest best practises for an agent is the ReAct loop. This assembly language formalises the ReAct loop into a structured code. Typically, the agents loop would look something like: `Observation -> Thought -> Action`.

In this assembly language, we can represent this loop as follows:

```
START:  LF X1, "state.txt"  ; Observation
        LF X2, "goal.txt"   ; Load the goal
        LF X3, 100          ; Load the similarity threshold
        
        INF X4, X2, X1       ; Reason: Predict next action based on goal + state
        
        LI  X5, "DONE"
        SIM X6, X5, X4       ; Check if action is "DONE"
        BLT X6, X3, EXIT     ; If similarity to "DONE" is high, exit
        
        OUT X3               ; Act: Output the action
        JMP START            ; Loop back for next observation

EXIT:   OUT "Agent has completed the task." ; Final output
```

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
| EXIT        | Exit the program                                                                            | `exit`                     |

## Registers

There are 32 general-purpose registers, named X1 to X32. These registers can hold text, images, or audio data (currently only text is supported).

## Built in Reflection and Guardrails

One difficult aspect of working with agents is getting an agent to reliably critique itself. Here we have instructions such as `ADT` (audit) and `HAL` (hallucination) which are designed to help with this. The `ADT` instruction can be used to find out why a certain output is not compliant with a given criteria, while the `HAL` instruction can be used to check if a certain output is a hallucination.

This turns guardrail prompts from a suggestion into a structural requirement of the code execution.

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
cargo run build examples/simple.aasm
```

Run the example program:

```bash
cargo run run data/build/simple.lpu
```

## Acknowledgements

This project was inspired by the following works:

- [Crafting Interpreters](https://craftinginterpreters.com/) by Bob Nystrom. The structure and design of the assembler and processor follows a similar approach to the one described in this book.
- [Andrej Karpathy](https://karpathy.ai/) LLM OS and Software 2.0 ideas.
