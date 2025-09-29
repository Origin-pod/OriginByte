<!-- # Ideation

6th September 2025

What is std and no-std ?

I'll explain what "std" and "no_std" mean in programming languages, particularly focusing on systems programming languages like Rust and C++.
Standard Library (std)
"std" refers to the standard library - a collection of pre-written code, functions, and data structures that come built-in with a programming language. It provides common functionality like:
Data structures: vectors, strings, hash maps, etc.
I/O operations: file reading/writing, console input/output
Memory management: allocation, deallocation
Mathematical functions: trigonometry, logarithms, etc.
String manipulation: parsing, formatting
Threading and concurrency: mutexes, threads, async operations
Examples:
C++: #include <iostream>, #include <vector>, #include <string>
Rust: use std::collections::HashMap, use std::fs::File
Python: import os, import json, import datetime

No Standard Library (no_std)
"no_std" means the program doesn't use the standard library. This is common in:

1. Embedded Systems
   Microcontrollers with limited memory
   Real-time systems where you need precise control
   IoT devices
2. Operating System Kernels
   Bootloaders
   Kernel development
   Bare metal programming
3. Performance-Critical Applications
   Where you need minimal overhead
   Custom memory management
   Specific hardware interactions

Why Use no_std?
Size: Smaller binary size
Control: Full control over memory and resources
Performance: No overhead from std abstractions
Compatibility: Works in environments where std isn't available
Real-time: Predictable execution time

Trade-offs
With std: More convenient, more features, larger size
With no_std: More work, more control, smaller size, more portable
The choice depends on your target environment and requirements!

What makes a programming language ?

- Languages features independent of OS - L1
- Standard libraries of the language - L2
- External libraries of the language - L3
- People start using language
  - They will find useful language code-templates for l1,l2 and l3 which are basically called Idiomatic patterns of the language
- Important to note L1-idiomatic is different from l3 idiomatic
- L-idiomatic can be idiomatic use of all l1, l2, l3 in a very specific way
- Idiomatic patterns are not thought of when the language features are released,they are discovered as an emergent property of the use the language itself, because a language is so complicated that all the permutations of combinations of using it cannot be predicted in advance.
- These 3 idiomatic collections are 20% of the language which can be enough to write

  - 99% code
  - with highest efficiency
  - with lowest bugs
  - with easiest readability

- Go language design in the service of software

- What all features of a language :-

  - Compile time errors
  - Minimum attempts
  - Long term maintenance
  - Frequent changes
  - High efficiency

- Our immediate task in to
  - Build a comprehensive idiomatic collections for L1, L2, and L3 based on internet research call it idiomatic-archive
  - Envolve idiomatic archive by discovering patterns in our own work and adding them to the collection
  - Further evolve the idiomatic-archive by finding new idiomatic patterns in the work and adding them to the collection, ir- making the llms go through hq codebases and find the idiomatic patterns.
  - Collect & evolve idiomatic architecture templates based on
    - Internet research
    - Our own work
    - The work of other code using LLMs
  - Use the idiomatic archive to write using llms
    - Break down problem into HLD and LLD
    - Use the TDD to impelemnt the architecture
    - Use the idiomatic archive to write meet the TDD criteria -->
