use super::{QuantumGate, QuantumRegister, QuantumState};
use crate::{format_amplitude, format_probability, Vector};
use core::fmt;

#[derive(Clone)]
pub struct CircuitOperation<'a> {
    pub gate: &'a QuantumGate<'a>,
    pub targets: Vec<usize>,
}

impl<'a> CircuitOperation<'a> {
    pub fn new(gate: &'a QuantumGate<'a>, targets: Vec<usize>) -> Self {
        CircuitOperation { gate, targets }
    }
}

pub struct QuantumCircuit<'a> {
    register: QuantumRegister<'a>,
    operations: Vec<CircuitOperation<'a>>,
}

impl<'a> QuantumCircuit<'a> {
    pub fn new(num_qubits: usize) -> QuantumCircuit<'a> {
        let names: Vec<String> = (0..num_qubits).map(|i| format!("q{}", i)).collect();
        let leaked_names: &'a [String] = Box::leak(names.into_boxed_slice());
        let name_refs: Vec<&'a str> = leaked_names.iter().map(|s| s.as_str()).collect();

        QuantumCircuit {
            register: QuantumRegister::new(
                Box::leak(Box::new("circuit".to_string())).as_str(),
                &name_refs,
            ),
            operations: Vec::new(),
        }
    }

    pub fn from_register(register: QuantumRegister<'a>) -> QuantumCircuit<'a> {
        QuantumCircuit {
            register,
            operations: Vec::new(),
        }
    }

    pub fn num_qubits(&self) -> usize {
        self.register.num_qubits()
    }

    pub fn state(&self) -> QuantumState {
        self.register.get_state()
    }

    pub fn register(&self) -> &QuantumRegister<'a> {
        &self.register
    }

    pub fn operations(&self) -> &[CircuitOperation<'a>] {
        &self.operations
    }

    pub fn apply(&mut self, gate: &'a QuantumGate<'a>, targets: &[usize]) -> &mut Self {
        self.register.apply_gate(gate, targets);
        self.operations
            .push(CircuitOperation::new(gate, targets.to_vec()));
        self
    }

    pub fn h(&mut self, target: usize) -> &mut Self {
        use crate::gates::HADAMARD;
        self.register.apply_gate(&HADAMARD, &[target]);
        self.operations
            .push(CircuitOperation::new(&HADAMARD, vec![target]));
        self
    }

    pub fn x(&mut self, target: usize) -> &mut Self {
        use crate::gates::PAULI_X;
        self.register.apply_gate(&PAULI_X, &[target]);
        self.operations
            .push(CircuitOperation::new(&PAULI_X, vec![target]));
        self
    }

    pub fn y(&mut self, target: usize) -> &mut Self {
        use crate::gates::PAULI_Y;
        self.register.apply_gate(&PAULI_Y, &[target]);
        self.operations
            .push(CircuitOperation::new(&PAULI_Y, vec![target]));
        self
    }

    pub fn z(&mut self, target: usize) -> &mut Self {
        use crate::gates::PAULI_Z;
        self.register.apply_gate(&PAULI_Z, &[target]);
        self.operations
            .push(CircuitOperation::new(&PAULI_Z, vec![target]));
        self
    }

    pub fn s(&mut self, target: usize) -> &mut Self {
        use crate::gates::S_GATE;
        self.register.apply_gate(&S_GATE, &[target]);
        self.operations
            .push(CircuitOperation::new(&S_GATE, vec![target]));
        self
    }

    pub fn t(&mut self, target: usize) -> &mut Self {
        use crate::gates::T_GATE;
        self.register.apply_gate(&T_GATE, &[target]);
        self.operations
            .push(CircuitOperation::new(&T_GATE, vec![target]));
        self
    }

    pub fn cnot(&mut self, control: usize, target: usize) -> &mut Self {
        use crate::gates::CNOT;
        self.register.apply_gate(&CNOT, &[control, target]);
        self.operations
            .push(CircuitOperation::new(&CNOT, vec![control, target]));
        self
    }

    pub fn cx(&mut self, control: usize, target: usize) -> &mut Self {
        self.cnot(control, target)
    }

    pub fn cz(&mut self, control: usize, target: usize) -> &mut Self {
        use crate::gates::CZ;
        self.register.apply_gate(&CZ, &[control, target]);
        self.operations
            .push(CircuitOperation::new(&CZ, vec![control, target]));
        self
    }

    pub fn swap(&mut self, qubit1: usize, qubit2: usize) -> &mut Self {
        use crate::gates::SWAP;
        self.register.apply_gate(&SWAP, &[qubit1, qubit2]);
        self.operations
            .push(CircuitOperation::new(&SWAP, vec![qubit1, qubit2]));
        self
    }

    pub fn ccnot(&mut self, control1: usize, control2: usize, target: usize) -> &mut Self {
        use crate::gates::TOFFOLI;
        self.register
            .apply_gate(&TOFFOLI, &[control1, control2, target]);
        self.operations.push(CircuitOperation::new(
            &TOFFOLI,
            vec![control1, control2, target],
        ));
        self
    }

    pub fn toffoli(&mut self, control1: usize, control2: usize, target: usize) -> &mut Self {
        self.ccnot(control1, control2, target)
    }

    pub fn cswap(&mut self, control: usize, target1: usize, target2: usize) -> &mut Self {
        use crate::gates::FREDKIN;
        self.register
            .apply_gate(&FREDKIN, &[control, target1, target2]);
        self.operations.push(CircuitOperation::new(
            &FREDKIN,
            vec![control, target1, target2],
        ));
        self
    }

    pub fn fredkin(&mut self, control: usize, target1: usize, target2: usize) -> &mut Self {
        self.cswap(control, target1, target2)
    }

    pub fn reset(&mut self) -> &mut Self {
        let n = self.num_qubits();
        let names: Vec<String> = (0..n).map(|i| format!("q{}", i)).collect();
        let leaked_names: &'a [String] = Box::leak(names.into_boxed_slice());
        let name_refs: Vec<&'a str> = leaked_names.iter().map(|s| s.as_str()).collect();

        self.register = QuantumRegister::new(
            Box::leak(Box::new("circuit".to_string())).as_str(),
            &name_refs,
        );
        self.operations.clear();
        self
    }

    pub fn probability(&self, state_index: usize) -> f64 {
        let state = self.state();
        let amp = state.get(state_index);
        amp.norm2()
    }

    pub fn probabilities(&self) -> Vec<f64> {
        let state = self.state();
        let n = 1 << self.num_qubits();
        (0..n).map(|i| state.get(i).norm2()).collect()
    }
}

impl<'a> fmt::Display for QuantumCircuit<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "QuantumCircuit ({} qubits)", self.num_qubits())?;
        writeln!(f, "Operations:")?;
        for (i, op) in self.operations.iter().enumerate() {
            writeln!(f, "  {}: {} on {:?}", i, op.gate.name, op.targets)?;
        }
        writeln!(f, "State:")?;
        let state = self.state();
        let n = 1 << self.num_qubits();
        for i in 0..n {
            let amp = state.get(i);
            if amp.real.abs() > 1e-10 || amp.imaginary.abs() > 1e-10 {
                let basis: String = format!("{:0width$b}", i, width = self.num_qubits());
                writeln!(f, "  |{}⟩: {}", basis, format_amplitude(&amp))?;
            }
        }
        Ok(())
    }
}

impl<'a> QuantumCircuit<'a> {
    pub fn print_probabilities(&self) {
        let probs = self.probabilities();
        let n = self.num_qubits();
        println!("Probabilities:");
        for (i, p) in probs.iter().enumerate() {
            if *p > 1e-10 {
                let basis: String = format!("{:0width$b}", i, width = n);
                println!("  |{}⟩: {}", basis, format_probability(*p));
            }
        }
    }
}
