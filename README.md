# Semaphore Circuits Implementation in Halo2

This repository contains an educational implementation of [Semaphore circuits](https://github.com/semaphore-protocol/semaphore/blob/main/packages/circuits/semaphore.circom) using [Halo2](https://github.com/zcash/halo2). One may find it useful as a more complex complement to the [SimpleExample in the Halo2 book](https://zcash.github.io/halo2/user/simple-example.html).

## Disclaimer

This implementation is for educational purposes and likely contains bugs. It is not recommended for production use. For a higher quality Semaphore implementation in Halo2, see [Andrija Novakovic's work](https://github.com/akinovak/halo2-semaphore/tree/main).

## Features
- Basic implementation of Semaphore circuits in Halo2.
- Utilizes the [Merkle tree gadget by young-rocks](https://github.com/young-rocks/rocks-smt/tree/main) without alterations.
- The circuit currently uses just above 2^7 rows, with potential optimizations to bring it below 2^7.

## Getting Started

### Prerequisites

- Basic understanding of the arithmetization step in zero-knowledge proofs.
- Familiarity with Rust programming language.

### Recommended Learning Path

1. Read sections 1 and 2 of the [Halo2 book](https://zcash.github.io/halo2/index.html).
2. Go through [Errol Drummond's Halo2 Tutorial](https://erroldrummond.gitbook.io/halo2-tutorial/section-3/gadgets).
3. Attend [0xparc's Halo2 learning group lectures](https://learn.0xparc.org/halo2/).
4. Build a basic circuit using multiple chips in Halo2.
5. Revisit the Halo2 book and lectures to reinforce your understanding.
6. Experiment with the circuit-layout printer to optimize the number of rows.

### Running the Example

- `main.rs` contains a basic example to get started. Modify and execute it to see how the implementation works.

## Contributing

While this project is primarily educational, contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests.

## Acknowledgments

- [Semaphore protocol](https://github.com/semaphore-protocol/semaphore) for the original Semaphore circuits.
- [Andrija Novakovic](https://github.com/akinovak/halo2-semaphore/tree/main) for the alternative Halo2 Semaphore implementation.
- [young-rocks](https://github.com/young-rocks/rocks-smt/tree/main) for the Merkle tree gadget.

## Background

My background includes implementing [vanilla plonk](https://github.com/ETHorHIL/plonk-rs) and [plonk with lookups](https://github.com/ETHorHIL/plonk-with-plookup). These experiences, although not necessary, were beneficial in understanding and working with Halo2.
