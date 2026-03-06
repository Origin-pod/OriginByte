---
title: 'Qubits, Bloch Sphere, and Measurement'
date: '2026-03-06'
tags: ['quantum-computing', 'qubits', 'measurement']
---

# Qubits, Bloch Sphere, and Measurement

## Summary

This note gives a geometric and operational picture of **single-qubit states** and **measurement**, which you’ll use everywhere: gates, circuits, noise models, and error correction.

## Notes

### Single-qubit state and Bloch sphere

Any pure qubit state can be written (up to global phase) as:

`|ψ⟩ = cos(θ/2)|0⟩ + e^{iφ} sin(θ/2)|1⟩`

with `0 ≤ θ ≤ π`, `0 ≤ φ < 2π`.

- This corresponds to a point on the **Bloch sphere** with coordinates:
  - `x = sin θ cos φ`
  - `y = sin θ sin φ`
  - `z = cos θ`

Useful named states:

- `|0⟩`: north pole
- `|1⟩`: south pole
- `|+⟩ = (|0⟩ + |1⟩)/√2`: +x direction
- `|−⟩ = (|0⟩ − |1⟩)/√2`: −x direction

### Measurement in different bases

- **Computational (Z) basis**: `|0⟩, |1⟩`
  - Outcome probabilities given by `|α|^2, |β|^2` if `|ψ⟩ = α|0⟩ + β|1⟩`.
- **X basis**: `|+⟩, |−⟩`
- **Y basis**: eigenstates of Pauli `Y` (phase-shifted superpositions).

Implementation idea: to measure in a different basis, you often **rotate the state** then measure in Z.

### Multi-qubit states and measurement

For `n` qubits, basis states are `|x⟩` for `x ∈ {0,1}^n`; measurement gives:

- a bitstring outcome `x` with probability `|α_x|^2`
- post-measurement state `|x⟩` (in that basis)

You can also measure **subsets of qubits**, leaving the rest in a (generally mixed) conditional state.

### Projective measurement vs POVMs (why you care)

- Most circuit-level descriptions use **projective measurements** in Pauli bases.
- More general measurements (POVMs) appear in:
  - realistic hardware models
  - optimization of readout schemes
  - security proofs in QKD

For this note, you can treat measurements as projectors plus classical outcomes.

### Noise and decoherence (brief)

Noisy processes move states **toward the center** of the Bloch sphere (loss of purity) and/or **rotate** them randomly.

Common single-qubit noise channels:

- **Depolarizing**: random Pauli with small probability.
- **Dephasing**: shrinks coherence between `|0⟩` and `|1⟩` (Z-axis “squash”).
- **Amplitude damping**: relaxation from `|1⟩` toward `|0⟩`.

These are the building blocks for error models used in benchmarking and error correction.

## References

- `03-quantum-mechanics-primer.md`
