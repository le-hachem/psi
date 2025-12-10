use super::{CustomGate, QuantumState, Runtime, RuntimeConfig};
use crate::{format_amplitude, format_probability, Vector};
use core::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub enum GateOp {
    H(usize),
    X(usize),
    Y(usize),
    Z(usize),
    S(usize),
    T(usize),
    Sdg(usize),
    Tdg(usize),
    Sx(usize),
    Sxdg(usize),
    Rx(usize, f64),
    Ry(usize, f64),
    Rz(usize, f64),
    P(usize, f64),
    U1(usize, f64),
    U2(usize, f64, f64),
    U3(usize, f64, f64, f64),
    CNOT(usize, usize),
    CZ(usize, usize),
    SWAP(usize, usize),
    CRx(usize, usize, f64),
    CRy(usize, usize, f64),
    CRz(usize, usize, f64),
    CP(usize, usize, f64),
    CCNOT(usize, usize, usize),
    CSWAP(usize, usize, usize),
    Measure(usize, usize),
    Custom(Arc<CustomGate>, Vec<usize>),
}

impl GateOp {
    pub fn name(&self) -> &str {
        match self {
            GateOp::H(_) => "H",
            GateOp::X(_) => "X",
            GateOp::Y(_) => "Y",
            GateOp::Z(_) => "Z",
            GateOp::S(_) => "S",
            GateOp::T(_) => "T",
            GateOp::Sdg(_) => "S†",
            GateOp::Tdg(_) => "T†",
            GateOp::Sx(_) => "√X",
            GateOp::Sxdg(_) => "√X†",
            GateOp::Rx(_, _) => "Rx",
            GateOp::Ry(_, _) => "Ry",
            GateOp::Rz(_, _) => "Rz",
            GateOp::P(_, _) => "P",
            GateOp::U1(_, _) => "U1",
            GateOp::U2(_, _, _) => "U2",
            GateOp::U3(_, _, _, _) => "U3",
            GateOp::CRx(_, _, _) => "CRx",
            GateOp::CRy(_, _, _) => "CRy",
            GateOp::CRz(_, _, _) => "CRz",
            GateOp::CP(_, _, _) => "CP",
            GateOp::CNOT(_, _) => "CNOT",
            GateOp::CZ(_, _) => "CZ",
            GateOp::SWAP(_, _) => "SWAP",
            GateOp::CCNOT(_, _, _) => "CCNOT",
            GateOp::CSWAP(_, _, _) => "CSWAP",
            GateOp::Measure(_, _) => "M",
            GateOp::Custom(gate, _) => &gate.name,
        }
    }

    pub fn quantum_targets(&self) -> Vec<usize> {
        match self {
            GateOp::H(t)
            | GateOp::X(t)
            | GateOp::Y(t)
            | GateOp::Z(t)
            | GateOp::S(t)
            | GateOp::T(t)
            | GateOp::Sdg(t)
            | GateOp::Tdg(t)
            | GateOp::Sx(t)
            | GateOp::Sxdg(t)
            | GateOp::Rx(t, _)
            | GateOp::Ry(t, _)
            | GateOp::Rz(t, _)
            | GateOp::P(t, _)
            | GateOp::U1(t, _)
            | GateOp::U2(t, _, _)
            | GateOp::U3(t, _, _, _) => vec![*t],
            GateOp::CNOT(c, t)
            | GateOp::CZ(c, t)
            | GateOp::SWAP(c, t)
            | GateOp::CRx(c, t, _)
            | GateOp::CRy(c, t, _)
            | GateOp::CRz(c, t, _)
            | GateOp::CP(c, t, _) => vec![*c, *t],
            GateOp::CCNOT(c1, c2, t) | GateOp::CSWAP(c1, c2, t) => vec![*c1, *c2, *t],
            GateOp::Measure(q, _) => vec![*q],
            GateOp::Custom(_, targets) => targets.clone(),
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

    pub fn is_custom(&self) -> bool {
        matches!(self, GateOp::Custom(_, _))
    }

    pub fn is_non_clifford(&self) -> bool {
        matches!(
            self,
            GateOp::T(_)
                | GateOp::Tdg(_)
                | GateOp::Sx(_)
                | GateOp::Sxdg(_)
                | GateOp::Rx(_, _)
                | GateOp::Ry(_, _)
                | GateOp::Rz(_, _)
                | GateOp::P(_, _)
                | GateOp::U1(_, _)
                | GateOp::U2(_, _, _)
                | GateOp::U3(_, _, _, _)
                | GateOp::CRx(_, _, _)
                | GateOp::CRy(_, _, _)
                | GateOp::CRz(_, _, _)
                | GateOp::CP(_, _, _)
        )
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
        self.compute_with(Runtime::default())
    }

    pub fn compute_with(&mut self, runtime: Runtime) -> &QuantumState {
        if self.computed_state.is_some() {
            return self.computed_state.as_ref().unwrap();
        }

        self.computed_state = Some(runtime.compute(self.num_qubits, &self.operations));
        self.computed_state.as_ref().unwrap()
    }

    pub fn compute_with_config(&mut self, config: RuntimeConfig) -> &QuantumState {
        if self.computed_state.is_some() {
            return self.computed_state.as_ref().unwrap();
        }

        self.computed_state = Some(config.compute(self.num_qubits, &self.operations));
        self.computed_state.as_ref().unwrap()
    }

    pub fn state(&mut self) -> &QuantumState {
        self.compute()
    }

    pub fn state_with(&mut self, runtime: Runtime) -> &QuantumState {
        self.compute_with(runtime)
    }

    pub fn state_with_config(&mut self, config: RuntimeConfig) -> &QuantumState {
        self.compute_with_config(config)
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

    pub fn sdg(&mut self, target: usize) -> &mut Self {
        self.operations.push(GateOp::Sdg(target));
        self.computed_state = None;
        self
    }

    pub fn tdg(&mut self, target: usize) -> &mut Self {
        self.operations.push(GateOp::Tdg(target));
        self.computed_state = None;
        self
    }

    pub fn sx(&mut self, target: usize) -> &mut Self {
        self.operations.push(GateOp::Sx(target));
        self.computed_state = None;
        self
    }

    pub fn sxdg(&mut self, target: usize) -> &mut Self {
        self.operations.push(GateOp::Sxdg(target));
        self.computed_state = None;
        self
    }

    pub fn rx(&mut self, target: usize, theta: f64) -> &mut Self {
        self.operations.push(GateOp::Rx(target, theta));
        self.computed_state = None;
        self
    }

    pub fn ry(&mut self, target: usize, theta: f64) -> &mut Self {
        self.operations.push(GateOp::Ry(target, theta));
        self.computed_state = None;
        self
    }

    pub fn rz(&mut self, target: usize, theta: f64) -> &mut Self {
        self.operations.push(GateOp::Rz(target, theta));
        self.computed_state = None;
        self
    }

    pub fn p(&mut self, target: usize, theta: f64) -> &mut Self {
        self.operations.push(GateOp::P(target, theta));
        self.computed_state = None;
        self
    }

    pub fn u1(&mut self, target: usize, lambda: f64) -> &mut Self {
        self.operations.push(GateOp::U1(target, lambda));
        self.computed_state = None;
        self
    }

    pub fn u2(&mut self, target: usize, phi: f64, lambda: f64) -> &mut Self {
        self.operations.push(GateOp::U2(target, phi, lambda));
        self.computed_state = None;
        self
    }

    pub fn u3(&mut self, target: usize, theta: f64, phi: f64, lambda: f64) -> &mut Self {
        self.operations.push(GateOp::U3(target, theta, phi, lambda));
        self.computed_state = None;
        self
    }

    pub fn crx(&mut self, control: usize, target: usize, theta: f64) -> &mut Self {
        self.operations.push(GateOp::CRx(control, target, theta));
        self.computed_state = None;
        self
    }

    pub fn cry(&mut self, control: usize, target: usize, theta: f64) -> &mut Self {
        self.operations.push(GateOp::CRy(control, target, theta));
        self.computed_state = None;
        self
    }

    pub fn crz(&mut self, control: usize, target: usize, theta: f64) -> &mut Self {
        self.operations.push(GateOp::CRz(control, target, theta));
        self.computed_state = None;
        self
    }

    pub fn cp(&mut self, control: usize, target: usize, theta: f64) -> &mut Self {
        self.operations.push(GateOp::CP(control, target, theta));
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
        self.operations
            .push(GateOp::CCNOT(control1, control2, target));
        self.computed_state = None;
        self
    }

    pub fn toffoli(&mut self, control1: usize, control2: usize, target: usize) -> &mut Self {
        self.ccnot(control1, control2, target)
    }

    pub fn cswap(&mut self, control: usize, target1: usize, target2: usize) -> &mut Self {
        self.operations
            .push(GateOp::CSWAP(control, target1, target2));
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

    pub fn custom(&mut self, gate: &Arc<CustomGate>, targets: &[usize]) -> &mut Self {
        self.operations
            .push(GateOp::Custom(Arc::clone(gate), targets.to_vec()));
        self.computed_state = None;
        self
    }

    pub fn apply_custom(&mut self, gate: CustomGate, targets: &[usize]) -> &mut Self {
        self.operations
            .push(GateOp::Custom(Arc::new(gate), targets.to_vec()));
        self.computed_state = None;
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
        writeln!(
            f,
            "QuantumCircuit ({} qubits, {} classical)",
            self.num_qubits, self.num_classical
        )?;
        writeln!(f, "Operations:")?;
        for (i, op) in self.operations.iter().enumerate() {
            match op {
                GateOp::Measure(q, c) => writeln!(f, "  {}: {} q{} → c{}", i, op.name(), q, c)?,
                GateOp::Custom(gate, targets) => {
                    writeln!(f, "  {}: [{}] on {:?}", i, gate.name, targets)?
                }
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
