use super::visualizer::Visualizer;
use core::fmt;
use libpsi_core::{GateOp, QuantumCircuit};

pub struct VerticalRenderer<'a> {
    circuit: &'a QuantumCircuit,
}

impl<'a> VerticalRenderer<'a> {
    pub fn new(circuit: &'a QuantumCircuit) -> Self {
        VerticalRenderer { circuit }
    }

    fn gate_label(op: &GateOp) -> String {
        match op {
            GateOp::H(_) => "[H]".to_string(),
            GateOp::X(_) => "[X]".to_string(),
            GateOp::Y(_) => "[Y]".to_string(),
            GateOp::Z(_) => "[Z]".to_string(),
            GateOp::S(_) => "[S]".to_string(),
            GateOp::T(_) => "[T]".to_string(),
            GateOp::Sdg(_) => "[S†]".to_string(),
            GateOp::Tdg(_) => "[T†]".to_string(),
            GateOp::Sx(_) => "[√X]".to_string(),
            GateOp::Sxdg(_) => "[√X†]".to_string(),
            GateOp::Rx(_, theta) => format!("[Rx({:.2})]", theta),
            GateOp::Ry(_, theta) => format!("[Ry({:.2})]", theta),
            GateOp::Rz(_, theta) => format!("[Rz({:.2})]", theta),
            GateOp::P(_, theta) => format!("[P({:.2})]", theta),
            GateOp::U1(_, lambda) => format!("[U1({:.2})]", lambda),
            GateOp::U2(_, _, _) => "[U2]".to_string(),
            GateOp::U3(_, _, _, _) => "[U3]".to_string(),
            GateOp::CRx(_, _, _) => "[CRx]".to_string(),
            GateOp::CRy(_, _, _) => "[CRy]".to_string(),
            GateOp::CRz(_, _, _) => "[CRz]".to_string(),
            GateOp::CP(_, _, _) => "[CP]".to_string(),
            GateOp::CNOT(_, _) => "●".to_string(),
            GateOp::CZ(_, _) => "●".to_string(),
            GateOp::SWAP(_, _) => "╳".to_string(),
            GateOp::CCNOT(_, _, _) => "●".to_string(),
            GateOp::CSWAP(_, _, _) => "●".to_string(),
            GateOp::Measure(_, _) => "[M]".to_string(),
            GateOp::Custom(gate, _) => format!("[{}]", gate.name),
        }
    }

    fn calculate_col_width(&self) -> usize {
        let min_width = 3;
        let mut max_label_len = min_width;

        for op in self.circuit.operations() {
            let label = Self::gate_label(op);
            let char_count: usize = label.chars().count();
            if char_count > max_label_len {
                max_label_len = char_count;
            }
        }

        let width = max_label_len + 2;
        if width % 2 == 0 {
            width + 1
        } else {
            width
        }
    }
}

impl<'a> Visualizer for VerticalRenderer<'a> {
    fn export(&self) -> String {
        format!("{}", self)
    }
}

impl<'a> fmt::Display for VerticalRenderer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nq = self.circuit.num_qubits();
        let nc = self.circuit.num_classical();
        let ops = self.circuit.operations();

        let col_width = self.calculate_col_width();
        let gap_width = 3;

        let q_header: String = (0..nq)
            .map(|i| format!("{:^width$}", format!("q{}", i), width = col_width))
            .collect::<Vec<_>>()
            .join(" ");

        let c_header: String = (0..nc)
            .map(|i| format!("{:^width$}", format!("c{}", i), width = col_width))
            .collect::<Vec<_>>()
            .join(" ");

        if nc > 0 {
            writeln!(f, "{}{}{}", q_header, " ".repeat(gap_width), c_header)?;
        } else {
            writeln!(f, "{}", q_header)?;
        }

        let q_wires: String = (0..nq)
            .map(|_| format!("{:^width$}", "│", width = col_width))
            .collect::<Vec<_>>()
            .join(" ");

        let c_wires: String = (0..nc)
            .map(|_| format!("{:^width$}", "║", width = col_width))
            .collect::<Vec<_>>()
            .join(" ");

        let full_wires = if nc > 0 {
            format!("{}{}{}", q_wires, " ".repeat(gap_width), c_wires)
        } else {
            q_wires.clone()
        };

        if ops.is_empty() {
            writeln!(f, "{}", full_wires)?;
            return Ok(());
        }

        let q_total = nq * col_width + (nq - 1);
        let c_total = if nc > 0 { nc * col_width + (nc - 1) } else { 0 };
        let total_width = q_total + gap_width + c_total;

        for op in ops {
            writeln!(f, "{}", full_wires)?;

            let q_targets = op.quantum_targets();
            let min_q = q_targets.iter().min().copied().unwrap_or(0);
            let max_q = q_targets.iter().max().copied().unwrap_or(0);

            let label = Self::gate_label(op);

            match op {
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
                | GateOp::U3(t, _, _, _) => {
                    let mut line: Vec<char> = vec![' '; total_width];

                    for i in 0..nq {
                        let col_start = i * (col_width + 1);
                        let center = col_start + col_width / 2;
                        if i == *t {
                            let label_start = col_start + (col_width - label.chars().count()) / 2;
                            for (j, ch) in label.chars().enumerate() {
                                line[label_start + j] = ch;
                            }
                        } else {
                            line[center] = '│';
                        }
                    }

                    for i in 0..nc {
                        let center = q_total + gap_width + i * (col_width + 1) + col_width / 2;
                        line[center] = '║';
                    }

                    let gate_line: String = line.into_iter().collect();
                    writeln!(f, "{}", gate_line)?;
                }
                GateOp::CNOT(c, t) | GateOp::CZ(c, t) | GateOp::SWAP(c, t)
                | GateOp::CRx(c, t, _) | GateOp::CRy(c, t, _) | GateOp::CRz(c, t, _) | GateOp::CP(c, t, _) => {
                    let (sym1, sym2) = match op {
                        GateOp::CNOT(_, _) => ('●', '⊕'),
                        GateOp::CZ(_, _) => ('●', '●'),
                        GateOp::SWAP(_, _) => ('╳', '╳'),
                        GateOp::CRx(_, _, _) | GateOp::CRy(_, _, _) | GateOp::CRz(_, _, _) | GateOp::CP(_, _, _) => ('●', '□'),
                        _ => unreachable!(),
                    };

                    let mut line: Vec<char> = vec![' '; total_width];

                    for i in 0..nq {
                        let col_start = i * (col_width + 1);
                        let center = col_start + col_width / 2;
                        if i < min_q || i > max_q {
                            line[center] = '│';
                        } else if i == *c {
                            line[center] = sym1;
                        } else if i == *t {
                            // For controlled parametric gates, show the gate label on target
                            if matches!(op, GateOp::CRx(_, _, _) | GateOp::CRy(_, _, _) | GateOp::CRz(_, _, _) | GateOp::CP(_, _, _)) {
                                let label_start = col_start + (col_width - label.chars().count()) / 2;
                                for (j, ch) in label.chars().enumerate() {
                                    if label_start + j < line.len() {
                                        line[label_start + j] = ch;
                                    }
                                }
                            } else {
                                line[center] = sym2;
                            }
                        }
                    }

                    let min_center = min_q * (col_width + 1) + col_width / 2;
                    let max_center = max_q * (col_width + 1) + col_width / 2;
                    for pos in (min_center + 1)..max_center {
                        if line[pos] == ' ' {
                            line[pos] = '─';
                        }
                    }

                    for i in 0..nc {
                        let center = q_total + gap_width + i * (col_width + 1) + col_width / 2;
                        line[center] = '║';
                    }

                    let gate_line: String = line.into_iter().collect();
                    writeln!(f, "{}", gate_line)?;
                }
                GateOp::CCNOT(c1, c2, t) | GateOp::CSWAP(c1, c2, t) => {
                    let (sym_c, sym_t) = match op {
                        GateOp::CCNOT(_, _, _) => ('●', '⊕'),
                        GateOp::CSWAP(_, _, _) => ('●', '╳'),
                        _ => unreachable!(),
                    };
                    let is_cswap = matches!(op, GateOp::CSWAP(_, _, _));

                    let mut line: Vec<char> = vec![' '; total_width];

                    for i in 0..nq {
                        let center = i * (col_width + 1) + col_width / 2;
                        if i < min_q || i > max_q {
                            line[center] = '│';
                        } else if i == *c1 {
                            line[center] = sym_c;
                        } else if i == *c2 {
                            line[center] = if is_cswap { sym_t } else { sym_c };
                        } else if i == *t {
                            line[center] = sym_t;
                        }
                    }

                    let min_center = min_q * (col_width + 1) + col_width / 2;
                    let max_center = max_q * (col_width + 1) + col_width / 2;
                    for pos in (min_center + 1)..max_center {
                        if line[pos] == ' ' {
                            line[pos] = '─';
                        }
                    }

                    for i in 0..nc {
                        let center = q_total + gap_width + i * (col_width + 1) + col_width / 2;
                        line[center] = '║';
                    }

                    let gate_line: String = line.into_iter().collect();
                    writeln!(f, "{}", gate_line)?;
                }
                GateOp::Measure(mq, mc) => {
                    let mut line: Vec<char> = vec![' '; total_width];

                    for i in 0..nq {
                        let col_start = i * (col_width + 1);
                        let center = col_start + col_width / 2;
                        if i < *mq {
                            line[center] = '│';
                        } else if i == *mq {
                            let label_start = col_start + (col_width - label.chars().count()) / 2;
                            for (j, ch) in label.chars().enumerate() {
                                line[label_start + j] = ch;
                            }
                        }
                    }

                    let mq_col_start = *mq * (col_width + 1);
                    let mq_center = mq_col_start + col_width / 2;
                    let mc_start = q_total + gap_width;
                    let mc_center = mc_start + *mc * (col_width + 1) + col_width / 2;

                    for pos in (mq_center + 2)..=mc_center {
                        if line[pos] == ' ' {
                            line[pos] = '═';
                        }
                    }
                    line[mc_center] = '╣';

                    for i in 0..nc {
                        let center = mc_start + i * (col_width + 1) + col_width / 2;
                        if i > *mc {
                            line[center] = '║';
                        }
                    }

                    let measure_line: String = line.into_iter().collect();
                    writeln!(f, "{}", measure_line)?;
                }
                GateOp::Custom(_, targets) => {
                    let mut line: Vec<char> = vec![' '; total_width];

                    if targets.len() == 1 {
                        for i in 0..nq {
                            let col_start = i * (col_width + 1);
                            let center = col_start + col_width / 2;
                            if i == targets[0] {
                                let label_start =
                                    col_start + (col_width - label.chars().count()) / 2;
                                for (j, ch) in label.chars().enumerate() {
                                    line[label_start + j] = ch;
                                }
                            } else {
                                line[center] = '│';
                            }
                        }

                        for i in 0..nc {
                            let center = q_total + gap_width + i * (col_width + 1) + col_width / 2;
                            line[center] = '║';
                        }
                    } else {
                        for i in 0..nq {
                            let col_start = i * (col_width + 1);
                            let center = col_start + col_width / 2;
                            if i < min_q || i > max_q {
                                line[center] = '│';
                            } else if i == targets[0] {
                                let label_start =
                                    col_start + (col_width - label.chars().count()) / 2;
                                for (j, ch) in label.chars().enumerate() {
                                    line[label_start + j] = ch;
                                }
                            } else if targets.contains(&i) {
                                line[center] = '□';
                            }
                        }

                        let min_center = min_q * (col_width + 1) + col_width / 2;
                        let max_center = max_q * (col_width + 1) + col_width / 2;
                        for pos in (min_center + 1)..max_center {
                            if line[pos] == ' ' {
                                line[pos] = '─';
                            }
                        }

                        for i in 0..nc {
                            let center = q_total + gap_width + i * (col_width + 1) + col_width / 2;
                            line[center] = '║';
                        }
                    }

                    let gate_line: String = line.into_iter().collect();
                    writeln!(f, "{}", gate_line)?;
                }
            }
        }

        writeln!(f, "{}", full_wires)?;

        let end_line: String = "░".repeat(total_width);
        writeln!(f, "{}", end_line)?;

        Ok(())
    }
}
