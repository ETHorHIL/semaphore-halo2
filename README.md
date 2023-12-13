# Semaphore Circuits Implementation in Halo2

This repository contains an educational implementation of [Semaphore circuits](https://github.com/semaphore-protocol/semaphore/blob/main/packages/circuits/semaphore.circom) using [Halo2](https://github.com/zcash/halo2). One may find it useful as a more complex complement to the [SimpleExample in the Halo2 book](https://zcash.github.io/halo2/user/simple-example.html).

## Disclaimer

This implementation is for educational purposes and likely contains bugs. It is not recommended for production use. For a higher quality Semaphore implementation in Halo2, see [Andrija Novakovic's work](https://github.com/akinovak/halo2-semaphore/tree/main).

## Features

- Basic implementation of Semaphore circuits in Halo2.
- Utilizes the [Merkle tree gadget by young-rocks](https://github.com/young-rocks/rocks-smt/tree/main) without alterations.

The circuit size is dominated by the Poseidon circuits. Each layer of the merkle tree adds rows to the circuit. A tree of height 48 (281,474,976,710,656 leafes) fits into 2^11 rows. The next smaller circuit with 2^10 rows would only be able to support 2^24 (16,777,216) leafes i.e. identities in the anonymity set. Halo2 allows to use more columns instead of rows to trade off prover time against verifier time.

## How I went about this project

### Prerequisites

- Basic understanding of the arithmetization step in zero-knowledge proofs.
- Familiarity with Rust programming language.

My prior knowledge includes implementing [vanilla plonk](https://github.com/ETHorHIL/plonk-rs) and [plonk with lookups](https://github.com/ETHorHIL/plonk-with-plookup). These experiences, although not necessary, were beneficial in understanding and working with Halo2.

### Learning Path

1. Read sections 1 and 2 of the [Halo2 book](https://zcash.github.io/halo2/index.html).
2. Go through [Errol Drummond's Halo2 Tutorial](https://erroldrummond.gitbook.io/halo2-tutorial/section-3/gadgets).
3. Attend [0xparc's Halo2 learning group lectures](https://learn.0xparc.org/halo2/).
4. Build a basic circuit using more than one chip in Halo2.
5. Revisit the Halo2 book and lectures to reinforce your understanding.
6. Experiment with the circuit-layout printer to optimize the number of rows.

### Running the Example

- `main.rs` contains a basic example to get started. Modify and execute it to see how the implementation works.
