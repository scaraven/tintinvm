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

mod tests {
    use super::*;

    #[test]
    fn test_field_addition() {
        let a = Fq::from(10u64);
        let b = Fq::from(20u64);
        let c = a + b;
        assert_eq!(c, Fq::from(30u64));
    }

    #[test]
    fn test_field_multiplication() {
        let a = Fq::from(10u64);
        let b = Fq::from(20u64);
        let c = a * b;
        assert_eq!(c, Fq::from(6u64)); // 200 mod 97 = 6
    }
}
