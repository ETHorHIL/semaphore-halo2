use halo2_proofs::{dev::MockProver, pasta::Fp};
use smt::smt::SparseMerkleTree;
mod mul;
mod semaphore;
use semaphore::SemaphoreCircuit;
use smt::poseidon::FieldHasher;
use smt::poseidon::{Poseidon, SmtP128Pow5T3};
use std::marker::PhantomData;

fn main() {
    const HEIGHT: usize = 3; // number of layers in tree
    let k = 8; //number of rows in circuit is 2^k
    let poseidon_2 = Poseidon::<Fp, 2>::new(); // poseidon hashfn with 2 inputs

    // private values
    let identity_nullifier = Fp::from(11222333444);
    let identity_trapdoor = Fp::from(555666777888);

    // publicly available values
    let external_nullifier = Fp::from(42);
    let signal_hash = Fp::from(99);

    // secret is hash(identity_nullifier,identity_trapdoor)
    let secret = poseidon_2
        .hash([identity_nullifier, identity_trapdoor])
        .unwrap();

    // having trouble with the poseidon_1 single input hence hashing twice
    let identity_commitment = poseidon_2.hash([secret, secret]).unwrap();

    let nullifier_hash = poseidon_2
        .hash([external_nullifier, identity_nullifier])
        .unwrap();

    // some leafes, the rest will be padded automatically
    let leaves = [Fp::from(1), identity_commitment, Fp::from(1)];

    // create the merkle tree it automatically adds empty leaves for the given height
    let smt = SparseMerkleTree::<Fp, Poseidon<Fp, 2>, HEIGHT>::new_sequential(
        &leaves,
        &poseidon_2,
        &[0u8; 64],
    )
    .unwrap();

    // generate a path to the first element
    let path = smt.generate_membership_proof(1);

    // calculate the root from the path
    let root = path.calculate_root(&leaves[1], &poseidon_2).unwrap();

    let circuit = SemaphoreCircuit::<Fp, SmtP128Pow5T3<Fp, 0>, Poseidon<Fp, 2>, 3, 2, HEIGHT> {
        path,
        root,
        identity_nullifier,
        identity_trapdoor,
        signal_hash,
        external_nullifier,
        nullifier_hash,
        _spec: PhantomData,
    };

    let public_inputs = vec![signal_hash, external_nullifier, nullifier_hash, root];

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
    println!("mock proof verified");
}
