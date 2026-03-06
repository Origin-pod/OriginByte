---
title: 'Alice & Bob: Cat Qubits and Elevator Codes'
date: '2026-03-06'
tags: ['quantum-computing', 'alice-and-bob', 'cat-qubits', 'error-correction']
---

# Alice & Bob: Cat Qubits and Elevator Codes

## Summary

Alice & Bob is a quantum computing company (Paris/Boston, founded 2020) focused on building a **fault-tolerant universal quantum computer** using **cat qubits**—bosonic qubits engineered to have strongly biased noise. Their recent work on **Elevator Codes** aims to dramatically reduce logical error rates with relatively low overhead by stacking (concatenating) codes tuned to that bias.

## Notes

### What are cat qubits?

- Implemented in **superconducting microwave cavities** (bosonic modes) coupled to nonlinear elements.
- Logical states are superpositions of coherent states (“Schrödinger cat” states) in phase space.
- Key property: noise can be made **highly biased**:
  - **Bit flips** (logical X errors) are **exponentially suppressed** as you tune parameters.
  - **Phase flips** (logical Z errors) increase more gently.

Why this matters:

- Traditional qubit architectures often have roughly symmetric X/Z error rates.
- If one error is much rarer, you can design codes that:
  - focus correction on the dominant error type
  - achieve lower logical error with fewer physical resources.

Alice & Bob has reported record **bit-flip lifetimes** (minutes+ region) for their cat qubits.

### Bosonic codes vs qubit codes

- **Bosonic codes** encode logical qubits into oscillator states instead of collections of two-level systems.
- Examples: cat codes, GKP codes, binomial codes.
- Advantages:
  - Can map natural noise (e.g., photon loss) into correctable patterns.
  - Potentially reduce the number of physical components per logical qubit.

Cat qubits are a particular bosonic code where the structure of the code matches the device physics.

### Elevator Codes (high-level)

Problem: even with biased noise and cat codes, you still need additional coding to reach **very low logical error rates** (e.g. ~\(10^{-12}\) per logical gate for chemistry).

Elevator Codes idea (simplified):

- Use **code concatenation**: a bosonic code (cat qubit) at the bottom, then additional repetition-like codes on top.
- Introduce a **logical ancilla qubit** that is reused and moved through different code blocks—like an elevator:
  - It interacts with different encoded blocks to extract syndrome information about logical bit flips.
  - Because bit flips are already rare, the overhead of this scheme is manageable.

Reported benefits (per Alice & Bob’s announcements, early 2026):

- Up to **10,000× reduction** in logical error rates.
- Overhead of only about **3× more qubits** compared to their previous architecture.

### Where this sits in the landscape

- Many players pursue **surface-code-based** FTQC, which is general but has large overhead.
- Alice & Bob’s approach:
  - Leverages **hardware-level bias** (cat qubits) and **hardware-friendly codes**.
  - Aims to reduce total qubit count per useful logical qubit by **orders of magnitude**.
- If successful, this could:
  - Lower the hardware scale needed for early useful FTQC.
  - Influence how others think about co-design of hardware + codes.

### What to watch for

- Demonstrations of:
  - Stable logical qubits with error decreasing as the code scales (true fault tolerance).
  - Benchmarking against surface-code-style logical qubits on similar hardware budgets.
  - Real algorithms or chemistry simulations using logical cat qubits + Elevator Codes.

## References

- Company site and technical blog: `https://alice-bob.com/`
- Elevator Codes overview: `https://alice-bob.com/blog/elevator-codes-pushing-logical-bit-flips-to-near-extinction/`
