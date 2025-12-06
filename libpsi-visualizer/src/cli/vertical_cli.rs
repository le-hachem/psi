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

        let col_width = 5;
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

        for op in ops {
            writeln!(f, "{}", full_wires)?;

            let q_targets = op.quantum_targets();
            let min_q = q_targets.iter().min().copied().unwrap_or(0);
            let max_q = q_targets.iter().max().copied().unwrap_or(0);

            let mut q_cols: Vec<String> = (0..nq)
                .map(|_| format!("{:^width$}", "│", width = col_width))
                .collect();

            let c_cols: Vec<String> = (0..nc)
                .map(|_| format!("{:^width$}", "║", width = col_width))
                .collect();

            match op {
                GateOp::H(t) => {
                    q_cols[*t] = format!("{:^width$}", "[H]", width = col_width);
                }
                GateOp::X(t) => {
                    q_cols[*t] = format!("{:^width$}", "[X]", width = col_width);
                }
                GateOp::Y(t) => {
                    q_cols[*t] = format!("{:^width$}", "[Y]", width = col_width);
                }
                GateOp::Z(t) => {
                    q_cols[*t] = format!("{:^width$}", "[Z]", width = col_width);
                }
                GateOp::S(t) => {
                    q_cols[*t] = format!("{:^width$}", "[S]", width = col_width);
                }
                GateOp::T(t) => {
                    q_cols[*t] = format!("{:^width$}", "[T]", width = col_width);
                }
                GateOp::CNOT(c, t) | GateOp::CZ(c, t) | GateOp::SWAP(c, t) => {
                    let (sym1, sym2) = match op {
                        GateOp::CNOT(_, _) => ("●", "⊕"),
                        GateOp::CZ(_, _) => ("●", "●"),
                        GateOp::SWAP(_, _) => ("╳", "╳"),
                        _ => unreachable!(),
                    };

                    let q_total = nq * col_width + (nq - 1);
                    let c_total = if nc > 0 { nc * col_width + (nc - 1) } else { 0 };
                    let total_width = q_total + gap_width + c_total;

                    let mut line: Vec<char> = vec![' '; total_width];

                    for i in 0..nq {
                        let center = i * (col_width + 1) + col_width / 2;
                        if i < min_q || i > max_q {
                            line[center] = '│';
                        } else if i == *c {
                            line[center] = sym1.chars().next().unwrap();
                        } else if i == *t {
                            line[center] = sym2.chars().next().unwrap();
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
                    continue;
                }
                GateOp::CCNOT(c1, c2, t) | GateOp::CSWAP(c1, c2, t) => {
                    let (sym_c, sym_t) = match op {
                        GateOp::CCNOT(_, _, _) => ('●', '⊕'),
                        GateOp::CSWAP(_, _, _) => ('●', '╳'),
                        _ => unreachable!(),
                    };
                    let is_cswap = matches!(op, GateOp::CSWAP(_, _, _));

                    let q_total = nq * col_width + (nq - 1);
                    let c_total = if nc > 0 { nc * col_width + (nc - 1) } else { 0 };
                    let total_width = q_total + gap_width + c_total;

                    let mut line: Vec<char> = vec![' '; total_width];

                    for i in 0..nq {
                        let center = i * (col_width + 1) + col_width / 2;
                        if i < min_q || i > max_q {
                            line[center] = '│';
                        } else if i == *c1 {
                            line[center] = sym_c;
                        } else if i == *c2 {
                            if is_cswap {
                                line[center] = sym_t;
                            } else {
                                line[center] = sym_c;
                            }
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
                    continue;
                }
                GateOp::Measure(mq, mc) => {
                    let q_total = nq * col_width + (nq - 1);
                    let c_total = if nc > 0 { nc * col_width + (nc - 1) } else { 0 };
                    let total_width = q_total + gap_width + c_total;

                    let mut line: Vec<char> = vec![' '; total_width];

                    for i in 0..nq {
                        let center = i * (col_width + 1) + col_width / 2;
                        if i < *mq {
                            line[center] = '│';
                        } else if i == *mq {
                            let start = i * (col_width + 1);
                            let chars: Vec<char> = "[M]".chars().collect();
                            for (j, ch) in chars.iter().enumerate() {
                                if start + j + 1 < total_width {
                                    line[start + j + 1] = *ch;
                                }
                            }
                        }
                    }

                    let mq_center = *mq * (col_width + 1) + col_width / 2;
                    let mc_start = q_total + gap_width;
                    let mc_center = mc_start + *mc * (col_width + 1) + col_width / 2;

                    for pos in (mq_center + 2)..=mc_center {
                        if line[pos] == ' ' {
                            line[pos] = '═';
                        }
                    }
                    line[mc_center] = '╣';

                    if nc > 0 {
                        for i in 0..nc {
                            let center = mc_start + i * (col_width + 1) + col_width / 2;
                            if i > *mc {
                                line[center] = '║';
                            }
                        }
                    }

                    let measure_line: String = line.into_iter().collect();
                    writeln!(f, "{}", measure_line)?;
                    continue;
                }
            }

            let q_row: String = q_cols.join(" ");
            let c_row: String = c_cols.join(" ");
            if nc > 0 {
                writeln!(f, "{}{}{}", q_row, " ".repeat(gap_width), c_row)?;
            } else {
                writeln!(f, "{}", q_row)?;
            }
        }

        writeln!(f, "{}", full_wires)?;

        let q_total = nq * col_width + (nq - 1);
        let c_total = if nc > 0 { nc * col_width + (nc - 1) } else { 0 };
        let total_width = q_total + gap_width + c_total;

        let end_line: String = "░".repeat(total_width);
        writeln!(f, "{}", end_line)?;

        Ok(())
    }
}
