use crate::{complex, Complex, Matrix};

#[derive(Clone, Debug)]
pub struct KrausOperator {
    pub matrix: Matrix<Complex<f64>>,
    pub name: String,
}

impl KrausOperator {
    pub fn new(name: &str, matrix: Matrix<Complex<f64>>) -> Self {
        Self {
            matrix,
            name: name.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NoiseChannel {
    pub name: String,
    pub operators: Vec<KrausOperator>,
    pub num_qubits: usize,
}

impl NoiseChannel {
    pub fn new(name: &str, operators: Vec<KrausOperator>, num_qubits: usize) -> Self {
        Self {
            name: name.to_string(),
            operators,
            num_qubits,
        }
    }

    pub fn depolarising(p: f64) -> Self {
        let sqrt_1_p = (1.0 - p).sqrt();
        let sqrt_p3 = (p / 3.0).sqrt();

        let k0 = Matrix::new(
            2,
            2,
            vec![
                complex!(sqrt_1_p, 0.0),
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
                complex!(sqrt_1_p, 0.0),
            ],
        );

        let k1 = Matrix::new(
            2,
            2,
            vec![
                complex!(0.0, 0.0),
                complex!(sqrt_p3, 0.0),
                complex!(sqrt_p3, 0.0),
                complex!(0.0, 0.0),
            ],
        );

        let k2 = Matrix::new(
            2,
            2,
            vec![
                complex!(0.0, 0.0),
                complex!(0.0, -sqrt_p3),
                complex!(0.0, sqrt_p3),
                complex!(0.0, 0.0),
            ],
        );

        let k3 = Matrix::new(
            2,
            2,
            vec![
                complex!(sqrt_p3, 0.0),
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
                complex!(-sqrt_p3, 0.0),
            ],
        );

        Self::new(
            "Depolarising",
            vec![
                KrausOperator::new("K0", k0),
                KrausOperator::new("K1(X)", k1),
                KrausOperator::new("K2(Y)", k2),
                KrausOperator::new("K3(Z)", k3),
            ],
            1,
        )
    }

    pub fn amplitude_damping(gamma: f64) -> Self {
        let sqrt_gamma = gamma.sqrt();
        let sqrt_1_gamma = (1.0 - gamma).sqrt();

        let k0 = Matrix::new(
            2,
            2,
            vec![
                complex!(1.0, 0.0),
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
                complex!(sqrt_1_gamma, 0.0),
            ],
        );

        let k1 = Matrix::new(
            2,
            2,
            vec![
                complex!(0.0, 0.0),
                complex!(sqrt_gamma, 0.0),
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
            ],
        );

        Self::new(
            "AmplitudeDamping",
            vec![
                KrausOperator::new("K0", k0),
                KrausOperator::new("K1", k1),
            ],
            1,
        )
    }

    pub fn phase_damping(gamma: f64) -> Self {
        let sqrt_gamma = gamma.sqrt();
        let sqrt_1_gamma = (1.0 - gamma).sqrt();

        let k0 = Matrix::new(
            2,
            2,
            vec![
                complex!(1.0, 0.0),
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
                complex!(sqrt_1_gamma, 0.0),
            ],
        );

        let k1 = Matrix::new(
            2,
            2,
            vec![
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
                complex!(sqrt_gamma, 0.0),
            ],
        );

        Self::new(
            "PhaseDamping",
            vec![
                KrausOperator::new("K0", k0),
                KrausOperator::new("K1", k1),
            ],
            1,
        )
    }

    pub fn bit_flip(p: f64) -> Self {
        let sqrt_1_p = (1.0 - p).sqrt();
        let sqrt_p = p.sqrt();

        let k0 = Matrix::new(
            2,
            2,
            vec![
                complex!(sqrt_1_p, 0.0),
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
                complex!(sqrt_1_p, 0.0),
            ],
        );

        let k1 = Matrix::new(
            2,
            2,
            vec![
                complex!(0.0, 0.0),
                complex!(sqrt_p, 0.0),
                complex!(sqrt_p, 0.0),
                complex!(0.0, 0.0),
            ],
        );

        Self::new(
            "BitFlip",
            vec![
                KrausOperator::new("K0(I)", k0),
                KrausOperator::new("K1(X)", k1),
            ],
            1,
        )
    }

    pub fn phase_flip(p: f64) -> Self {
        let sqrt_1_p = (1.0 - p).sqrt();
        let sqrt_p = p.sqrt();

        let k0 = Matrix::new(
            2,
            2,
            vec![
                complex!(sqrt_1_p, 0.0),
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
                complex!(sqrt_1_p, 0.0),
            ],
        );

        let k1 = Matrix::new(
            2,
            2,
            vec![
                complex!(sqrt_p, 0.0),
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
                complex!(-sqrt_p, 0.0),
            ],
        );

        Self::new(
            "PhaseFlip",
            vec![
                KrausOperator::new("K0(I)", k0),
                KrausOperator::new("K1(Z)", k1),
            ],
            1,
        )
    }

    pub fn bit_phase_flip(p: f64) -> Self {
        let sqrt_1_p = (1.0 - p).sqrt();
        let sqrt_p = p.sqrt();

        let k0 = Matrix::new(
            2,
            2,
            vec![
                complex!(sqrt_1_p, 0.0),
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
                complex!(sqrt_1_p, 0.0),
            ],
        );

        let k1 = Matrix::new(
            2,
            2,
            vec![
                complex!(0.0, 0.0),
                complex!(0.0, -sqrt_p),
                complex!(0.0, sqrt_p),
                complex!(0.0, 0.0),
            ],
        );

        Self::new(
            "BitPhaseFlip",
            vec![
                KrausOperator::new("K0(I)", k0),
                KrausOperator::new("K1(Y)", k1),
            ],
            1,
        )
    }

    pub fn generalised_amplitude_damping(p: f64, gamma: f64) -> Self {
        let sqrt_p = p.sqrt();
        let sqrt_1_p = (1.0 - p).sqrt();
        let sqrt_gamma = gamma.sqrt();
        let sqrt_1_gamma = (1.0 - gamma).sqrt();

        let k0 = Matrix::new(
            2,
            2,
            vec![
                complex!(sqrt_p, 0.0),
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
                complex!(sqrt_p * sqrt_1_gamma, 0.0),
            ],
        );

        let k1 = Matrix::new(
            2,
            2,
            vec![
                complex!(0.0, 0.0),
                complex!(sqrt_p * sqrt_gamma, 0.0),
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
            ],
        );

        let k2 = Matrix::new(
            2,
            2,
            vec![
                complex!(sqrt_1_p * sqrt_1_gamma, 0.0),
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
                complex!(sqrt_1_p, 0.0),
            ],
        );

        let k3 = Matrix::new(
            2,
            2,
            vec![
                complex!(0.0, 0.0),
                complex!(0.0, 0.0),
                complex!(sqrt_1_p * sqrt_gamma, 0.0),
                complex!(0.0, 0.0),
            ],
        );

        Self::new(
            "GeneralisedAmplitudeDamping",
            vec![
                KrausOperator::new("K0", k0),
                KrausOperator::new("K1", k1),
                KrausOperator::new("K2", k2),
                KrausOperator::new("K3", k3),
            ],
            1,
        )
    }
}

#[derive(Clone)]
pub struct DensityMatrix {
    pub data: Vec<Complex<f64>>,
    pub dim: usize,
    pub num_qubits: usize,
}

impl DensityMatrix {
    pub fn new(num_qubits: usize) -> Self {
        let dim = 1 << num_qubits;
        let mut data = vec![complex!(0.0, 0.0); dim * dim];
        data[0] = complex!(1.0, 0.0);
        Self {
            data,
            dim,
            num_qubits,
        }
    }

    pub fn from_state_vector(state: &[Complex<f64>]) -> Self {
        let dim = state.len();
        let num_qubits = (dim as f64).log2() as usize;
        let mut data = vec![complex!(0.0, 0.0); dim * dim];

        for i in 0..dim {
            for j in 0..dim {
                data[i * dim + j] = state[i] * state[j].get_conjugate();
            }
        }

        Self {
            data,
            dim,
            num_qubits,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Complex<f64> {
        self.data[row * self.dim + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: Complex<f64>) {
        self.data[row * self.dim + col] = value;
    }

    pub fn trace(&self) -> Complex<f64> {
        let mut sum = complex!(0.0, 0.0);
        for i in 0..self.dim {
            sum = sum + self.get(i, i);
        }
        sum
    }

    pub fn purity(&self) -> f64 {
        let mut sum = complex!(0.0, 0.0);
        for i in 0..self.dim {
            for j in 0..self.dim {
                let rho_ij = self.get(i, j);
                let rho_ji = self.get(j, i);
                sum = sum + rho_ij * rho_ji;
            }
        }
        sum.real
    }

    pub fn is_pure(&self, tolerance: f64) -> bool {
        (self.purity() - 1.0).abs() < tolerance
    }

    pub fn probabilities(&self) -> Vec<f64> {
        (0..self.dim).map(|i| self.get(i, i).real).collect()
    }

    pub fn apply_unitary(&mut self, gate: &Matrix<Complex<f64>>, targets: &[usize]) {
        let g = targets.len();
        let gate_dim = 1 << g;

        let target_bits: Vec<usize> = targets
            .iter()
            .map(|&t| self.num_qubits - 1 - t)
            .collect();

        let mut non_target_mask: usize = (1 << self.num_qubits) - 1;
        for &pos in &target_bits {
            non_target_mask &= !(1 << pos);
        }

        let mut new_data = vec![complex!(0.0, 0.0); self.dim * self.dim];

        for i in 0..self.dim {
            for j in 0..self.dim {
                let mut sum = complex!(0.0, 0.0);

                for k in 0..gate_dim {
                    for l in 0..gate_dim {
                        let mut src_i = i & non_target_mask;
                        let mut src_j = j & non_target_mask;

                        for (idx, &pos) in target_bits.iter().enumerate() {
                            if (k >> (g - 1 - idx)) & 1 == 1 {
                                src_i |= 1 << pos;
                            }
                            if (l >> (g - 1 - idx)) & 1 == 1 {
                                src_j |= 1 << pos;
                            }
                        }

                        let mut tgt_i = 0usize;
                        let mut tgt_j = 0usize;
                        for (idx, &pos) in target_bits.iter().enumerate() {
                            if (i >> pos) & 1 == 1 {
                                tgt_i |= 1 << (g - 1 - idx);
                            }
                            if (j >> pos) & 1 == 1 {
                                tgt_j |= 1 << (g - 1 - idx);
                            }
                        }

                        let u_ik = gate.data[tgt_i * gate_dim + k];
                        let u_jl_dag = gate.data[tgt_j * gate_dim + l].get_conjugate();
                        let rho_kl = self.get(src_i, src_j);

                        sum = sum + u_ik * rho_kl * u_jl_dag;
                    }
                }

                new_data[i * self.dim + j] = sum;
            }
        }

        self.data = new_data;
    }

    pub fn apply_noise_channel(&mut self, channel: &NoiseChannel, target: usize) {
        if channel.num_qubits != 1 {
            panic!("Only single-qubit noise channels are currently supported");
        }

        let target_bit = self.num_qubits - 1 - target;
        let mut new_data = vec![complex!(0.0, 0.0); self.dim * self.dim];

        for kraus in &channel.operators {
            let k = &kraus.matrix;

            for i in 0..self.dim {
                for j in 0..self.dim {
                    let i_target = (i >> target_bit) & 1;
                    let j_target = (j >> target_bit) & 1;

                    for ki in 0..2 {
                        for kj in 0..2 {
                            let src_i = (i & !(1 << target_bit)) | (ki << target_bit);
                            let src_j = (j & !(1 << target_bit)) | (kj << target_bit);

                            let k_elem = k.data[i_target * 2 + ki];
                            let k_dag_elem = k.data[j_target * 2 + kj].get_conjugate();
                            let rho_elem = self.get(src_i, src_j);

                            new_data[i * self.dim + j] =
                                new_data[i * self.dim + j] + k_elem * rho_elem * k_dag_elem;
                        }
                    }
                }
            }
        }

        self.data = new_data;
    }

    pub fn measure_probability(&self, qubit: usize, outcome: usize) -> f64 {
        let target_bit = self.num_qubits - 1 - qubit;
        let mut prob = 0.0;

        for i in 0..self.dim {
            if (i >> target_bit) & 1 == outcome {
                prob += self.get(i, i).real;
            }
        }

        prob
    }

    pub fn fidelity_with_pure_state(&self, state: &[Complex<f64>]) -> f64 {
        let mut sum = complex!(0.0, 0.0);

        for i in 0..self.dim {
            for j in 0..self.dim {
                sum = sum + state[i].get_conjugate() * self.get(i, j) * state[j];
            }
        }

        sum.real
    }
}

impl std::fmt::Display for DensityMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DensityMatrix ({} qubits, {}×{}):", self.num_qubits, self.dim, self.dim)?;
        writeln!(f, "  Trace: {:.6}", self.trace().real)?;
        writeln!(f, "  Purity: {:.6}", self.purity())?;
        writeln!(f, "  Pure: {}", self.is_pure(1e-10))?;
        writeln!(f, "  Probabilities: {:?}", self.probabilities())?;
        Ok(())
    }
}

impl std::fmt::Debug for DensityMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DensityMatrix {}×{}:", self.dim, self.dim)?;
        for i in 0..self.dim {
            write!(f, "  [")?;
            for j in 0..self.dim {
                let val = self.get(i, j);
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:.4}+{:.4}i", val.real, val.imaginary)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

