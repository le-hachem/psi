use super::{GateOp, QuantumGate, QuantumRegister, QuantumState};
use crate::gates::{
    cp_matrix, crx_matrix, cry_matrix, crz_matrix, p_matrix, rx_matrix, ry_matrix,
    rz_matrix, u1_matrix, u2_matrix, u3_matrix, CNOT, CZ, FREDKIN, HADAMARD,
    PAULI_X, PAULI_Y, PAULI_Z, SDG_GATE, SWAP, SXDG_GATE, SX_GATE, S_GATE, TDG_GATE,
    TOFFOLI, T_GATE,
};
use crate::maths::vector::Vector;
use crate::{complex, Complex, Matrix};
use rayon::prelude::*;

/// Minimum number of qubits to enable parallelism (2^8 = 256 state vector elements)
const PARALLEL_THRESHOLD: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Runtime {
    #[default]
    BasicRT,
    BasicRTMT,
    WFEvolution,
    WFEvolutionMT,
    GPUAccelerated,
}

impl Runtime {
    pub fn compute(&self, num_qubits: usize, operations: &[GateOp]) -> QuantumState {
        match self {
            Runtime::BasicRT => Self::compute_basic(num_qubits, operations),
            Runtime::BasicRTMT => Self::compute_basic_mt(num_qubits, operations),
            Runtime::WFEvolution => {
                unimplemented!("WFEvolution (Schrödinger equation) runtime not yet implemented")
            }
            Runtime::WFEvolutionMT => {
                unimplemented!(
                    "WFEvolutionMT (multi-threaded Schrödinger) runtime not yet implemented"
                )
            }
            Runtime::GPUAccelerated => {
                unimplemented!("GPUAccelerated runtime not yet implemented")
            }
        }
    }

    fn compute_basic(num_qubits: usize, operations: &[GateOp]) -> QuantumState {
        let names: Vec<String> = (0..num_qubits).map(|i| format!("q{}", i)).collect();
        let leaked_names: &'static [String] = Box::leak(names.into_boxed_slice());
        let name_refs: Vec<&'static str> = leaked_names.iter().map(|s| s.as_str()).collect();

        let mut register = QuantumRegister::new(
            Box::leak(Box::new("circuit".to_string())).as_str(),
            &name_refs,
        );

        for op in operations {
            match op {
                // Clifford gates
                GateOp::H(t) => register.apply_gate(&HADAMARD, &[*t]),
                GateOp::X(t) => register.apply_gate(&PAULI_X, &[*t]),
                GateOp::Y(t) => register.apply_gate(&PAULI_Y, &[*t]),
                GateOp::Z(t) => register.apply_gate(&PAULI_Z, &[*t]),
                GateOp::S(t) => register.apply_gate(&S_GATE, &[*t]),
                GateOp::CNOT(c, t) => register.apply_gate(&CNOT, &[*c, *t]),
                GateOp::CZ(c, t) => register.apply_gate(&CZ, &[*c, *t]),
                GateOp::SWAP(a, b) => register.apply_gate(&SWAP, &[*a, *b]),
                GateOp::CCNOT(c1, c2, t) => register.apply_gate(&TOFFOLI, &[*c1, *c2, *t]),
                GateOp::CSWAP(c, t1, t2) => register.apply_gate(&FREDKIN, &[*c, *t1, *t2]),
                
                // Non-Clifford fixed gates
                GateOp::T(t) => register.apply_gate(&T_GATE, &[*t]),
                GateOp::Sdg(t) => register.apply_gate(&SDG_GATE, &[*t]),
                GateOp::Tdg(t) => register.apply_gate(&TDG_GATE, &[*t]),
                GateOp::Sx(t) => register.apply_gate(&SX_GATE, &[*t]),
                GateOp::Sxdg(t) => register.apply_gate(&SXDG_GATE, &[*t]),
                
                // Parametric single-qubit gates (non-Clifford for most angles)
                GateOp::Rx(t, theta) => {
                    let gate = QuantumGate {
                        name: "Rx",
                        matrix: rx_matrix(*theta),
                        num_qubits: 1,
                    };
                    register.apply_gate(&gate, &[*t]);
                }
                GateOp::Ry(t, theta) => {
                    let gate = QuantumGate {
                        name: "Ry",
                        matrix: ry_matrix(*theta),
                        num_qubits: 1,
                    };
                    register.apply_gate(&gate, &[*t]);
                }
                GateOp::Rz(t, theta) => {
                    let gate = QuantumGate {
                        name: "Rz",
                        matrix: rz_matrix(*theta),
                        num_qubits: 1,
                    };
                    register.apply_gate(&gate, &[*t]);
                }
                GateOp::P(t, theta) => {
                    let gate = QuantumGate {
                        name: "P",
                        matrix: p_matrix(*theta),
                        num_qubits: 1,
                    };
                    register.apply_gate(&gate, &[*t]);
                }
                GateOp::U1(t, lambda) => {
                    let gate = QuantumGate {
                        name: "U1",
                        matrix: u1_matrix(*lambda),
                        num_qubits: 1,
                    };
                    register.apply_gate(&gate, &[*t]);
                }
                GateOp::U2(t, phi, lambda) => {
                    let gate = QuantumGate {
                        name: "U2",
                        matrix: u2_matrix(*phi, *lambda),
                        num_qubits: 1,
                    };
                    register.apply_gate(&gate, &[*t]);
                }
                GateOp::U3(t, theta, phi, lambda) => {
                    let gate = QuantumGate {
                        name: "U3",
                        matrix: u3_matrix(*theta, *phi, *lambda),
                        num_qubits: 1,
                    };
                    register.apply_gate(&gate, &[*t]);
                }
                
                // Controlled parametric gates
                GateOp::CRx(c, t, theta) => {
                    let gate = QuantumGate {
                        name: "CRx",
                        matrix: crx_matrix(*theta),
                        num_qubits: 2,
                    };
                    register.apply_gate(&gate, &[*c, *t]);
                }
                GateOp::CRy(c, t, theta) => {
                    let gate = QuantumGate {
                        name: "CRy",
                        matrix: cry_matrix(*theta),
                        num_qubits: 2,
                    };
                    register.apply_gate(&gate, &[*c, *t]);
                }
                GateOp::CRz(c, t, theta) => {
                    let gate = QuantumGate {
                        name: "CRz",
                        matrix: crz_matrix(*theta),
                        num_qubits: 2,
                    };
                    register.apply_gate(&gate, &[*c, *t]);
                }
                GateOp::CP(c, t, theta) => {
                    let gate = QuantumGate {
                        name: "CP",
                        matrix: cp_matrix(*theta),
                        num_qubits: 2,
                    };
                    register.apply_gate(&gate, &[*c, *t]);
                }
                
                // Measurement and custom gates
                GateOp::Measure(_, _) => {}
                GateOp::Custom(gate, targets) => {
                    let quantum_gate = gate.to_quantum_gate();
                    register.apply_gate(&quantum_gate, targets);
                }
            }
        }

        register.get_state()
    }

    fn compute_basic_mt(num_qubits: usize, operations: &[GateOp]) -> QuantumState {
        // For small circuits, fall back to single-threaded (overhead not worth it)
        if num_qubits < PARALLEL_THRESHOLD {
            return Self::compute_basic(num_qubits, operations);
        }

        let dim = 1 << num_qubits;

        // Initialize state to |0...0⟩
        let mut state: Vec<Complex<f64>> = vec![complex!(0.0, 0.0); dim];
        state[0] = complex!(1.0, 0.0);

        for op in operations {
            let (gate_matrix, targets): (Matrix<Complex<f64>>, Vec<usize>) = match op {
                // Clifford gates
                GateOp::H(t) => (HADAMARD.matrix.clone(), vec![*t]),
                GateOp::X(t) => (PAULI_X.matrix.clone(), vec![*t]),
                GateOp::Y(t) => (PAULI_Y.matrix.clone(), vec![*t]),
                GateOp::Z(t) => (PAULI_Z.matrix.clone(), vec![*t]),
                GateOp::S(t) => (S_GATE.matrix.clone(), vec![*t]),
                GateOp::CNOT(c, t) => (CNOT.matrix.clone(), vec![*c, *t]),
                GateOp::CZ(c, t) => (CZ.matrix.clone(), vec![*c, *t]),
                GateOp::SWAP(a, b) => (SWAP.matrix.clone(), vec![*a, *b]),
                GateOp::CCNOT(c1, c2, t) => (TOFFOLI.matrix.clone(), vec![*c1, *c2, *t]),
                GateOp::CSWAP(c, t1, t2) => (FREDKIN.matrix.clone(), vec![*c, *t1, *t2]),
                
                // Non-Clifford fixed gates
                GateOp::T(t) => (T_GATE.matrix.clone(), vec![*t]),
                GateOp::Sdg(t) => (SDG_GATE.matrix.clone(), vec![*t]),
                GateOp::Tdg(t) => (TDG_GATE.matrix.clone(), vec![*t]),
                GateOp::Sx(t) => (SX_GATE.matrix.clone(), vec![*t]),
                GateOp::Sxdg(t) => (SXDG_GATE.matrix.clone(), vec![*t]),
                
                // Parametric single-qubit gates
                GateOp::Rx(t, theta) => (rx_matrix(*theta), vec![*t]),
                GateOp::Ry(t, theta) => (ry_matrix(*theta), vec![*t]),
                GateOp::Rz(t, theta) => (rz_matrix(*theta), vec![*t]),
                GateOp::P(t, theta) => (p_matrix(*theta), vec![*t]),
                GateOp::U1(t, lambda) => (u1_matrix(*lambda), vec![*t]),
                GateOp::U2(t, phi, lambda) => (u2_matrix(*phi, *lambda), vec![*t]),
                GateOp::U3(t, theta, phi, lambda) => (u3_matrix(*theta, *phi, *lambda), vec![*t]),
                
                // Controlled parametric gates
                GateOp::CRx(c, t, theta) => (crx_matrix(*theta), vec![*c, *t]),
                GateOp::CRy(c, t, theta) => (cry_matrix(*theta), vec![*c, *t]),
                GateOp::CRz(c, t, theta) => (crz_matrix(*theta), vec![*c, *t]),
                GateOp::CP(c, t, theta) => (cp_matrix(*theta), vec![*c, *t]),
                
                // Measurement (skip) and custom gates
                GateOp::Measure(_, _) => continue,
                GateOp::Custom(custom_gate, tgts) => {
                    let quantum_gate = custom_gate.to_quantum_gate();
                    state = apply_gate_parallel(&state, &quantum_gate.matrix, tgts, num_qubits);
                    continue;
                }
            };

            state = apply_gate_parallel(&state, &gate_matrix, &targets, num_qubits);
        }

        QuantumState::new(state)
    }
}

/// Apply a gate to the state vector in parallel using sparse application
/// This is O(2^n * 2^g) instead of O(2^2n) for full matrix multiplication
fn apply_gate_parallel(
    state: &[Complex<f64>],
    gate_matrix: &Matrix<Complex<f64>>,
    targets: &[usize],
    num_qubits: usize,
) -> Vec<Complex<f64>> {
    let dim = 1 << num_qubits;
    let g = targets.len();
    let gate_dim = 1 << g;

    // Convert target qubit indices to bit positions (from MSB)
    let target_bits: Vec<usize> = targets.iter().map(|&t| num_qubits - 1 - t).collect();

    // Create a mask for non-target qubits
    let mut non_target_mask: usize = (1 << num_qubits) - 1;
    for &pos in &target_bits {
        non_target_mask &= !(1 << pos);
    }

    // Parallel computation of new state
    let new_state: Vec<Complex<f64>> = (0..dim)
        .into_par_iter()
        .map(|i| {
            // Extract the target qubit bits from index i
            let mut target_idx = 0usize;
            for (k, &pos) in target_bits.iter().enumerate() {
                if (i >> pos) & 1 == 1 {
                    target_idx |= 1 << (g - 1 - k);
                }
            }

            // Compute the contribution to state[i]
            let mut sum = complex!(0.0, 0.0);

            // For each possible input state that could contribute
            for j in 0..gate_dim {
                // Get the gate matrix element
                let gate_elem = gate_matrix.data[target_idx * gate_dim + j];

                // Skip if zero (sparse optimization)
                if gate_elem.real.abs() < 1e-15 && gate_elem.imaginary.abs() < 1e-15 {
                    continue;
                }

                // Compute the source index by replacing target bits in i with bits from j
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
        .collect();

    new_state
}
