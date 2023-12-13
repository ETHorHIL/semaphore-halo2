use halo2_proofs::{dev::MockProver, pasta::Fp};
use smt::smt::SparseMerkleTree;
mod mul;
mod semaphore;

use semaphore::SemaphoreCircuit;
use smt::poseidon::FieldHasher;
use smt::poseidon::{Poseidon, SmtP128Pow5T3};
use std::marker::PhantomData;

fn main() {
    const HEIGHT: usize = 20; // number of layers in tree
    let k = 9; //number of rows in circuit is 2^k
    let poseidon_2 = Poseidon::<Fp, 2>::new(); // poseidon hashfn with 2 inputs

    // private values
    let identity_nullifier = Fp::from(11222333444);
    let identity_trapdoor = Fp::from(555666777888);

    // publicly available values
    let external_nullifier = Fp::from(42);
    let signal_hash = Fp::from(99);

    // calculate values used in Semaphore protocol
    // having trouble with the poseidon_1 single input hence hashing identity_commitment twice
    let secret = poseidon_2
        .hash([identity_nullifier, identity_trapdoor])
        .unwrap();
    let nullifier_hash = poseidon_2
        .hash([external_nullifier, identity_nullifier])
        .unwrap();
    let identity_commitment = poseidon_2.hash([secret, secret]).unwrap();

    // create the merkle tree it automatically adds empty leaves for the given height
    let smt = SparseMerkleTree::<Fp, Poseidon<Fp, 2>, HEIGHT>::new_sequential(
        &[Fp::from(1), identity_commitment], // moving the identity_commitment to position 1
        &poseidon_2,
        &[0u8; 64],
    )
    .unwrap();

    // generate a path to the first element
    let path = smt.generate_membership_proof(1);

    // calculate the root from the path
    let root = path
        .calculate_root(&identity_commitment, &poseidon_2)
        .unwrap();

    // construct the circuit
    let circuit = SemaphoreCircuit::<Fp, SmtP128Pow5T3<Fp, 0>, Poseidon<Fp, 2>, 3, 2, HEIGHT> {
        path,
        root,
        identity_nullifier,
        identity_trapdoor,
        signal_hash,
        external_nullifier,
        _spec: PhantomData,
    };
    circuit.plot();

    // specify the public inputs
    let public_inputs = vec![signal_hash, external_nullifier, nullifier_hash, root];

    // proove
    MockProver::run(k, &circuit, vec![public_inputs])
        .unwrap()
        .assert_satisfied();
    println!("mock proof verified");

    // proving will fail if we change any of the public inputs
    let public_inputs = vec![
        signal_hash,
        external_nullifier,
        nullifier_hash + Fp::from(1),
    ];
    let prover = MockProver::run(k, &circuit, vec![public_inputs]).unwrap();
    assert!(prover.verify().is_err());
    println!("wrong inputs failed successfully");
}
