---
title: 'What is Quantum Computing?'
---

# What is Quantum Computing?

## Summary

Quantum computing is a model of computation that uses **quantum states** as information carriers. It can outperform classical computing for *some* tasks by exploiting:

- **Superposition** (state vectors add)
- **Interference** (amplitudes reinforce/cancel)
- **Entanglement** (non-classical correlations)

The punchline: you **don’t get speedups by “trying all answers at once”**. You get speedups when you design circuits so that wrong answers destructively interfere and the right answers constructively interfere *at measurement time*.

## Notes

### The unit of information: qubit vs bit

- **Classical bit**: \(0\) or \(1\).
- **Qubit**: \(|\psi\rangle = \alpha|0\rangle + \beta|1\rangle\), with complex amplitudes \(\alpha,\beta\) and \(|\alpha|^2 + |\beta|^2 = 1\).
- When you **measure** a qubit in the computational basis:
  - You see `0` with probability \(|\alpha|^2\)
  - You see `1` with probability \(|\beta|^2\)
  - The state “collapses” to the observed basis state (in that basis).

### Three “views” that help

- **Linear algebra view**: states are vectors, gates are matrices, measurement gives probabilities.
- **Circuit view**: computation is a sequence of gates on qubits, then measurement.
- **Information view**: noise is the enemy; useful quantum computing requires controlling errors.

### Computational models you’ll hear about

- **Gate-based (circuit model)**: universal; the default when people say “quantum computer”.
- **Quantum annealing / adiabatic**: optimization-oriented; different guarantees/assumptions.
- **Measurement-based (cluster states)**: compute by adaptive measurements on entangled resource states.

### Where quantum helps (and doesn’t)

- **Known strong candidates for advantage**
  - Factoring / discrete log (Shor) → breaks RSA/ECC *if* fault-tolerant machines exist at scale.
  - Some structured linear algebra / simulation problems.
  - Quantum simulation of chemistry/materials (often cited as first “useful” FTQC app).
- **Not a general replacement**
  - Most everyday computing tasks won’t benefit.
  - Quantum speedups are about asymptotics *and* constant factors; overhead can dominate.

### NISQ vs FTQC

- **NISQ**: Noisy devices with tens–thousands of physical qubits; errors limit depth.
- **FTQC** (fault-tolerant quantum computing): uses **quantum error correction** to create logical qubits with extremely low error rates.
- Many headline use-cases require **FTQC**, because they need billions/trillions of reliable operations.

### Common misconceptions (worth killing early)

- “A qubit stores infinite information.”  
  - A pure state is described by continuous parameters, but measurement yields limited classical bits unless you repeat/structure experiments. Information access is constrained (Holevo bound, etc.).
- “Quantum computes all branches and reads the answer.”  
  - You don’t get to read all branches. You only sample an outcome distribution shaped by interference.

## References

- `00-index.md` (this folder’s roadmap)
