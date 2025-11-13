use ark_ff::fields::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "97"]
#[generator = "5"]
pub struct FqConfig;
pub type Fq = Fp64<MontBackend<FqConfig, 1>>;
