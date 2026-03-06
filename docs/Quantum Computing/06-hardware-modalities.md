---
title: 'Hardware Modalities and Scaling Constraints'
date: '2026-03-06'
tags: ['quantum-computing', 'hardware', 'scaling']
---

# Hardware Modalities and Scaling Constraints

## Summary

Different physical systems implement qubits with different trade-offs in **coherence, gate speed, connectivity, fabrication, and control complexity**. This note gives you a concise map.

## Notes

### Superconducting qubits

- Implemented with Josephson junction circuits at millikelvin temperatures.
- Pros:
  - Fast gates (tens of ns)
  - Leverages existing microwave and fabrication tech
  - Strong industry investment (IBM, Google, Rigetti, Alice & Bob, etc.)
- Cons:
  - Coherence limited (microseconds–milliseconds typically)
  - 2D layout and cross-talk constraints
  - Heavy cryogenics and control wiring overhead

### Trapped ions

- Qubits are internal states of ions trapped by EM fields, manipulated with lasers.
- Pros:
  - Very long coherence times
  - High-fidelity gates and readout
  - All qubits in a chain can be long-range coupled
- Cons:
  - Slower gates (µs–ms)
  - Scaling to many ions per chain and across modules is hard (shuttling, photonic links).

### Photonic qubits

- Use single photons (polarization, time bins, paths) as carriers.
- Pros:
  - Room-temperature operation possible
  - Naturally good for communication (QKD, networks)
- Cons:
  - Deterministic entangling gates and sources are challenging
  - Losses and detector inefficiencies matter a lot.

### Neutral atoms / Rydberg arrays

- Neutral atoms held in optical tweezers; Rydberg interactions provide gates.
- Pros:
  - Naturally large 2D/3D arrays (hundreds–thousands of qubits)
  - Flexible geometry via dynamic tweezers
- Cons:
  - Error rates and control still maturing
  - Engineering stack less standardized than superconducting/ions.

### Analog / annealing machines

- Quantum annealers and analog simulators target specific Hamiltonians.
- Useful for:
  - optimization heuristics
  - quantum simulation of particular models
- Caution: not universal in the same sense as gate-model machines; guarantees and speedups are more problem- and instance-dependent.

### Scaling constraints (cross-cutting)

- **Error rates vs threshold**: need physical error rates below a code’s threshold to get benefit from error correction.
- **Connectivity**: which qubits can talk directly affects circuit depth and compilation overhead.
- **Control complexity**: number of classical control lines, cryogenic I/O, laser systems, etc.
- **Fabrication and repeatability**: yield and variability across qubits and chips.

## References

- `07-error-correction-ftqc.md`
