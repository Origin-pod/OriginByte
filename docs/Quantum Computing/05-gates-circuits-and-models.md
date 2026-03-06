---
title: 'Gates, Circuits, and Computational Models'
date: '2026-03-06'
tags: ['quantum-computing', 'gates', 'circuits', 'models']
---

# Gates, Circuits, and Computational Models

## Summary

This note covers the **circuit model** used in most quantum computing discussions: what gates are, how circuits compose, and how this relates to other models like adiabatic computing and measurement-based computing.

## Notes

### Gates as unitaries

- A quantum gate on \(k\) qubits is a \(2^k \times 2^k\) **unitary** matrix.
- Gates compose via matrix multiplication; circuits correspond to products of unitaries.

### Common single-qubit gates

- Pauli \(X, Y, Z\)
- Hadamard \(H\): creates/undoes superposition.
- Phase gates \(S, T\): rotations around the Z-axis.

Geometric picture: these gates are rotations on the Bloch sphere.

### Two-qubit gates and entanglement

- **CNOT** (controlled-NOT): entangles and disentangles states; fundamental for universal sets.
- **Controlled-phase** and other controlled rotations.

Any multi-qubit unitary can be decomposed into single-qubit + two-qubit gates from a **universal gate set**.

### Circuit depth, width, and resources

- **Width**: number of qubits used.
- **Depth**: number of time steps / layers, often counted in **two-qubit gate layers** (they’re usually the bottleneck).
- **T-count / T-depth**: in fault-tolerant settings, non-Clifford gates (like \(T\)) dominate overhead.

### Other models and equivalences

- **Adiabatic model**: slowly change a Hamiltonian so the system stays in the ground state; equivalent in power to the circuit model under broad conditions.
- **Measurement-based (cluster state) model**: prepare a highly entangled resource state, then drive computation via adaptive single-qubit measurements.
- **Topological model**: computation via braiding of anyons (in some physical proposals).

For most software-level work, you can stay in the circuit model and treat others as implementation details.

## References

- `04-qubits-and-measurement.md`
