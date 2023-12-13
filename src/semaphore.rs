use chiplet::poseidon_chip::{PoseidonChip, PoseidonConfig};
use chiplet::smt_chip::{PathChip, PathConfig};
use chiplet::utilities::{AssertEqualChip, AssertEqualConfig};
use halo2_gadgets::poseidon::primitives::Spec;
use halo2_proofs::plonk::Instance;
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{floor_planner::V1, Cell, Layouter, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error},
};

use crate::mul::NumericOperation;
use crate::mul::{FieldChip, FieldConfig};
use halo2_proofs::dev::CircuitLayout;
use smt::poseidon::FieldHasher;
use smt::smt::Path;
use std::clone::Clone;
use std::marker::PhantomData;

use plotters::prelude::*;

#[derive(Clone)]
pub struct SemaphoreConfig<
    F: FieldExt,
    S: Spec<F, WIDTH, RATE>,
    H: FieldHasher<F, 2>,
    const WIDTH: usize,
    const RATE: usize,
    const N: usize,
> {
    path_config: PathConfig<F, S, WIDTH, RATE, N>,
    poseidon_config_2: PoseidonConfig<F, WIDTH, RATE>,
    advices: [Column<Advice>; 2],
    assert_equal_config: AssertEqualConfig<F>,
    field_chip: FieldConfig,
    public_inputs: Column<Instance>,
    _hasher: PhantomData<H>,
}

pub struct SemaphoreCircuit<
    F: FieldExt,
    S: Spec<F, WIDTH, RATE>,
    H: FieldHasher<F, 2>,
    const WIDTH: usize,
    const RATE: usize,
    const N: usize,
> {
    pub path: Path<F, H, N>,
    pub root: F,
    pub identity_nullifier: F,
    pub identity_trapdoor: F,
    pub signal_hash: F,
    pub external_nullifier: F,
    pub _spec: PhantomData<S>,
}

impl<
        F: FieldExt,
        S: Spec<F, WIDTH, RATE> + Clone,
        H: FieldHasher<F, 2> + Clone,
        const WIDTH: usize,
        const RATE: usize,
        const N: usize,
    > SemaphoreCircuit<F, S, H, WIDTH, RATE, N>
{
    fn expose_public(
        &self,
        mut layouter: impl Layouter<F>,
        column: Column<Instance>,
        var: Cell,
        row: usize,
    ) -> Result<(), Error> {
        layouter.constrain_instance(var, column, row)
    }

    pub fn plot(&self) {
        let drawing_area = BitMapBackend::new("example-circuit-layout.png", (1024 * 2, 768 * 2))
            .into_drawing_area();
        drawing_area.fill(&WHITE).unwrap();
        let drawing_area = drawing_area
            .titled("Example Circuit Layout", ("sans-serif", 60))
            .unwrap();

        let circuit = self.without_witnesses();
        let k = 8; // Suitable size for MyCircuit
        CircuitLayout::default()
            .render(k, &circuit, &drawing_area)
            .unwrap();
    }
}
impl<
        F: FieldExt,
        S: Spec<F, WIDTH, RATE> + Clone,
        H: FieldHasher<F, 2> + Clone,
        const WIDTH: usize,
        const RATE: usize,
        const N: usize,
    > Circuit<F> for SemaphoreCircuit<F, S, H, WIDTH, RATE, N>
{
    type Config = SemaphoreConfig<F, S, H, WIDTH, RATE, N>;
    type FloorPlanner = V1;

    fn without_witnesses(&self) -> Self {
        Self {
            path: Path {
                path: [(F::zero(), F::zero()); N],
                marker: PhantomData,
            },
            root: F::zero(),
            identity_nullifier: F::zero(),
            identity_trapdoor: F::zero(),
            signal_hash: F::zero(),
            external_nullifier: F::zero(),
            _spec: PhantomData,
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advices = [(); 2].map(|_| meta.advice_column());
        advices
            .iter()
            .for_each(|column| meta.enable_equality(*column));

        let public_inputs = meta.instance_column();
        meta.enable_equality(public_inputs);

        SemaphoreConfig {
            path_config: PathChip::<F, S, H, WIDTH, RATE, N>::configure(meta),
            poseidon_config_2: PoseidonChip::<F, S, WIDTH, RATE, 2>::configure(meta),
            advices,
            assert_equal_config: AssertEqualChip::configure(meta, [advices[0], advices[1]]),
            field_chip: FieldChip::configure(meta, [advices[0], advices[1]]),
            public_inputs,
            _hasher: PhantomData,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let (
            identity_nullifier_cell,
            identity_trapdoor_cell,
            one_cell,
            signal_hash_cell,
            external_nullifier_cell,
            root_cell,
        ) = layouter.assign_region(
            || "semaphore circuit",
            |mut region| {
                // we have two advice colums, setting the first three inputs in col 0 and next in col 1
                // the reason we have two instead of one advice column is the fieldchip and assert_equal chip which require two
                // otherwise we would have put the values below in one because the number of rows is for sure sufficient
                let identity_nullifier = region.assign_advice(
                    || "identity_nullifier",
                    config.advices[0],
                    0,
                    || Value::known(self.identity_nullifier),
                )?;

                let identity_trapdoor = region.assign_advice(
                    || "identity_trapdoor",
                    config.advices[0],
                    1,
                    || Value::known(self.identity_trapdoor),
                )?;

                let one = region.assign_advice(
                    || "one",
                    config.advices[0],
                    2,
                    || Value::known(F::one()),
                )?;

                let signal_hash = region.assign_advice(
                    || "signal_hash",
                    config.advices[1],
                    0,
                    || Value::known(self.signal_hash),
                )?;

                let external_nullifier = region.assign_advice(
                    || "external_nullifier",
                    config.advices[1],
                    1,
                    || Value::known(self.external_nullifier),
                )?;

                let root = region.assign_advice(
                    || "root",
                    config.advices[1],
                    2,
                    || Value::known(self.root),
                )?;

                Ok((
                    identity_nullifier,
                    identity_trapdoor,
                    one,
                    signal_hash,
                    external_nullifier,
                    root,
                ))
            },
        )?;

        let poseidon_chip_2 =
            PoseidonChip::<F, S, WIDTH, RATE, 2>::construct(config.poseidon_config_2);

        let secret_circuit = poseidon_chip_2.hash(
            &mut layouter,
            &[identity_nullifier_cell.clone(), identity_trapdoor_cell],
        )?;

        let leaf_circuit =
            poseidon_chip_2.hash(&mut layouter, &[secret_circuit.clone(), secret_circuit])?;

        let nullifier_hash_circuit = poseidon_chip_2.hash(
            &mut layouter,
            &[external_nullifier_cell.clone(), identity_nullifier_cell],
        )?;

        let path_chip = PathChip::<F, S, H, WIDTH, RATE, N>::from_native(
            config.path_config,
            &mut layouter,
            self.path.clone(),
        )?;

        let membership_cell =
            path_chip.check_membership(&mut layouter, root_cell.clone(), leaf_circuit)?;

        // in the PSE circom version they dod this to "prevent tempering withth"
        let mul_chip = FieldChip::<F>::construct(config.field_chip);
        mul_chip.mul(
            &mut layouter,
            signal_hash_cell.clone(),
            signal_hash_cell.clone(),
        )?;

        let assert_equal_chip = AssertEqualChip::construct(config.assert_equal_config, ());

        assert_equal_chip.assert_equal(&mut layouter, membership_cell, one_cell)?;

        self.expose_public(
            layouter.namespace(|| "constrain signal_hash"),
            config.public_inputs,
            signal_hash_cell.cell(),
            0,
        )?;

        self.expose_public(
            layouter.namespace(|| "constrain external_nullifier"),
            config.public_inputs,
            external_nullifier_cell.cell(),
            1,
        )?;

        self.expose_public(
            layouter.namespace(|| "constrain nullifier_hash"),
            config.public_inputs,
            nullifier_hash_circuit.cell(),
            2,
        )?;

        self.expose_public(
            layouter.namespace(|| "constrain root"),
            config.public_inputs,
            root_cell.cell(),
            3,
        )?;

        Ok(())
    }
}
