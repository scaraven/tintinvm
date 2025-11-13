use ark_ff::{
    AdditiveGroup, Field,
    fields::{Fp64, MontBackend, MontConfig},
};

#[derive(MontConfig)]
#[modulus = "97"]
#[generator = "5"]
pub struct FqConfig;
pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

pub const ZERO: Fq = Fq::ZERO;
pub const ONE: Fq = Fq::ONE;
