use super::{QuantumRegister, QuantumState};
use crate::{format_amplitude, format_probability, Vector};
use core::fmt;

#[derive(Clone, Copy)]
pub enum GateOp {
    H(usize),
    X(usize),
    Y(usize),
    Z(usize),
    S(usize),
    T(usize),
    CNOT(usize, usize),
    CZ(usize, usize),
    SWAP(usize, usize),
    CCNOT(usize, usize, usize),
    CSWAP(usize, usize, usize),
    Measure(usize, usize),
}

impl GateOp {
    pub fn name(&self) -> &'static str {
        match self {
            GateOp::H(_) => "H",
            GateOp::X(_) => "X",
            GateOp::Y(_) => "Y",
            GateOp::Z(_) => "Z",
            GateOp::S(_) => "S",
            GateOp::T(_) => "T",
            GateOp::CNOT(_, _) => "CNOT",
            GateOp::CZ(_, _) => "CZ",
            GateOp::SWAP(_, _) => "SWAP",
            GateOp::CCNOT(_, _, _) => "CCNOT",
            GateOp::CSWAP(_, _, _) => "CSWAP",
            GateOp::Measure(_, _) => "M",
        }
    }

    pub fn quantum_targets(&self) -> Vec<usize> {
        match self {
            GateOp::H(t) | GateOp::X(t) | GateOp::Y(t) | GateOp::Z(t) | GateOp::S(t) | GateOp::T(t) => vec![*t],
            GateOp::CNOT(c, t) | GateOp::CZ(c, t) | GateOp::SWAP(c, t) => vec![*c, *t],
            GateOp::CCNOT(c1, c2, t) | GateOp::CSWAP(c1, c2, t) => vec![*c1, *c2, *t],
            GateOp::Measure(q, _) => vec![*q],
        }
    }

    pub fn classical_targets(&self) -> Vec<usize> {
        match self {
            GateOp::Measure(_, c) => vec![*c],
            _ => vec![],
        }
    }

    pub fn is_measurement(&self) -> bool {
        matches!(self, GateOp::Measure(_, _))
    }
}

pub struct QuantumCircuit {
    num_qubits: usize,
    num_classical: usize,
    operations: Vec<GateOp>,
    computed_state: Option<QuantumState>,
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> QuantumCircuit {
        QuantumCircuit {
            num_qubits,
            num_classical: 0,
            operations: Vec::new(),
            computed_state: None,
        }
    }

    pub fn with_classical(num_qubits: usize, num_classical: usize) -> QuantumCircuit {
        QuantumCircuit {
            num_qubits,
            num_classical,
            operations: Vec::new(),
            computed_state: None,
        }
    }

    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn num_classical(&self) -> usize {
        self.num_classical
    }

    pub fn operations(&self) -> &[GateOp] {
        &self.operations
    }

    pub fn is_computed(&self) -> bool {
        self.computed_state.is_some()
    }

    pub fn compute(&mut self) -> &QuantumState {
        if self.computed_state.is_some() {
            return self.computed_state.as_ref().unwrap();
        }

        let names: Vec<String> = (0..self.num_qubits).map(|i| format!("q{}", i)).collect();
        let leaked_names: &'static [String] = Box::leak(names.into_boxed_slice());
        let name_refs: Vec<&'static str> = leaked_names.iter().map(|s| s.as_str()).collect();

        let mut register = QuantumRegister::new(
            Box::leak(Box::new("circuit".to_string())).as_str(),
            &name_refs,
        );

        use crate::gates::*;
        for op in &self.operations {
            match op {
                GateOp::H(t) => register.apply_gate(&HADAMARD, &[*t]),
                GateOp::X(t) => register.apply_gate(&PAULI_X, &[*t]),
                GateOp::Y(t) => register.apply_gate(&PAULI_Y, &[*t]),
                GateOp::Z(t) => register.apply_gate(&PAULI_Z, &[*t]),
                GateOp::S(t) => register.apply_gate(&S_GATE, &[*t]),
                GateOp::T(t) => register.apply_gate(&T_GATE, &[*t]),
                GateOp::CNOT(c, t) => register.apply_gate(&CNOT, &[*c, *t]),
                GateOp::CZ(c, t) => register.apply_gate(&CZ, &[*c, *t]),
                GateOp::SWAP(a, b) => register.apply_gate(&SWAP, &[*a, *b]),
                GateOp::CCNOT(c1, c2, t) => register.apply_gate(&TOFFOLI, &[*c1, *c2, *t]),
                GateOp::CSWAP(c, t1, t2) => register.apply_gate(&FREDKIN, &[*c, *t1, *t2]),
                GateOp::Measure(_, _) => {}
            }
        }

        self.computed_state = Some(register.get_state());
        self.computed_state.as_ref().unwrap()
    }

    pub fn state(&mut self) -> &QuantumState {
        self.compute()
    }

    pub fn h(&mut self, target: usize) -> &mut Self {
        self.operations.push(GateOp::H(target));
        self.computed_state = None;
        self
    }

    pub fn x(&mut self, target: usize) -> &mut Self {
        self.operations.push(GateOp::X(target));
        self.computed_state = None;
        self
    }

    pub fn y(&mut self, target: usize) -> &mut Self {
        self.operations.push(GateOp::Y(target));
        self.computed_state = None;
        self
    }

    pub fn z(&mut self, target: usize) -> &mut Self {
        self.operations.push(GateOp::Z(target));
        self.computed_state = None;
        self
    }

    pub fn s(&mut self, target: usize) -> &mut Self {
        self.operations.push(GateOp::S(target));
        self.computed_state = None;
        self
    }

    pub fn t(&mut self, target: usize) -> &mut Self {
        self.operations.push(GateOp::T(target));
        self.computed_state = None;
        self
    }

    pub fn cnot(&mut self, control: usize, target: usize) -> &mut Self {
        self.operations.push(GateOp::CNOT(control, target));
        self.computed_state = None;
        self
    }

    pub fn cx(&mut self, control: usize, target: usize) -> &mut Self {
        self.cnot(control, target)
    }

    pub fn cz(&mut self, control: usize, target: usize) -> &mut Self {
        self.operations.push(GateOp::CZ(control, target));
        self.computed_state = None;
        self
    }

    pub fn swap(&mut self, qubit1: usize, qubit2: usize) -> &mut Self {
        self.operations.push(GateOp::SWAP(qubit1, qubit2));
        self.computed_state = None;
        self
    }

    pub fn ccnot(&mut self, control1: usize, control2: usize, target: usize) -> &mut Self {
        self.operations.push(GateOp::CCNOT(control1, control2, target));
        self.computed_state = None;
        self
    }

    pub fn toffoli(&mut self, control1: usize, control2: usize, target: usize) -> &mut Self {
        self.ccnot(control1, control2, target)
    }

    pub fn cswap(&mut self, control: usize, target1: usize, target2: usize) -> &mut Self {
        self.operations.push(GateOp::CSWAP(control, target1, target2));
        self.computed_state = None;
        self
    }

    pub fn fredkin(&mut self, control: usize, target1: usize, target2: usize) -> &mut Self {
        self.cswap(control, target1, target2)
    }

    pub fn measure(&mut self, qubit: usize, classical: usize) -> &mut Self {
        if classical >= self.num_classical {
            self.num_classical = classical + 1;
        }
        self.operations.push(GateOp::Measure(qubit, classical));
        self
    }

    pub fn measure_all(&mut self) -> &mut Self {
        for i in 0..self.num_qubits {
            self.measure(i, i);
        }
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.operations.clear();
        self.computed_state = None;
        self
    }

    pub fn probability(&mut self, state_index: usize) -> f64 {
        self.compute();
        let state = self.computed_state.as_ref().unwrap();
        let amp = state.get(state_index);
        amp.norm2()
    }

    pub fn probabilities(&mut self) -> Vec<f64> {
        self.compute();
        let n = 1 << self.num_qubits;
        let state = self.computed_state.as_ref().unwrap();
        (0..n).map(|i| state.get(i).norm2()).collect()
    }

    pub fn print_probabilities(&mut self) {
        let probs = self.probabilities();
        let n = self.num_qubits;
        println!("Probabilities:");
        for (i, p) in probs.iter().enumerate() {
            if *p > 1e-10 {
                let basis: String = format!("{:0width$b}", i, width = n);
                println!("  |{}⟩: {}", basis, format_probability(*p));
            }
        }
    }
}

impl fmt::Display for QuantumCircuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "QuantumCircuit ({} qubits, {} classical)", self.num_qubits, self.num_classical)?;
        writeln!(f, "Operations:")?;
        for (i, op) in self.operations.iter().enumerate() {
            match op {
                GateOp::Measure(q, c) => writeln!(f, "  {}: {} q{} → c{}", i, op.name(), q, c)?,
                _ => writeln!(f, "  {}: {} on {:?}", i, op.name(), op.quantum_targets())?,
            }
        }
        if let Some(state) = &self.computed_state {
            writeln!(f, "State:")?;
            let n = 1 << self.num_qubits;
            for i in 0..n {
                let amp = state.get(i);
                if amp.real.abs() > 1e-10 || amp.imaginary.abs() > 1e-10 {
                    let basis: String = format!("{:0width$b}", i, width = self.num_qubits);
                    writeln!(f, "  |{}⟩: {}", basis, format_amplitude(&amp))?;
                }
            }
        } else {
            writeln!(f, "State: (not computed)")?;
        }
        Ok(())
    }
}
