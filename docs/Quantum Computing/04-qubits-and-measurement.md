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
\[
|\psi\rangle = \cos\frac{\theta}{2}\,|0\rangle + e^{i\phi}\sin\frac{\theta}{2}\,|1\rangle
\]
with \(0 \le \theta \le \pi\), \(0 \le \phi < 2\pi\).

- This corresponds to a point on the **Bloch sphere** with coordinates:
  - \(x = \sin\theta\cos\phi\)
  - \(y = \sin\theta\sin\phi\)
  - \(z = \cos\theta\)

Useful named states:

- \(|0\rangle\): north pole
- \(|1\rangle\): south pole
- \(|+\rangle = (|0\rangle + |1\rangle)/\sqrt{2}\): +x direction
- \(|-\rangle = (|0\rangle - |1\rangle)/\sqrt{2}\): −x direction

### Measurement in different bases

- **Computational (Z) basis**: \(|0\rangle, |1\rangle\)
  - Outcome probabilities given by \(|\alpha|^2, |\beta|^2\) if \(|\psi\rangle = \alpha|0\rangle + \beta|1\rangle\).
- **X basis**: \(|+\rangle, |-\rangle\)
- **Y basis**: eigenstates of Pauli \(Y\) (phase-shifted superpositions).

Implementation idea: to measure in a different basis, you often **rotate the state** then measure in Z.

### Multi-qubit states and measurement

For \(n\) qubits, basis states are \(|x\rangle\) for \(x \in \{0,1\}^n\); measurement gives:

- a bitstring outcome \(x\) with probability \(|\alpha_x|^2\)
- post-measurement state \(|x\rangle\) (in that basis)

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
- **Dephasing**: shrinks coherence between \(|0\rangle\) and \(|1\rangle\) (Z-axis “squash”).
- **Amplitude damping**: relaxation from \(|1\rangle\) toward \(|0\rangle\).

These are the building blocks for error models used in benchmarking and error correction.

## References

- `03-quantum-mechanics-primer.md`
