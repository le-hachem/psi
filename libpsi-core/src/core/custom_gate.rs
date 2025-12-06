use crate::{Complex, Matrix, QuantumGate};

#[derive(Clone)]
pub enum CustomGateDefinition {
    Matrix(Matrix<Complex<f64>>),
    Composite(Vec<(CompositeOp, Vec<usize>)>),
}

#[derive(Clone, Copy)]
pub enum CompositeOp {
    H,
    X,
    Y,
    Z,
    S,
    T,
    CNOT,
    CZ,
    SWAP,
    CCNOT,
    CSWAP,
}

#[derive(Clone)]
pub struct CustomGate {
    pub name: String,
    pub num_qubits: usize,
    pub definition: CustomGateDefinition,
}

impl CustomGate {
    pub fn from_matrix(name: &str, matrix: Matrix<Complex<f64>>) -> Self {
        let dim = matrix.rows;
        let num_qubits = (dim as f64).log2() as usize;
        assert_eq!(
            1 << num_qubits,
            dim,
            "Matrix dimension must be a power of 2"
        );
        assert_eq!(matrix.rows, matrix.cols, "Matrix must be square");

        CustomGate {
            name: String::from(name),
            num_qubits,
            definition: CustomGateDefinition::Matrix(matrix),
        }
    }

    pub fn from_composite(
        name: &str,
        num_qubits: usize,
        ops: Vec<(CompositeOp, Vec<usize>)>,
    ) -> Self {
        CustomGate {
            name: String::from(name),
            num_qubits,
            definition: CustomGateDefinition::Composite(ops),
        }
    }

    pub fn to_quantum_gate(&self) -> QuantumGate<'static> {
        match &self.definition {
            CustomGateDefinition::Matrix(matrix) => {
                let name: &'static str = Box::leak(self.name.clone().into_boxed_str());
                QuantumGate {
                    name,
                    matrix: matrix.clone(),
                    num_qubits: self.num_qubits,
                }
            }
            CustomGateDefinition::Composite(ops) => {
                let matrix = self.compute_composite_matrix(ops);
                let name: &'static str = Box::leak(self.name.clone().into_boxed_str());
                QuantumGate {
                    name,
                    matrix,
                    num_qubits: self.num_qubits,
                }
            }
        }
    }

    fn compute_composite_matrix(&self, ops: &[(CompositeOp, Vec<usize>)]) -> Matrix<Complex<f64>> {
        use crate::gates::*;
        use crate::Complex;

        let dim = 1 << self.num_qubits;
        let mut result = Matrix::new(dim, dim, vec![Complex::new(0.0, 0.0); dim * dim]);
        for i in 0..dim {
            result.data[i * dim + i] = Complex::new(1.0, 0.0);
        }

        for (op, targets) in ops {
            let gate: &QuantumGate = match op {
                CompositeOp::H => &HADAMARD,
                CompositeOp::X => &PAULI_X,
                CompositeOp::Y => &PAULI_Y,
                CompositeOp::Z => &PAULI_Z,
                CompositeOp::S => &S_GATE,
                CompositeOp::T => &T_GATE,
                CompositeOp::CNOT => &CNOT,
                CompositeOp::CZ => &CZ,
                CompositeOp::SWAP => &SWAP,
                CompositeOp::CCNOT => &TOFFOLI,
                CompositeOp::CSWAP => &FREDKIN,
            };

            let full_gate = build_full_operator(&gate.matrix, targets, self.num_qubits);
            result = matrix_multiply(&full_gate, &result);
        }

        result
    }
}

fn build_full_operator(
    gate_matrix: &Matrix<Complex<f64>>,
    targets: &[usize],
    total_qubits: usize,
) -> Matrix<Complex<f64>> {
    let dim = 1 << total_qubits;
    let gate_dim = gate_matrix.rows;
    let num_gate_qubits = targets.len();

    let mut result = Matrix::new(dim, dim, vec![Complex::new(0.0, 0.0); dim * dim]);

    for i in 0..dim {
        for j in 0..dim {
            let mut gate_i = 0usize;
            let mut gate_j = 0usize;
            let mut match_non_targets = true;

            for q in 0..total_qubits {
                let bit_i = (i >> (total_qubits - 1 - q)) & 1;
                let bit_j = (j >> (total_qubits - 1 - q)) & 1;

                if let Some(pos) = targets.iter().position(|&t| t == q) {
                    gate_i |= bit_i << (num_gate_qubits - 1 - pos);
                    gate_j |= bit_j << (num_gate_qubits - 1 - pos);
                } else if bit_i != bit_j {
                    match_non_targets = false;
                    break;
                }
            }

            if match_non_targets {
                result.data[i * dim + j] = gate_matrix.data[gate_i * gate_dim + gate_j];
            }
        }
    }

    result
}

fn matrix_multiply(a: &Matrix<Complex<f64>>, b: &Matrix<Complex<f64>>) -> Matrix<Complex<f64>> {
    let n = a.rows;
    let mut result = Matrix::new(n, n, vec![Complex::new(0.0, 0.0); n * n]);

    for i in 0..n {
        for j in 0..n {
            let mut sum = Complex::new(0.0, 0.0);
            for k in 0..n {
                sum = sum + a.data[i * n + k] * b.data[k * n + j];
            }
            result.data[i * n + j] = sum;
        }
    }

    result
}

pub struct CustomGateBuilder {
    name: String,
    num_qubits: usize,
    ops: Vec<(CompositeOp, Vec<usize>)>,
}

impl CustomGateBuilder {
    pub fn new(name: &str, num_qubits: usize) -> Self {
        CustomGateBuilder {
            name: String::from(name),
            num_qubits,
            ops: Vec::new(),
        }
    }

    pub fn h(mut self, target: usize) -> Self {
        self.ops.push((CompositeOp::H, vec![target]));
        self
    }

    pub fn x(mut self, target: usize) -> Self {
        self.ops.push((CompositeOp::X, vec![target]));
        self
    }

    pub fn y(mut self, target: usize) -> Self {
        self.ops.push((CompositeOp::Y, vec![target]));
        self
    }

    pub fn z(mut self, target: usize) -> Self {
        self.ops.push((CompositeOp::Z, vec![target]));
        self
    }

    pub fn s(mut self, target: usize) -> Self {
        self.ops.push((CompositeOp::S, vec![target]));
        self
    }

    pub fn t(mut self, target: usize) -> Self {
        self.ops.push((CompositeOp::T, vec![target]));
        self
    }

    pub fn cnot(mut self, control: usize, target: usize) -> Self {
        self.ops.push((CompositeOp::CNOT, vec![control, target]));
        self
    }

    pub fn cz(mut self, control: usize, target: usize) -> Self {
        self.ops.push((CompositeOp::CZ, vec![control, target]));
        self
    }

    pub fn swap(mut self, a: usize, b: usize) -> Self {
        self.ops.push((CompositeOp::SWAP, vec![a, b]));
        self
    }

    pub fn ccnot(mut self, c1: usize, c2: usize, target: usize) -> Self {
        self.ops.push((CompositeOp::CCNOT, vec![c1, c2, target]));
        self
    }

    pub fn cswap(mut self, control: usize, t1: usize, t2: usize) -> Self {
        self.ops.push((CompositeOp::CSWAP, vec![control, t1, t2]));
        self
    }

    pub fn build(self) -> CustomGate {
        CustomGate::from_composite(&self.name, self.num_qubits, self.ops)
    }
}
