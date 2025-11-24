// This constant defines how many times larger the number of rows of the 
// polynomial trace is compared to the number of rows in the original VM trace.
// This improves the soundness of the protocol by amplifying any invalid trace row 
pub const REDUNDANCY_FACTOR: usize = 4;

use ark_ff::Field;
use ark_poly::{
    univariate::DensePolynomial,
    EvaluationDomain, GeneralEvaluationDomain,
};
use crate::crypto::field::Fq;

pub struct VMTracePolynomial<F: Field = Fq> {
    /// One polynomial per VM trace column.
    pub columns: Vec<DensePolynomial<F>>,
}

impl<F: Field> VMTracePolynomial<F> {
    /// Create from a vector of column polynomials.
    pub fn new(columns: Vec<DensePolynomial<F>>) -> Self {
        Self { columns }
    }

    /// Number of columns (polynomials) in the trace.
    pub fn num_columns(&self) -> usize {
        self.columns.len()
    }

    /// Immutable slice of all column polynomials.
    pub fn as_slice(&self) -> &[DensePolynomial<F>] {
        &self.columns
    }

    /// Mutable slice of all column polynomials.
    pub fn as_mut_slice(&mut self) -> &mut [DensePolynomial<F>] {
        &mut self.columns
    }
}

/// Trait for converting evaluations on a geometric progression into a polynomial.
pub trait ToCoefficients<F: Field> {
    /// Given evaluations `[f(g), f(g^2), ..., f(g^n)]` and the generator `g`,
    /// return the polynomial `f(x)` in coefficient form.
    fn to_coefficients(evaluations: &[F], generator: F) -> DensePolynomial<F>;
}

/// A default implementation using an IFFT over a multiplicative subgroup
/// when the evaluations are taken over a domain compatible with ark-poly.
impl<F: Field> ToCoefficients<F> for VMTracePolynomial<F> {
    fn to_coefficients(evaluations: &[F], _generator: F) -> DensePolynomial<F> {
        // Note: this assumes `evaluations` correspond to an evaluation domain
        // supported by `GeneralEvaluationDomain`. If not, a custom interpolation
        // routine (e.g. lagrange interpolation) would be required.
        let n = evaluations.len();
        let domain =
            GeneralEvaluationDomain::<F>::new(n).expect("failed to create evaluation domain");
        let mut evals = evaluations.to_vec();
        domain.ifft_in_place(&mut evals);
        DensePolynomial::from_coefficients_vec(evals)
    }
}