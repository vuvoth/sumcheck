use ark_ff::Field;

// multilinear monomial
pub struct MultiVarMonomial<F: Field> {
    coefficient: F,
    power: Vec<usize>, // 0 or 1
}

impl<F: Field> MultiVarMonomial<F> {
    pub fn new(coefficient: F, power: Vec<usize>) -> Self {
        Self { coefficient, power }
    }

    pub fn num_vars(&self) -> usize {
        self.power.iter().fold(0, |acc, element| acc + element)
    }

    pub fn evaluate(&self, input: &[F]) -> F {
        let mut mul = self.coefficient;
        for value in input {
            mul = mul * value
        }
        mul
    }
}
// list of value[x]
pub struct MultilinearPolynomial<F: Field> {
    terms: Vec<MultiVarMonomial<F>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::{BigInteger, Field, PrimeField, Zero};
    use ark_test_curves::{
        bls12_381::{Fr as F, FrConfig},
        secp256k1::Fr,
    };

    #[test]
    fn monomial_test() {
        let power = vec![1, 1, 0];
        let a = F::from(10);

        
        let term = MultiVarMonomial::<F>::new(F::from(10), power);
    }
}
