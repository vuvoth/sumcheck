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
        self.power.iter().sum::<usize>()
    }

    pub fn evaluate(&self, input: &[F]) -> F {
        let mut mul = self.coefficient;
        for (i, value) in input.iter().enumerate() {
            if self.power[i] == 1 {
                mul *= value
            }
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

    use super::MultiVarMonomial;

    use ark_test_curves::bls12_381::Fr as F;

    #[test]
    fn monomial_test() {
        let power = vec![1, 1, 0];
        let a = F::from(10);
        let b = F::from(10);

        let c = a + b;
        
        let coeff = F::from(10);

        let term = MultiVarMonomial::<F>::new(coeff, power);
        let ans = term.evaluate(&[a, b, c]);

        assert_eq!(ans, a * b * coeff);
    }
}
