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

- A pure state is a normalized vector \(|\psi\rangle\) in a complex vector space (Hilbert space).
- For one qubit, a common basis is \(\{|0\rangle, |1\rangle\}\).
- For \(n\) qubits, the space dimension is \(2^n\).

### Superposition is linear combination

\[
|\psi\rangle = \sum_x \alpha_x |x\rangle,\quad \sum_x |\alpha_x|^2 = 1
\]

The \(\alpha_x\) are **amplitudes**; probabilities come from **squared magnitudes**.

### Global phase doesn’t matter (relative phase does)

- \(|\psi\rangle\) and \(e^{i\theta}|\psi\rangle\) represent the same physical state.
- But phases *between components* (relative phase) affect interference.

### Evolution (closed systems) is unitary

- Quantum gates are represented by **unitary** matrices \(U\) where \(U^\dagger U = I\).
- Applying a gate updates the state: \(|\psi'\rangle = U|\psi\rangle\).

### Measurement = probabilities + state update

Measuring in the computational basis \(\{|0\rangle,|1\rangle\}\):

- Outcome \(0\) occurs with probability \(|\langle 0|\psi\rangle|^2\)
- Outcome \(1\) occurs with probability \(|\langle 1|\psi\rangle|^2\)

After measurement, the post-measurement state is the normalized projection onto the observed outcome’s subspace.

### Composite systems: tensor products

- Two qubits: \(|\psi\rangle \otimes |\phi\rangle\) (often written \(|\psi\rangle|\phi\rangle\)).
- Basis states: \(|00\rangle, |01\rangle, |10\rangle, |11\rangle\).

### Entanglement

A state is **entangled** if it cannot be written as a simple product state.

Example Bell state:
\[
|\Phi^+\rangle = \frac{1}{\sqrt{2}}(|00\rangle + |11\rangle)
\]

Measuring one qubit gives a correlated result in the other; importantly, this correlation can’t be reproduced by classical shared randomness *for all measurement choices*.

### Interference

Because amplitudes add, paths can cancel.

- If two computational paths lead to the same basis state with opposite phase, their amplitudes can cancel.
- Most quantum algorithms are “interference engines”: design \(U\) so the amplitude mass concentrates on good outcomes.

### Mixed states and density matrices (why you care)

Real devices interact with the environment → noise → you can’t assume a pure state.

- A mixed state is represented by a **density matrix** \(\rho\).
- Pure state: \(\rho = |\psi\rangle\langle\psi|\)
- Noise and measurement can be expressed as **quantum channels** (CPTP maps).

You’ll revisit this for error correction and benchmarking.

### No-cloning (practically important)

There is no universal operation that takes an unknown \(|\psi\rangle\) and outputs \(|\psi\rangle|\psi\rangle\) for all \(|\psi\rangle\). This is why:

- quantum “backup copies” don’t exist
- many classical fault-tolerance intuitions don’t transfer directly

## References

- `04-qubits-and-measurement.md` (Bloch sphere + measurement intuition)
