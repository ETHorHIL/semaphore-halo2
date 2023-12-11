use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Chip, Layouter, Region},
    plonk::{Advice, Column, ConstraintSystem, Error, Selector},
    poly::Rotation,
};
use std::clone::Clone;
use std::marker::PhantomData;

pub trait NumericOperation<F: FieldExt>: Chip<F> {
    type Num;
    fn mul(
        &self,
        layouter: &mut impl Layouter<F>,
        a: AssignedCell<F, F>,
        b: AssignedCell<F, F>,
    ) -> Result<Self::Num, Error>;
}

pub struct FieldChip<F: FieldExt> {
    config: FieldConfig,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> Chip<F> for FieldChip<F> {
    type Config = FieldConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

#[derive(Clone, Debug)]
pub struct FieldConfig {
    advice: [Column<Advice>; 2],
    s_mul: Selector,
}

impl<F: FieldExt> FieldChip<F> {
    pub fn construct(config: <Self as Chip<F>>::Config) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    pub fn configure(meta: &mut ConstraintSystem<F>, advice: [Column<Advice>; 2]) -> FieldConfig {
        for column in &advice {
            meta.enable_equality(*column);
        }
        let s_mul = meta.selector();
        meta.create_gate("mul", |meta| {
            let lhs = meta.query_advice(advice[0], Rotation::cur());
            let rhs = meta.query_advice(advice[1], Rotation::cur());
            let out = meta.query_advice(advice[0], Rotation::next());
            let s_mul = meta.query_selector(s_mul);
            vec![s_mul * (lhs * rhs - out)]
        });

        FieldConfig { advice, s_mul }
    }
}

#[derive(Clone)]
pub struct Number<F: FieldExt>(AssignedCell<F, F>);

impl<F: FieldExt> NumericOperation<F> for FieldChip<F> {
    type Num = Number<F>;

    fn mul(
        &self,
        layouter: &mut impl Layouter<F>,
        a: AssignedCell<F, F>,
        b: AssignedCell<F, F>,
    ) -> Result<Number<F>, Error> {
        let config = &self.config;
        layouter.assign_region(
            || "mul",
            |mut region: Region<'_, F>| {
                config.s_mul.enable(&mut region, 0)?;
                a.copy_advice(|| "rhs", &mut region, config.advice[0], 0)?;
                b.copy_advice(|| "lhs", &mut region, config.advice[1], 0)?;
                let value = a.value().copied() * b.value();
                region
                    .assign_advice(|| "rhs * lhs", config.advice[0], 1, || value)
                    .map(Number)
            },
        )
    }
}
