use crate::{complex, Complex, Matrix};
use rayon::prelude::*;

#[derive(Clone)]
pub struct Kernel {
    pub matrix: Matrix<Complex<f64>>,
    pub targets: Vec<usize>,
    pub name: String,
}

impl Kernel {
    pub fn new(name: &str, matrix: Matrix<Complex<f64>>, targets: Vec<usize>) -> Self {
        Self {
            matrix,
            targets,
            name: name.to_string(),
        }
    }

    pub fn num_qubits(&self) -> usize {
        self.targets.len()
    }

    pub fn can_fuse_with(&self, other: &Kernel) -> bool {
        if self.targets.len() != 1 || other.targets.len() != 1 {
            return false;
        }
        self.targets[0] == other.targets[0]
    }

    pub fn fuse(&self, other: &Kernel) -> Option<Kernel> {
        if !self.can_fuse_with(other) {
            return None;
        }
        let fused_matrix = other.matrix.dot(&self.matrix)?;
        Some(Kernel {
            matrix: fused_matrix,
            targets: self.targets.clone(),
            name: format!("{}+{}", self.name, other.name),
        })
    }
}

pub struct KernelBatch {
    kernels: Vec<Kernel>,
    num_qubits: usize,
}

impl KernelBatch {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            kernels: Vec::new(),
            num_qubits,
        }
    }

    pub fn add(&mut self, kernel: Kernel) {
        self.kernels.push(kernel);
    }

    pub fn len(&self) -> usize {
        self.kernels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.kernels.is_empty()
    }

    pub fn kernels(&self) -> &[Kernel] {
        &self.kernels
    }

    pub fn optimize(&mut self) {
        if self.kernels.len() < 2 {
            return;
        }

        let mut optimized: Vec<Kernel> = Vec::with_capacity(self.kernels.len());
        let mut i = 0;

        while i < self.kernels.len() {
            let current = &self.kernels[i];

            if i + 1 < self.kernels.len() {
                let next = &self.kernels[i + 1];
                if let Some(fused) = current.fuse(next) {
                    optimized.push(fused);
                    i += 2;
                    continue;
                }
            }

            optimized.push(current.clone());
            i += 1;
        }

        self.kernels = optimized;
    }

    pub fn execute(&self, state: &mut Vec<Complex<f64>>) {
        for kernel in &self.kernels {
            *state = apply_kernel(state, kernel, self.num_qubits);
        }
    }

    pub fn execute_parallel(&self, state: &mut Vec<Complex<f64>>) {
        for kernel in &self.kernels {
            *state = apply_kernel_parallel(state, kernel, self.num_qubits);
        }
    }
}

fn apply_kernel(state: &[Complex<f64>], kernel: &Kernel, num_qubits: usize) -> Vec<Complex<f64>> {
    let dim = 1 << num_qubits;
    let g = kernel.targets.len();
    let gate_dim = 1 << g;

    let target_bits: Vec<usize> = kernel.targets.iter().map(|&t| num_qubits - 1 - t).collect();

    let mut non_target_mask: usize = (1 << num_qubits) - 1;
    for &pos in &target_bits {
        non_target_mask &= !(1 << pos);
    }

    let mut new_state = vec![complex!(0.0, 0.0); dim];

    for i in 0..dim {
        let mut target_idx = 0usize;
        for (k, &pos) in target_bits.iter().enumerate() {
            if (i >> pos) & 1 == 1 {
                target_idx |= 1 << (g - 1 - k);
            }
        }

        let mut sum = complex!(0.0, 0.0);

        for j in 0..gate_dim {
            let gate_elem = kernel.matrix.data[target_idx * gate_dim + j];

            if gate_elem.real.abs() < 1e-15 && gate_elem.imaginary.abs() < 1e-15 {
                continue;
            }

            let mut source_idx = i & non_target_mask;
            for (k, &pos) in target_bits.iter().enumerate() {
                if (j >> (g - 1 - k)) & 1 == 1 {
                    source_idx |= 1 << pos;
                }
            }

            sum = sum + gate_elem * state[source_idx];
        }

        new_state[i] = sum;
    }

    new_state
}

fn apply_kernel_parallel(
    state: &[Complex<f64>],
    kernel: &Kernel,
    num_qubits: usize,
) -> Vec<Complex<f64>> {
    let dim = 1 << num_qubits;
    let g = kernel.targets.len();
    let gate_dim = 1 << g;

    let target_bits: Vec<usize> = kernel.targets.iter().map(|&t| num_qubits - 1 - t).collect();

    let mut non_target_mask: usize = (1 << num_qubits) - 1;
    for &pos in &target_bits {
        non_target_mask &= !(1 << pos);
    }

    (0..dim)
        .into_par_iter()
        .map(|i| {
            let mut target_idx = 0usize;
            for (k, &pos) in target_bits.iter().enumerate() {
                if (i >> pos) & 1 == 1 {
                    target_idx |= 1 << (g - 1 - k);
                }
            }

            let mut sum = complex!(0.0, 0.0);

            for j in 0..gate_dim {
                let gate_elem = kernel.matrix.data[target_idx * gate_dim + j];

                if gate_elem.real.abs() < 1e-15 && gate_elem.imaginary.abs() < 1e-15 {
                    continue;
                }

                let mut source_idx = i & non_target_mask;
                for (k, &pos) in target_bits.iter().enumerate() {
                    if (j >> (g - 1 - k)) & 1 == 1 {
                        source_idx |= 1 << pos;
                    }
                }

                sum = sum + gate_elem * state[source_idx];
            }

            sum
        })
        .collect()
}

pub struct KernelBuilder {
    num_qubits: usize,
}

impl KernelBuilder {
    pub fn new(num_qubits: usize) -> Self {
        Self { num_qubits }
    }

    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }
}
