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
START:  MOV R1, "cat.jpg"   ; Load the input image of a cat into register R1.
        MOV R2, "dog.jpg"   ; Load the target image of a dog into register R2.
        SIM R1, R2, R3      ; Compare R2 to R1 and store the similarity score in R3.

        JLT R3, 80, START   ; If similarity score in R3 is less than 80, jump back to START.

        OUT "Is a dog."     ; Output the result.
```

# Requirements

- [Rust](https://rust-lang.org/) minimum version 1.93.0
- [LLama.cpp](https://github.com/ggml-org/llama.cpp) server with minimum release tag b7843

# Acknowledgements

This project was inspired by the following works:

- [Crafting Interpreters](https://craftinginterpreters.com/) by Bob Nystrom
- [Andrej Karpathy](https://karpathy.ai/) LLM OS and Software 2.0 ideas
