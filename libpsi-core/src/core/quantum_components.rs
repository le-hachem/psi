use crate::{column_vector, complex, ColumnVector, Complex, Float, Matrix, Vector, VectorMatrix};
use core::{fmt, ops};

#[macro_export]
macro_rules! count {
    () => { 0 };
    ($head:expr $(,$tail:expr)*) => { 1 + count!($( $tail ),*) };
}

#[macro_export]
macro_rules! qubit {
    ($(($re:expr, $im:expr)),*) => {
        {
            let mut vector = Vec::new();
            $(
                vector.push(complex!($re, $im));
            )*
            QuantumBit::new(vector)
        }
    };
}

#[macro_export]
macro_rules! quantum_register {
    ($($bit:expr),*) => {
        {
            const N: usize = count!($($bit),*);
            let mut bits: [QuantumBit; N] = [$($bit),*];
            QuantumRegister::from(&mut bits)
        }
    };
}

pub type QuantumState = ColumnVector<Complex<f64>>;
impl QuantumState {
    pub fn state_0() -> QuantumState {
        column_vector![complex!(1.0, 0.0), complex!(0.0, 0.0)]
    }

    pub fn state_1() -> QuantumState {
        column_vector![complex!(0.0, 0.0), complex!(1.0, 0.0)]
    }
}

fn identity_matrix<T: Float>(size: usize) -> Matrix<T> {
    let mut data = vec![T::zero(); size * size];
    for i in 0..size {
        data[i * size + i] = T::one();
    }
    Matrix::new(size, size, data)
}

#[derive(Clone)]
pub struct QuantumBit<'a> {
    state: QuantumState,
    name: &'a str,
}

#[derive(Clone)]
pub struct QuantumRegister<'a> {
    state_vector: QuantumState,
    name: &'a str,
    qubits: Vec<QuantumBit<'a>>,
}

#[derive(Clone)]
pub struct QuantumGate<'a> {
    pub name: &'a str,
    pub matrix: Matrix<Complex<f64>>,
    pub num_qubits: usize,
}

impl<'a> QuantumGate<'a> {
    pub fn new(name: &'a str, matrix: Matrix<Complex<f64>>, num_qubits: usize) -> Self {
        let expected_dim = 1 << num_qubits;
        assert_eq!(
            matrix.rows, expected_dim,
            "Gate matrix rows must be 2^num_qubits"
        );
        assert_eq!(
            matrix.cols, expected_dim,
            "Gate matrix cols must be 2^num_qubits"
        );
        QuantumGate {
            name,
            matrix,
            num_qubits,
        }
    }

    pub fn from_matrix(name: &'a str, matrix: Matrix<Complex<f64>>) -> Self {
        assert_eq!(matrix.rows, matrix.cols, "Gate matrix must be square");
        let dim = matrix.rows;
        assert!(
            dim > 0 && (dim & (dim - 1)) == 0,
            "Matrix dimension must be a power of 2"
        );
        let num_qubits = (dim as f64).log2() as usize;
        QuantumGate {
            name,
            matrix,
            num_qubits,
        }
    }
}

impl<'a> QuantumBit<'a> {
    pub fn new(name: &'a str, state: QuantumState) -> QuantumBit<'a> {
        QuantumBit { name, state }
    }

    pub fn get_state(&self) -> QuantumState {
        self.state.clone()
    }

    pub fn get_name(&self) -> &'a str {
        self.name
    }
}

impl<'a> QuantumRegister<'a> {
    pub fn new(name: &'a str, names: &[&'a str]) -> QuantumRegister<'a> {
        let mut bits: Vec<QuantumBit<'a>> = Vec::new();
        for i in 0..names.len() {
            bits.push(QuantumBit::new(names[i], QuantumState::state_0()))
        }

        QuantumRegister::from(name, &mut bits)
    }

    pub fn from(name: &'a str, bits: &mut [QuantumBit<'a>]) -> QuantumRegister<'a> {
        let mut register = QuantumRegister {
            name,
            qubits: bits.to_vec(),
            state_vector: ColumnVector::new(vec![]),
        };

        register.update();
        register
    }

    fn update(&mut self) {
        let matrices: Vec<Matrix<Complex<f64>>> = self
            .qubits
            .iter()
            .map(|qubit| qubit.state.to_matrix())
            .collect();
        let mut new_result = matrices[0].clone();
        for matrix in &matrices[1..] {
            new_result = new_result.kronecker(matrix);
        }

        self.state_vector = ColumnVector::from_matrix(&new_result);
    }

    pub fn get_bits(&self) -> Vec<QuantumBit<'_>> {
        self.qubits.clone()
    }

    pub fn get_state(&self) -> QuantumState {
        self.state_vector.clone()
    }

    pub fn get_name(&self) -> &'a str {
        self.name
    }

    pub fn num_qubits(&self) -> usize {
        self.qubits.len()
    }

    pub fn apply_gate(&mut self, gate: &QuantumGate, targets: &[usize]) {
        let n = self.num_qubits();

        assert_eq!(
            gate.num_qubits,
            targets.len(),
            "Number of target qubits must match gate's qubit count"
        );
        for &t in targets {
            assert!(
                t < n,
                "Target qubit index {} out of range for {}-qubit register",
                t,
                n
            );
        }

        let mut sorted_targets = targets.to_vec();
        sorted_targets.sort();
        for i in 1..sorted_targets.len() {
            assert_ne!(
                sorted_targets[i],
                sorted_targets[i - 1],
                "Duplicate target qubit indices are not allowed"
            );
        }

        let full_operator = self.build_full_operator(gate, targets);

        self.state_vector = self
            .state_vector
            .mul_matrix(&full_operator)
            .expect("Matrix multiplication failed during gate application");
    }

    fn build_full_operator(&self, gate: &QuantumGate, targets: &[usize]) -> Matrix<Complex<f64>> {
        let n = self.num_qubits();
        let g = gate.num_qubits;
        let dim = 1 << n;

        let mut contiguous = true;
        for i in 1..targets.len() {
            if targets[i] != targets[i - 1] + 1 {
                contiguous = false;
                break;
            }
        }

        if contiguous && g == n {
            return gate.matrix.clone();
        }

        if contiguous {
            return self.build_contiguous_operator(gate, targets[0]);
        }

        let mut result = Matrix::new(dim, dim, vec![complex!(0.0, 0.0); dim * dim]);

        for col in 0..dim {
            for row in 0..dim {
                let mut target_row_bits = 0usize;
                let mut target_col_bits = 0usize;

                for (i, &t) in targets.iter().enumerate() {
                    let qubit_pos = n - 1 - t;
                    if (row >> qubit_pos) & 1 == 1 {
                        target_row_bits |= 1 << (g - 1 - i);
                    }
                    if (col >> qubit_pos) & 1 == 1 {
                        target_col_bits |= 1 << (g - 1 - i);
                    }
                }

                let mut non_target_match = true;
                for q in 0..n {
                    if !targets.contains(&q) {
                        let qubit_pos = n - 1 - q;
                        if ((row >> qubit_pos) & 1) != ((col >> qubit_pos) & 1) {
                            non_target_match = false;
                            break;
                        }
                    }
                }

                if non_target_match {
                    result.set(row, col, gate.matrix.get(target_row_bits, target_col_bits));
                }
            }
        }

        result
    }

    fn build_contiguous_operator(
        &self,
        gate: &QuantumGate,
        start_idx: usize,
    ) -> Matrix<Complex<f64>> {
        let n = self.num_qubits();
        let g = gate.num_qubits;

        let mut result: Option<Matrix<Complex<f64>>> = None;

        for i in 0..n {
            let part: Matrix<Complex<f64>> = if i == start_idx {
                gate.matrix.clone()
            } else if i > start_idx && i < start_idx + g {
                continue;
            } else {
                identity_matrix(2)
            };

            result = Some(match result {
                None => part,
                Some(r) => r.kronecker(&part),
            });
        }

        result.unwrap_or_else(|| identity_matrix(1 << n))
    }

    pub fn apply_gates(&mut self, operations: &[(&QuantumGate, &[usize])]) {
        for (gate, targets) in operations {
            self.apply_gate(gate, targets);
        }
    }
}

impl<'a> ops::Index<usize> for QuantumRegister<'a> {
    type Output = QuantumBit<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.qubits[index]
    }
}

impl<'a> ops::IndexMut<usize> for QuantumRegister<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.qubits[index]
    }
}

impl<'a> fmt::Display for QuantumGate<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
