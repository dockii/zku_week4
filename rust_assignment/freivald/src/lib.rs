use ark_bls12_381::Fq;
use ark_std;
use ndarray;

// declare types for convenience and readability
type Vector = ndarray::Array1<Fq>;
type Matrix = ndarray::Array2<Fq>;

pub struct Freivald {
    x: Vec<Vector>, // vector of Array/Vec of Fq,
}

impl Freivald {
    // creates an instance of algorithm with one verification vector
    pub fn new(array_size: usize) -> Self {
        // delegate to a constructor with 1 verification vector
        Self::with_vectors(array_size, 1)
    }

    // creates an instance of algorithm with n verification vectors
    pub fn with_vectors(array_size: usize, verification_vectors: usize) -> Self {
        let vectors: Vec<Vector> = (0..verification_vectors)
            .map(|_| Self::random_vector(array_size))
            .collect();
        Self { x: vectors }
    }

    // generate a random vector used for verification
    fn random_vector(n: usize) -> Vector {
        // Generate random number
        let mut r = ark_std::rand::random();
        // Populate vector with values r^i for i=0..matrix_size
        let mut v = ndarray::Array::zeros(n);
        for i in 1..n {
            v[i] = r;
            r *= r;
        }
        v
    }

    pub fn verify(&self, matrix_a: &Matrix, matrix_b: &Matrix, supposed_ab: &Matrix) -> bool {
        assert!(check_matrix_dimensions(matrix_a, matrix_b, supposed_ab));

        // iterate over all verification vectors
        for v in &self.x {
            let left = matrix_a.dot(&matrix_b.dot(v));
            let right = supposed_ab.dot(v);
            // if one verification vector fails verification, then supposed_ab is not result of a*b
            if left != right {
                return false;
            }
        }

        // all verification vectors confirm multiplication
        true
    }

    // utility function to not have to instantiate Freivalds if you just want to make one
    // verification.
    pub fn verify_once(matrix_a: &Matrix, matrix_b: &Matrix, supposed_ab: &Matrix) -> bool {
        let freivald = Freivald::new(supposed_ab.nrows());
        freivald.verify(matrix_a, matrix_b, supposed_ab)
    }
}

// [Bonus] Modify code to increase your certainty that A * B == C by iterating over the protocol.
// Note that you need to generate new vectors for new iterations or you'll be recomputing same
// value over and over. No problem in changing data structures used by the algorithm (currently its a struct
// but that can change if you want to)

// You can either do a test on main or just remove main function and rename this file to lib.rs to remove the
// warning of not having a main implementation
// (converted to a lib)

pub fn check_matrix_dimensions(matrix_a: &Matrix, matrix_b: &Matrix, supposed_ab: &Matrix) -> bool {
    // Check if dimensions of making matrix_a * matrix_b matches values in supposed_ab.
    // If it doesn't you know its not the correct result independently of matrix contents
    (matrix_a.dim() == matrix_b.dim()) && (matrix_a.dim() == supposed_ab.dim()) // simplified, because all matrices are square
}

#[cfg(test)]
mod tests {
    // #[macro_use]
    use lazy_static::lazy_static;
    use rstest::rstest;

    use super::*;

    lazy_static! {
        // size of matrices in tests
        static ref SIZE: usize = 200;

        // note: the structure of test cases forces all of these matrices to be the same size
        static ref MATRIX_A: Matrix = random_matrix(*SIZE);
        static ref MATRIX_A_DOT_A: Matrix = dot(&MATRIX_A, &MATRIX_A);
        static ref MATRIX_B: Matrix = random_matrix(*SIZE);
        static ref MATRIX_B_DOT_B: Matrix = dot(&MATRIX_B, &MATRIX_B);
        static ref MATRIX_C: Matrix = random_matrix(*SIZE);
        static ref MATRIX_C_DOT_C: Matrix = dot(&MATRIX_C, &MATRIX_C);
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_C, &MATRIX_C_DOT_C)]
    fn freivald_verify_success_test(
        #[case] matrix_a: &Matrix,
        #[case] matrix_b: &Matrix,
        #[case] supposed_ab: &Matrix,
    ) {
        let freivald = Freivald::new(supposed_ab.nrows());
        assert!(freivald.verify(matrix_a, matrix_b, supposed_ab));
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_B, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_A, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_B, &MATRIX_C_DOT_C)]
    fn freivald_verify_fail_test(#[case] a: &Matrix, #[case] b: &Matrix, #[case] c: &Matrix) {
        let freivald = Freivald::new(c.nrows());
        assert!(!freivald.verify(a, b, c));
    }

    // generates random n-by-n matrix
    fn random_matrix(n: usize) -> Matrix {
        let a = ndarray::Array::from_iter((0..n * n).map(|_| ark_std::rand::random()));
        a.into_shape((n, n)).unwrap()
    }
    // calculates matrices product using ndarray implementation
    fn dot(a: &Matrix, b: &Matrix) -> Matrix {
        a.dot(b)
    }
}
