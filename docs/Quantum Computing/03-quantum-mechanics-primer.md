---
title: 'Quantum Mechanics Primer (for Computing)'
date: '2026-03-06'
tags: ['quantum-computing', 'quantum-mechanics', 'basics']
---

# Quantum Mechanics Primer (for Computing)

## Summary

Quantum computing is mostly **linear algebra + probability**, with a few quantum-specific rules. This note is the “minimum viable QM” you need for circuits, algorithms, and error correction.

## Notes

### States are vectors (kets)

- A pure state is a normalized vector often written as a “ket” like `|ψ⟩` in a complex Hilbert space.
- For one qubit, a common basis is `|0⟩, |1⟩`.
- For `n` qubits, the space dimension is `2^n`.

### Superposition is linear combination

You can write a state as

`|ψ⟩ = Σ_x α_x |x⟩` with `Σ_x |α_x|^2 = 1`.

The `α_x` are **amplitudes**; probabilities come from **squared magnitudes**.

### Global phase doesn’t matter (relative phase does)

- `|ψ⟩` and `e^{iθ}|ψ⟩` represent the same physical state.
- But phases *between components* (relative phase) affect interference.

### Evolution (closed systems) is unitary

- Quantum gates are represented by **unitary** matrices `U` where `U† U = I`.
- Applying a gate updates the state: `|ψ'⟩ = U|ψ⟩`.

### Measurement = probabilities + state update

Measuring in the computational basis `|0⟩, |1⟩`:

- Outcome `0` occurs with probability `|⟨0|ψ⟩|^2`
- Outcome `1` occurs with probability `|⟨1|ψ⟩|^2`

After measurement, the post-measurement state is the normalized projection onto the observed outcome’s subspace.

### Composite systems: tensor products

- Two qubits: `|ψ⟩ ⊗ |φ⟩` (often written `|ψ⟩|φ⟩`).
- Basis states: `|00⟩, |01⟩, |10⟩, |11⟩`.

### Entanglement

A state is **entangled** if it cannot be written as a simple product state.

Example Bell state:

`|Φ⁺⟩ = (|00⟩ + |11⟩) / √2`

Measuring one qubit gives a correlated result in the other; importantly, this correlation can’t be reproduced by classical shared randomness *for all measurement choices*.

### Interference

Because amplitudes add, paths can cancel.

- If two computational paths lead to the same basis state with opposite phase, their amplitudes can cancel.
- Most quantum algorithms are “interference engines”: design \(U\) so the amplitude mass concentrates on good outcomes.

### Mixed states and density matrices (why you care)

Real devices interact with the environment → noise → you can’t assume a pure state.

- A mixed state is represented by a **density matrix** `ρ`.
- Pure state: `ρ = |ψ⟩⟨ψ|`
- Noise and measurement can be expressed as **quantum channels** (CPTP maps).

You’ll revisit this for error correction and benchmarking.

### No-cloning (practically important)

There is no universal operation that takes an unknown `|ψ⟩` and outputs `|ψ⟩|ψ⟩` for all `|ψ⟩`. This is why:

- quantum “backup copies” don’t exist
- many classical fault-tolerance intuitions don’t transfer directly

## References

- `04-qubits-and-measurement.md` (Bloch sphere + measurement intuition)
