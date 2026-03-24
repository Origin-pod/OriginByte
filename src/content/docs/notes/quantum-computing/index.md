---
title: 'Quantum Computing — Index'
---

# Quantum Computing — Index

## Summary

This folder is a structured set of notes to go from **zero → competent** in quantum computing, with a practical bias: what quantum computers are, what they’re good for, why they’re hard to build, and how the field evolved. It includes a company deep-dive on **Alice & Bob** (cat qubits, biased noise, and their error-correction approach).

## How to use this

- If you want a fast “map”: skim `02-evolution-timeline.md` + `01-what-is-quantum-computing.md`.
- If you want real understanding: do the learning path below in order.
- If you care about industry: read `06-hardware-modalities.md`, `07-error-correction-ftqc.md`, then `08-alice-and-bob.md`.

## Learning path (recommended order)

1. `01-what-is-quantum-computing.md`
2. `02-evolution-timeline.md`
3. `03-quantum-mechanics-primer.md`
4. `04-qubits-and-measurement.md`
5. `05-gates-circuits-and-models.md`
6. `06-hardware-modalities.md`
7. `07-error-correction-ftqc.md`
8. `08-alice-and-bob.md`
9. `09-cryptography-qkd-and-pqc.md`
10. `10-glossary.md`
11. `11-resources.md`

## What “quantum advantage” means (quick)

- **Speedup is problem-dependent**: quantum computers aren’t “faster computers”; they are different machines with different cost models.
- **Practical advantage** often requires **fault tolerance** (error-corrected logical qubits), not just more physical qubits.
- Most current devices are **NISQ** (Noisy Intermediate-Scale Quantum): useful for research, limited for reliable long computations.

## Open questions to keep in mind

- What applications still need FTQC vs which might work in NISQ/hybrid regimes?
- Which hardware approach scales fastest while hitting fault-tolerance thresholds?
- How big is the “overhead gap” between a lab demo and a useful logical-qubit machine?

## References

- Wikipedia timeline (orientation only): `https://en.wikipedia.org/wiki/Timeline_of_quantum_computing_and_communication`
