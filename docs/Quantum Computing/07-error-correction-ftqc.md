---
title: 'Quantum Error Correction and Fault Tolerance'
date: '2026-03-06'
tags: ['quantum-computing', 'error-correction', 'fault-tolerance']
---

# Quantum Error Correction and Fault Tolerance

## Summary

Error correction is the bridge from noisy physical qubits to **logical qubits** that can run deep algorithms. The key idea: encode information across many physical qubits so that errors can be detected and corrected without learning the encoded quantum state.

## Notes

### Why classical intuition fails

- You can’t just **copy** an unknown quantum state (no-cloning theorem).
- Measuring qubits directly would collapse the state.

So QEC uses **syndrome measurements**: measure carefully chosen observables that reveal **where errors happened**, not the logical state itself.

### Codes, stabilizers, and syndromes

- A **quantum code** encodes \(k\) logical qubits into \(n\) physical qubits.
- **Stabilizer codes** describe the code space as the +1 eigenspace of a set of commuting Pauli operators (stabilizers).
- Measuring stabilizers yields **syndromes** (patterns of ±1) that indicate which error likely occurred.

### Surface code (very high level)

- Qubits arranged on a 2D lattice; stabilizers correspond to “plaquettes” and “stars”.
- Pros:
  - High threshold error rate (≈1% region, implementation-dependent).
  - Local interactions (good for planar hardware).
- Cons:
  - Large overhead: thousands of physical qubits per high-quality logical qubit for demanding tasks.

### Logical qubits and thresholds

- A **logical qubit** is an encoded qubit; logical gates operate on the encoded space.
- If physical error rate \(p\) is **below threshold**, then:
  - Increasing the code distance (more physical qubits) can make logical error rate \(p_L\) fall roughly like \((p/p_{\text{th}})^{(d+1)/2}\).
  - This is the core “scaling” property you need for FTQC.

### Biased noise and bosonic codes (link to Alice & Bob)

- In some devices, one error type (e.g. bit flips) is much rarer than another (e.g. phase flips) → **biased noise**.
- **Bosonic codes** encode qubits into harmonic oscillators (modes) rather than discrete qubits.
- **Cat qubits** are a type of bosonic code that:
  - strongly suppress one error channel (e.g. bit flips)
  - allow tailored codes that exploit this bias for lower overhead.

Alice & Bob’s approach fits here: leverage biased noise + bosonic codes + concatenated schemes to reduce the number of physical qubits needed per logical qubit.

## References

- `06-hardware-modalities.md`
- `08-alice-and-bob.md`
