---
title: 'Cryptography: QKD and Post-Quantum Crypto'
date: '2026-03-06'
tags: ['quantum-computing', 'cryptography', 'qkd', 'pqc']
---

# Cryptography: QKD and Post-Quantum Crypto

## Summary

Quantum affects cryptography along two axes:

- **QKD (Quantum Key Distribution)**: uses quantum channels to *establish* keys with eavesdropping detection.
- **PQC (Post-Quantum Cryptography)**: classical cryptography designed to stay secure even against quantum computers.

They solve different problems; PQC is the main path for Internet-scale deployment, while QKD targets specialized links.

## Notes

### Shor’s algorithm and impact

- Large fault-tolerant quantum computers running Shor’s algorithm could break:
  - RSA
  - Elliptic-curve schemes (ECDSA, ECDH, etc.)
- This motivates:
  - **“Harvest now, decrypt later” risk**: adversaries store encrypted traffic today, decrypt if/when they get a quantum computer.
  - Migration to algorithms that resist both classical and quantum attacks.

### QKD (Quantum Key Distribution)

Goal: establish a shared secret key between Alice and Bob such that **any eavesdropping is detectable**.

Core idea:

- Quantum states (e.g. photon polarizations) carry potential key bits.
- Any attempt by an eavesdropper (Eve) to **measure** these states disturbs them, changing statistics that Alice and Bob can detect.

Two main families:

- **BB84 (prepare-and-measure)**:
  - Alice sends single photons prepared in one of two non-orthogonal bases.
  - Bob measures in randomly chosen bases.
  - They publicly compare bases (not outcomes) and keep only matches; error rates indicate eavesdropping.
- **E91 (entanglement-based)**:
  - A source (possibly untrusted) sends entangled photon pairs to Alice and Bob.
  - Security is tied to violations of Bell inequalities, proving genuine entanglement.

Limitations:

- Requires specialized hardware and often dedicated links.
- Mostly realistic for backbone/point-to-point links, not general-purpose Internet endpoints.

### Post-Quantum Cryptography (PQC)

PQC = **classical** schemes designed to resist attacks from both classical and quantum adversaries.

As of mid-2020s, NIST has selected:

- **ML-KEM (Kyber)**: lattice-based KEM for key establishment.
- **ML-DSA (Dilithium)**: lattice-based digital signatures.
- **FN-DSA (Falcon)** and **SLH-DSA (SPHINCS+)**: additional signature schemes.
- **HQC**: additional key-establishment algorithm (fourth-round selection).

These are deployed over classical networks and are the main migration target for real-world systems.

### How to think about QKD vs PQC

- **QKD**:
  - Pros: information-theoretic security under idealized assumptions, strong eavesdrop detection.
  - Cons: infrastructure-heavy, limited topologies, still needs authentication (often via PQC or classical crypto).
- **PQC**:
  - Pros: software-level deployment, scales like current public-key crypto.
  - Cons: relies on hardness assumptions (like today’s crypto).

For most practical purposes, “what you need to do” is **migrate to PQC**, while QKD remains a specialized tool.

## References

- NIST PQC project overview: `https://csrc.nist.gov/projects/post-quantum-cryptography/selected-algorithms`
