// let make it easy

pub mod polynomial;

use ark_ff::Field;
use polynomial::MultilinearPolynomial;

/// Sumcheck protocol
/// Reduce Sum {f(u)} = C to f(r_1, r_2, ...,r_n) = f_n(r_n)
/// In this lib f(u) is multilinear polynomial.
pub struct Sumcheck<F: Field> {
    pub ml_poly: MultilinearPolynomial<F>,
}

impl<F: Field> Sumcheck<F> {
    pub fn verify() {
        todo!()
    }

    pub fn prove() {
        todo!()
    }
}
