---
title: 'Evolution of Quantum Computing (Timeline + Eras)'
---

# Evolution of Quantum Computing (Timeline + Eras)

## Summary

Quantum computing evolved in waves: foundational theory, algorithmic breakthroughs, early experiments, then modern engineering to scale **physical qubits** and (now) **logical qubits** via error correction.

## Notes

## Mental model: eras

- **Theory era**: define what a quantum computer is; show it can do something different.
- **Algorithm era**: discover speedups (Shor/Grover), clarify what “advantage” could mean.
- **Prototype era**: build small systems, demonstrate gates, coherence, early algorithms.
- **NISQ era**: scale to tens–thousands of qubits; explore near-term apps; hit noise limits.
- **Fault-tolerance era (current focus)**: build error-corrected logical qubits; make error decrease with scale.

## Milestones (high-level)

### Foundations

- **1980**: Benioff describes a quantum mechanical model of computation.
- **1981–1982**: Feynman argues quantum systems are hard to simulate classically → motivates quantum simulators.
- **1985**: Deutsch formalizes the universal quantum computer concept (circuit model foundations).

### Algorithmic breakthroughs

- **1994**: Shor’s factoring algorithm (and discrete log) → cryptography implications.
- **1996**: Grover’s search algorithm → quadratic speedup for unstructured search.

### Early experimental demonstrations

- **Late 1990s–2000s**: NMR, ion traps, superconducting qubits, photons show small-scale control.
- **2001**: early experimental Shor demonstration on small numbers (proof-of-principle).

### Modern scaling and public access

- **2010**: D-Wave markets quantum annealing systems (distinct from universal gate model).
- **2016**: IBM makes gate-based quantum processors accessible via cloud (broad adoption by researchers).
- **2019**: Google’s “quantum supremacy” sampling experiment (controversial framing, but meaningful engineering milestone).

### Toward fault tolerance

- **2020s**: focus shifts to error correction, logical qubits, and demonstrating that **more qubits can mean lower error** (the core requirement for scalable FTQC).

## Why the timeline matters

Most “useful” quantum algorithms assume:

- a **universal** gate-based machine
- **low enough logical error rates** for deep circuits

So the key transition is not “more physical qubits,” but **demonstrating fault-tolerant scaling**.

## References

- Wikipedia timeline (good for orientation; verify details in primary sources): `https://en.wikipedia.org/wiki/Timeline_of_quantum_computing_and_communication`
