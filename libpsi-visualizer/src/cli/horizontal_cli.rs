use super::visualizer::Visualizer;
use core::fmt;
use libpsi_core::{GateOp, QuantumCircuit};

pub struct HorizontalRenderer<'a> {
    circuit: &'a QuantumCircuit,
}

impl<'a> HorizontalRenderer<'a> {
    pub fn new(circuit: &'a QuantumCircuit) -> Self {
        HorizontalRenderer { circuit }
    }
}

impl<'a> Visualizer for HorizontalRenderer<'a> {
    fn export(&self) -> String {
        format!("{}", self)
    }
}

impl<'a> fmt::Display for HorizontalRenderer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nq = self.circuit.num_qubits();
        let nc = self.circuit.num_classical();
        let ops = self.circuit.operations();

        let mut q_lines: Vec<String> = (0..nq).map(|i| format!("q{}: ", i)).collect();
        let mut c_lines: Vec<String> = (0..nc).map(|i| format!("c{}: ", i)).collect();

        let max_label = q_lines
            .iter()
            .chain(c_lines.iter())
            .map(|s| s.len())
            .max()
            .unwrap_or(3);

        for line in &mut q_lines {
            while line.len() < max_label {
                line.insert(0, ' ');
            }
        }
        for line in &mut c_lines {
            while line.len() < max_label {
                line.insert(0, ' ');
            }
        }
        let mut gap_line = " ".repeat(max_label);

        if ops.is_empty() {
            for line in &q_lines {
                writeln!(f, "{}───░", line)?;
            }
            if nc > 0 {
                writeln!(f, "{}   ░", gap_line)?;
                for line in &c_lines {
                    writeln!(f, "{}═══░", line)?;
                }
            }
            return Ok(());
        }

        for op in ops {
            let q_targets = op.quantum_targets();

            let min_q = q_targets.iter().min().copied().unwrap_or(0);
            let max_q = q_targets.iter().max().copied().unwrap_or(0);

            match op {
                GateOp::H(t) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str("─[H]─");
                        } else {
                            line.push_str("─────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("═════");
                    }
                    gap_line.push_str("     ");
                }
                GateOp::X(t) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str("─[X]─");
                        } else {
                            line.push_str("─────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("═════");
                    }
                    gap_line.push_str("     ");
                }
                GateOp::Y(t) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str("─[Y]─");
                        } else {
                            line.push_str("─────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("═════");
                    }
                    gap_line.push_str("     ");
                }
                GateOp::Z(t) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str("─[Z]─");
                        } else {
                            line.push_str("─────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("═════");
                    }
                    gap_line.push_str("     ");
                }
                GateOp::S(t) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str("─[S]─");
                        } else {
                            line.push_str("─────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("═════");
                    }
                    gap_line.push_str("     ");
                }
                GateOp::T(t) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str("─[T]─");
                        } else {
                            line.push_str("─────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("═════");
                    }
                    gap_line.push_str("     ");
                }
                GateOp::Sdg(t) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str("─[S†]─");
                        } else {
                            line.push_str("──────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("══════");
                    }
                    gap_line.push_str("      ");
                }
                GateOp::Tdg(t) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str("─[T†]─");
                        } else {
                            line.push_str("──────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("══════");
                    }
                    gap_line.push_str("      ");
                }
                GateOp::Sx(t) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str("─[√X]─");
                        } else {
                            line.push_str("──────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("══════");
                    }
                    gap_line.push_str("      ");
                }
                GateOp::Sxdg(t) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str("─[√X†]─");
                        } else {
                            line.push_str("───────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("═══════");
                    }
                    gap_line.push_str("       ");
                }
                GateOp::Rx(t, theta) => {
                    let label = format!("[Rx({:.2})]", theta);
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str(&format!("─{}─", label));
                        } else {
                            line.push_str(&format!("─{}─", "─".repeat(label.len())));
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str(&format!("═{}═", "═".repeat(label.len())));
                    }
                    gap_line.push_str(&format!(" {} ", " ".repeat(label.len())));
                }
                GateOp::Ry(t, theta) => {
                    let label = format!("[Ry({:.2})]", theta);
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str(&format!("─{}─", label));
                        } else {
                            line.push_str(&format!("─{}─", "─".repeat(label.len())));
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str(&format!("═{}═", "═".repeat(label.len())));
                    }
                    gap_line.push_str(&format!(" {} ", " ".repeat(label.len())));
                }
                GateOp::Rz(t, theta) => {
                    let label = format!("[Rz({:.2})]", theta);
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str(&format!("─{}─", label));
                        } else {
                            line.push_str(&format!("─{}─", "─".repeat(label.len())));
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str(&format!("═{}═", "═".repeat(label.len())));
                    }
                    gap_line.push_str(&format!(" {} ", " ".repeat(label.len())));
                }
                GateOp::P(t, theta) => {
                    let label = format!("[P({:.2})]", theta);
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str(&format!("─{}─", label));
                        } else {
                            line.push_str(&format!("─{}─", "─".repeat(label.len())));
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str(&format!("═{}═", "═".repeat(label.len())));
                    }
                    gap_line.push_str(&format!(" {} ", " ".repeat(label.len())));
                }
                GateOp::U1(t, lambda) => {
                    let label = format!("[U1({:.2})]", lambda);
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str(&format!("─{}─", label));
                        } else {
                            line.push_str(&format!("─{}─", "─".repeat(label.len())));
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str(&format!("═{}═", "═".repeat(label.len())));
                    }
                    gap_line.push_str(&format!(" {} ", " ".repeat(label.len())));
                }
                GateOp::U2(t, _, _) => {
                    let label = "[U2]";
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str(&format!("─{}─", label));
                        } else {
                            line.push_str(&format!("─{}─", "─".repeat(label.len())));
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str(&format!("═{}═", "═".repeat(label.len())));
                    }
                    gap_line.push_str(&format!(" {} ", " ".repeat(label.len())));
                }
                GateOp::U3(t, _, _, _) => {
                    let label = "[U3]";
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *t {
                            line.push_str(&format!("─{}─", label));
                        } else {
                            line.push_str(&format!("─{}─", "─".repeat(label.len())));
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str(&format!("═{}═", "═".repeat(label.len())));
                    }
                    gap_line.push_str(&format!(" {} ", " ".repeat(label.len())));
                }
                GateOp::CRx(c, t, theta) | GateOp::CRy(c, t, theta) | GateOp::CRz(c, t, theta) | GateOp::CP(c, t, theta) => {
                    let label = match op {
                        GateOp::CRx(_, _, _) => format!("[CRx({:.2})]", theta),
                        GateOp::CRy(_, _, _) => format!("[CRy({:.2})]", theta),
                        GateOp::CRz(_, _, _) => format!("[CRz({:.2})]", theta),
                        GateOp::CP(_, _, _) => format!("[CP({:.2})]", theta),
                        _ => unreachable!(),
                    };
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *c {
                            line.push_str(&format!("─{}─", "●".to_string() + &"─".repeat(label.len() - 1)));
                        } else if i == *t {
                            line.push_str(&format!("─{}─", label));
                        } else if i > min_q && i < max_q {
                            line.push_str(&format!("─{}─", "│".to_string() + &"─".repeat(label.len() - 1)));
                        } else {
                            line.push_str(&format!("─{}─", "─".repeat(label.len())));
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str(&format!("═{}═", "═".repeat(label.len())));
                    }
                    gap_line.push_str(&format!(" {} ", " ".repeat(label.len())));
                }
                GateOp::CNOT(c, t) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *c {
                            line.push_str("──●──");
                        } else if i == *t {
                            line.push_str("──⊕──");
                        } else if i > min_q && i < max_q {
                            line.push_str("──│──");
                        } else {
                            line.push_str("─────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("═════");
                    }
                    gap_line.push_str("     ");
                }
                GateOp::CZ(c, t) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *c || i == *t {
                            line.push_str("──●──");
                        } else if i > min_q && i < max_q {
                            line.push_str("──│──");
                        } else {
                            line.push_str("─────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("═════");
                    }
                    gap_line.push_str("     ");
                }
                GateOp::SWAP(a, b) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *a || i == *b {
                            line.push_str("──╳──");
                        } else if i > min_q && i < max_q {
                            line.push_str("──│──");
                        } else {
                            line.push_str("─────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("═════");
                    }
                    gap_line.push_str("     ");
                }
                GateOp::CCNOT(c1, c2, t) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *c1 || i == *c2 {
                            line.push_str("──●──");
                        } else if i == *t {
                            line.push_str("──⊕──");
                        } else if i > min_q && i < max_q {
                            line.push_str("──│──");
                        } else {
                            line.push_str("─────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("═════");
                    }
                    gap_line.push_str("     ");
                }
                GateOp::CSWAP(c, t1, t2) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *c {
                            line.push_str("──●──");
                        } else if i == *t1 || i == *t2 {
                            line.push_str("──╳──");
                        } else if i > min_q && i < max_q {
                            line.push_str("──│──");
                        } else {
                            line.push_str("─────");
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str("═════");
                    }
                    gap_line.push_str("     ");
                }
                GateOp::Measure(q, c) => {
                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if i == *q {
                            line.push_str("─[M]─");
                        } else if i > *q {
                            line.push_str("──║──");
                        } else {
                            line.push_str("─────");
                        }
                    }
                    for (i, line) in c_lines.iter_mut().enumerate() {
                        if i == *c {
                            line.push_str("══╩══");
                        } else if i < *c {
                            line.push_str("══║══");
                        } else {
                            line.push_str("═════");
                        }
                    }
                    gap_line.push_str("  ║  ");
                }
                GateOp::Custom(gate, targets) => {
                    let name = &gate.name;
                    let label = format!("[{}]", name);

                    for (i, line) in q_lines.iter_mut().enumerate() {
                        if targets.contains(&i) {
                            if i == targets[0] {
                                line.push_str(&format!("─{}─", label));
                            } else {
                                line.push_str(&format!("─{}─", "─".repeat(label.len())));
                            }
                        } else if i > min_q && i < max_q {
                            line.push_str(&format!(
                                "─{}─",
                                "│".to_string() + &"─".repeat(label.len() - 1)
                            ));
                        } else {
                            line.push_str(&format!("─{}─", "─".repeat(label.len())));
                        }
                    }
                    for line in c_lines.iter_mut() {
                        line.push_str(&format!("═{}═", "═".repeat(label.len())));
                    }
                    gap_line.push_str(&format!(" {} ", " ".repeat(label.len())));
                }
            }
        }

        for line in &q_lines {
            writeln!(f, "{}░", line)?;
        }
        if nc > 0 {
            writeln!(f, "{}░", gap_line)?;
            for line in &c_lines {
                writeln!(f, "{}░", line)?;
            }
        }

        Ok(())
    }
}
