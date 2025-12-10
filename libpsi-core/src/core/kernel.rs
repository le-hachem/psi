use crate::maths::simd::{
    apply_single_qubit_gate_simd, apply_single_qubit_gate_simd_parallel, SimdCapability,
};
use crate::{complex, Complex, Matrix};
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GateType {
    Diagonal,
    NonDiagonal,
    Controlled,
}

#[derive(Clone)]
pub struct Kernel {
    pub matrix: Matrix<Complex<f64>>,
    pub targets: Vec<usize>,
    pub name: String,
    pub gate_type: GateType,
}

impl Kernel {
    pub fn new(name: &str, matrix: Matrix<Complex<f64>>, targets: Vec<usize>) -> Self {
        let gate_type = Self::detect_gate_type(name, &matrix);
        Self {
            matrix,
            targets,
            name: name.to_string(),
            gate_type,
        }
    }

    fn detect_gate_type(name: &str, matrix: &Matrix<Complex<f64>>) -> GateType {
        let diagonal_gates = [
            "Z", "S", "T", "Sdg", "Tdg", "Rz", "P", "U1", "CZ", "CP", "CRz",
        ];
        if diagonal_gates.iter().any(|&g| name.starts_with(g)) {
            return GateType::Diagonal;
        }

        let controlled_gates = [
            "CNOT", "CZ", "SWAP", "CRx", "CRy", "CRz", "CP", "CCNOT", "CSWAP",
        ];
        if controlled_gates.iter().any(|&g| name.starts_with(g)) {
            return GateType::Controlled;
        }

        if matrix.rows == 2 && matrix.cols == 2 {
            let is_diag = matrix.data[1].real.abs() < 1e-10
                && matrix.data[1].imaginary.abs() < 1e-10
                && matrix.data[2].real.abs() < 1e-10
                && matrix.data[2].imaginary.abs() < 1e-10;
            if is_diag {
                return GateType::Diagonal;
            }
        }

        GateType::NonDiagonal
    }

    pub fn num_qubits(&self) -> usize {
        self.targets.len()
    }

    pub fn target_set(&self) -> HashSet<usize> {
        self.targets.iter().cloned().collect()
    }

    pub fn shares_qubits(&self, other: &Kernel) -> bool {
        self.targets.iter().any(|t| other.targets.contains(t))
    }

    pub fn commutes_with(&self, other: &Kernel) -> bool {
        if !self.shares_qubits(other) {
            return true;
        }

        if self.gate_type == GateType::Diagonal && other.gate_type == GateType::Diagonal {
            if self.targets == other.targets {
                return true;
            }
        }

        false
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
        let new_type =
            if self.gate_type == GateType::Diagonal && other.gate_type == GateType::Diagonal {
                GateType::Diagonal
            } else {
                GateType::NonDiagonal
            };
        Some(Kernel {
            matrix: fused_matrix,
            targets: self.targets.clone(),
            name: format!("{}+{}", self.name, other.name),
            gate_type: new_type,
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

    pub fn execute_simd(&self, state: &mut Vec<Complex<f64>>) {
        for kernel in &self.kernels {
            if kernel.targets.len() == 1 {
                let gate = matrix_to_2x2(&kernel.matrix);
                apply_single_qubit_gate_simd(state, &gate, kernel.targets[0], self.num_qubits);
            } else {
                *state = apply_kernel(state, kernel, self.num_qubits);
            }
        }
    }

    pub fn execute_simd_parallel(&self, state: &mut Vec<Complex<f64>>) {
        for kernel in &self.kernels {
            if kernel.targets.len() == 1 && self.num_qubits >= 10 {
                let gate = matrix_to_2x2(&kernel.matrix);
                apply_single_qubit_gate_simd_parallel(
                    state,
                    &gate,
                    kernel.targets[0],
                    self.num_qubits,
                );
            } else if kernel.targets.len() == 1 {
                let gate = matrix_to_2x2(&kernel.matrix);
                apply_single_qubit_gate_simd(state, &gate, kernel.targets[0], self.num_qubits);
            } else {
                *state = apply_kernel_parallel(state, kernel, self.num_qubits);
            }
        }
    }

    pub fn simd_capability(&self) -> SimdCapability {
        SimdCapability::detect()
    }
}

fn matrix_to_2x2(matrix: &Matrix<Complex<f64>>) -> [[Complex<f64>; 2]; 2] {
    [
        [matrix.data[0], matrix.data[1]],
        [matrix.data[2], matrix.data[3]],
    ]
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

#[derive(Clone)]
pub struct ExecutionLayer {
    pub kernels: Vec<Kernel>,
}

impl ExecutionLayer {
    pub fn new() -> Self {
        Self {
            kernels: Vec::new(),
        }
    }

    pub fn can_add(&self, kernel: &Kernel) -> bool {
        !self.kernels.iter().any(|k| k.shares_qubits(kernel))
    }

    pub fn add(&mut self, kernel: Kernel) {
        self.kernels.push(kernel);
    }

    pub fn affected_qubits(&self) -> HashSet<usize> {
        self.kernels
            .iter()
            .flat_map(|k| k.targets.iter().cloned())
            .collect()
    }
}

impl Default for ExecutionLayer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StructureAwareKernelBatch {
    kernels: Vec<Kernel>,
    layers: Vec<ExecutionLayer>,
    num_qubits: usize,
    optimised: bool,
}

impl StructureAwareKernelBatch {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            kernels: Vec::new(),
            layers: Vec::new(),
            num_qubits,
            optimised: false,
        }
    }

    pub fn add(&mut self, kernel: Kernel) {
        self.kernels.push(kernel);
        self.optimised = false;
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

    pub fn layers(&self) -> &[ExecutionLayer] {
        &self.layers
    }

    pub fn num_layers(&self) -> usize {
        self.layers.len()
    }

    pub fn optimise(&mut self) {
        if self.optimised || self.kernels.len() < 2 {
            return;
        }

        self.reorder_commuting_gates();
        self.multi_pass_fusion();
        self.build_execution_layers();
        self.optimised = true;
    }

    fn reorder_commuting_gates(&mut self) {
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 100;

        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;

            for i in 0..self.kernels.len().saturating_sub(1) {
                let current = &self.kernels[i];
                let next = &self.kernels[i + 1];

                if current.targets.len() == 1
                    && next.targets.len() == 1
                    && current.targets[0] != next.targets[0]
                    && current.commutes_with(next)
                {
                    for j in (i + 2)..self.kernels.len() {
                        let candidate = &self.kernels[j];

                        if candidate.targets.len() == 1
                            && candidate.targets[0] == current.targets[0]
                        {
                            let can_move = (i + 1..j).all(|k| {
                                let between = &self.kernels[k];
                                !between.shares_qubits(current) || current.commutes_with(between)
                            });

                            if can_move && current.can_fuse_with(candidate) {
                                let kernel_to_move = self.kernels.remove(j);
                                self.kernels.insert(i + 1, kernel_to_move);
                                changed = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn multi_pass_fusion(&mut self) {
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 50;

        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;

            let mut new_kernels: Vec<Kernel> = Vec::with_capacity(self.kernels.len());
            let mut i = 0;

            while i < self.kernels.len() {
                if i + 1 < self.kernels.len() {
                    let current = &self.kernels[i];
                    let next = &self.kernels[i + 1];

                    if let Some(fused) = current.fuse(next) {
                        new_kernels.push(fused);
                        i += 2;
                        changed = true;
                        continue;
                    }
                }

                new_kernels.push(self.kernels[i].clone());
                i += 1;
            }

            self.kernels = new_kernels;
        }
    }

    fn build_execution_layers(&mut self) {
        self.layers.clear();

        for kernel in &self.kernels {
            let mut placed = false;

            for layer in &mut self.layers {
                if layer.can_add(kernel) {
                    layer.add(kernel.clone());
                    placed = true;
                    break;
                }
            }

            if !placed {
                let mut new_layer = ExecutionLayer::new();
                new_layer.add(kernel.clone());
                self.layers.push(new_layer);
            }
        }
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

    pub fn execute_layered(&self, state: &mut Vec<Complex<f64>>) {
        for layer in &self.layers {
            for kernel in &layer.kernels {
                *state = apply_kernel(state, kernel, self.num_qubits);
            }
        }
    }

    pub fn execute_layered_parallel(&self, state: &mut Vec<Complex<f64>>) {
        for layer in &self.layers {
            for kernel in &layer.kernels {
                *state = apply_kernel_parallel(state, kernel, self.num_qubits);
            }
        }
    }

    pub fn execute_simd(&self, state: &mut Vec<Complex<f64>>) {
        for kernel in &self.kernels {
            if kernel.targets.len() == 1 {
                let gate = matrix_to_2x2(&kernel.matrix);
                apply_single_qubit_gate_simd(state, &gate, kernel.targets[0], self.num_qubits);
            } else {
                *state = apply_kernel(state, kernel, self.num_qubits);
            }
        }
    }

    pub fn execute_simd_parallel(&self, state: &mut Vec<Complex<f64>>) {
        for kernel in &self.kernels {
            if kernel.targets.len() == 1 && self.num_qubits >= 10 {
                let gate = matrix_to_2x2(&kernel.matrix);
                apply_single_qubit_gate_simd_parallel(
                    state,
                    &gate,
                    kernel.targets[0],
                    self.num_qubits,
                );
            } else if kernel.targets.len() == 1 {
                let gate = matrix_to_2x2(&kernel.matrix);
                apply_single_qubit_gate_simd(state, &gate, kernel.targets[0], self.num_qubits);
            } else {
                *state = apply_kernel_parallel(state, kernel, self.num_qubits);
            }
        }
    }

    pub fn stats(&self) -> KernelStats {
        let single_qubit = self.kernels.iter().filter(|k| k.targets.len() == 1).count();
        let two_qubit = self.kernels.iter().filter(|k| k.targets.len() == 2).count();
        let multi_qubit = self.kernels.iter().filter(|k| k.targets.len() > 2).count();
        let diagonal = self
            .kernels
            .iter()
            .filter(|k| k.gate_type == GateType::Diagonal)
            .count();

        KernelStats {
            total_kernels: self.kernels.len(),
            single_qubit,
            two_qubit,
            multi_qubit,
            diagonal,
            execution_layers: self.layers.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KernelStats {
    pub total_kernels: usize,
    pub single_qubit: usize,
    pub two_qubit: usize,
    pub multi_qubit: usize,
    pub diagonal: usize,
    pub execution_layers: usize,
}

impl std::fmt::Display for KernelStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Kernels: {} (1q: {}, 2q: {}, 3q+: {}, diag: {}), Layers: {}",
            self.total_kernels,
            self.single_qubit,
            self.two_qubit,
            self.multi_qubit,
            self.diagonal,
            self.execution_layers
        )
    }
}
