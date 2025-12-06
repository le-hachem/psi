use libpsi_core::*;
use libpsi_visualizer::*;
use std::time::{Duration, Instant};

struct BenchmarkResult {
    name: String,
    basic_time: Duration,
    mt_time: Duration,
    results_match: bool,
}

fn benchmark_circuit<F>(name: &str, circuit_builder: F) -> BenchmarkResult
where
    F: Fn() -> QuantumCircuit,
{
    let mut circuit_st = circuit_builder();
    let mut circuit_mt = circuit_builder();

    let start_st = Instant::now();
    circuit_st.compute_with(Runtime::BasicRT);
    let basic_time = start_st.elapsed();

    let start_mt = Instant::now();
    circuit_mt.compute_with(Runtime::BasicRTMT);
    let mt_time = start_mt.elapsed();

    let state_st = circuit_st.state();
    let state_mt = circuit_mt.state();

    let results_match = states_equal(state_st, state_mt);

    BenchmarkResult {
        name: name.to_string(),
        basic_time,
        mt_time,
        results_match,
    }
}

fn states_equal(a: &QuantumState, b: &QuantumState) -> bool {
    use crate::maths::vector::Vector;
    if a.size() != b.size() {
        return false;
    }
    for i in 0..a.size() {
        let amp_a = a.get(i);
        let amp_b = b.get(i);
        let diff_real = (amp_a.real - amp_b.real).abs();
        let diff_imag = (amp_a.imaginary - amp_b.imaginary).abs();
        if diff_real > 1e-10 || diff_imag > 1e-10 {
            return false;
        }
    }
    true
}

fn format_duration(d: Duration) -> String {
    if d.as_secs() > 0 {
        format!("{:.3}s", d.as_secs_f64())
    } else if d.as_millis() > 0 {
        format!("{:.3}ms", d.as_secs_f64() * 1000.0)
    } else {
        format!("{:.3}us", d.as_secs_f64() * 1_000_000.0)
    }
}

fn print_benchmark_table(results: &[BenchmarkResult]) {
    // Column widths (content only, not including borders)
    const C1: usize = 30; // Circuit name
    const C2: usize = 12; // BasicRT
    const C3: usize = 12; // BasicRTMT
    const C4: usize = 10; // Speedup
    const C5: usize = 5; // Match

    let top = format!(
        "╔{}═{}═{}═{}═{}╗",
        "═".repeat(C1 + 2),
        "═".repeat(C2 + 2),
        "═".repeat(C3 + 2),
        "═".repeat(C4 + 2),
        "═".repeat(C5 + 2)
    );
    let title = format!(
        "╠{}╤{}╤{}╤{}╤{}╣",
        "═".repeat(C1 + 2),
        "═".repeat(C2 + 2),
        "═".repeat(C3 + 2),
        "═".repeat(C4 + 2),
        "═".repeat(C5 + 2)
    );
    let header = format!(
        "╠{}╪{}╪{}╪{}╪{}╣",
        "═".repeat(C1 + 2),
        "═".repeat(C2 + 2),
        "═".repeat(C3 + 2),
        "═".repeat(C4 + 2),
        "═".repeat(C5 + 2)
    );
    let bottom = format!(
        "╚{}╧{}╧{}╧{}╧{}╝",
        "═".repeat(C1 + 2),
        "═".repeat(C2 + 2),
        "═".repeat(C3 + 2),
        "═".repeat(C4 + 2),
        "═".repeat(C5 + 2)
    );

    let total_width = C1 + C2 + C3 + C4 + C5 + 14;

    println!("\n{}", top);
    println!(
        "║{:^width$}║",
        "RUNTIME BENCHMARK RESULTS",
        width = total_width
    );
    println!("{}", title);
    println!(
        "║ {:<C1$} │ {:^C2$} │ {:^C3$} │ {:^C4$} │ {:^C5$} ║",
        "Circuit",
        "BasicRT",
        "BasicRTMT",
        "Speedup",
        "Match",
        C1 = C1,
        C2 = C2,
        C3 = C3,
        C4 = C4,
        C5 = C5
    );
    println!("{}", header);

    for r in results {
        let speedup = r.basic_time.as_secs_f64() / r.mt_time.as_secs_f64();
        let speedup_str = format!("{:.2}x", speedup);
        let match_str = if r.results_match { "✓" } else { "✗" };

        println!(
            "║ {:<C1$} │ {:>C2$} │ {:>C3$} │ {:>C4$} │ {:^C5$} ║",
            r.name,
            format_duration(r.basic_time),
            format_duration(r.mt_time),
            speedup_str,
            match_str,
            C1 = C1,
            C2 = C2,
            C3 = C3,
            C4 = C4,
            C5 = C5
        );
    }

    println!("{}", bottom);
}

fn main() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    PSI Quantum Simulator");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut results: Vec<BenchmarkResult> = Vec::new();

    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ Bell State with Measurement                                 │");
    println!("└─────────────────────────────────────────────────────────────┘\n");

    let bell_builder = || {
        let mut circuit = QuantumCircuit::with_classical(2, 2);
        circuit.h(0).cnot(0, 1).measure(0, 0).measure(1, 1);
        circuit
    };

    let bell = bell_builder();
    println!("Horizontal:\n{}", HorizontalRenderer::new(&bell));
    println!("Vertical:\n{}", VerticalRenderer::new(&bell));

    results.push(benchmark_circuit("Bell State (2 qubits)", bell_builder));
    let mut bell_display = bell_builder();
    bell_display.compute();
    println!("{}\n", bell_display);

    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ GHZ State                                                   │");
    println!("└─────────────────────────────────────────────────────────────┘\n");

    let ghz_builder = || {
        let mut circuit = QuantumCircuit::new(3);
        circuit.h(0).cnot(0, 1).cnot(0, 2);
        circuit
    };

    let ghz = ghz_builder();
    println!("Horizontal:\n{}", HorizontalRenderer::new(&ghz));
    println!("Vertical:\n{}", VerticalRenderer::new(&ghz));

    results.push(benchmark_circuit("GHZ State (3 qubits)", ghz_builder));
    let mut ghz_display = ghz_builder();
    ghz_display.compute();
    println!("{}\n", ghz_display);

    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ SWAP via 3 CNOTs                                            │");
    println!("└─────────────────────────────────────────────────────────────┘\n");

    let swap_builder = || {
        let mut circuit = QuantumCircuit::new(2);
        circuit.x(0).cnot(0, 1).cnot(1, 0).cnot(0, 1);
        circuit
    };

    let swap_circuit = swap_builder();
    println!("Horizontal:\n{}", HorizontalRenderer::new(&swap_circuit));
    println!("Vertical:\n{}", VerticalRenderer::new(&swap_circuit));

    results.push(benchmark_circuit("SWAP via CNOTs (2 qubits)", swap_builder));
    let mut swap_display = swap_builder();
    swap_display.compute();
    println!("{}\n", swap_display);

    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ Toffoli Gate                                                │");
    println!("└─────────────────────────────────────────────────────────────┘\n");

    let toffoli_builder = || {
        let mut circuit = QuantumCircuit::new(3);
        circuit.x(0).x(1).toffoli(0, 1, 2);
        circuit
    };

    let toffoli = toffoli_builder();
    println!("Horizontal:\n{}", HorizontalRenderer::new(&toffoli));
    println!("Vertical:\n{}", VerticalRenderer::new(&toffoli));

    results.push(benchmark_circuit("Toffoli (3 qubits)", toffoli_builder));
    let mut toffoli_display = toffoli_builder();
    toffoli_display.compute();
    println!("{}\n", toffoli_display);

    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ Full Circuit with Measurements                              │");
    println!("└─────────────────────────────────────────────────────────────┘\n");

    let full_builder = || {
        let mut circuit = QuantumCircuit::with_classical(3, 3);
        circuit.h(0).h(1).h(2).measure_all();
        circuit
    };

    let full = full_builder();
    println!("Horizontal:\n{}", HorizontalRenderer::new(&full));
    println!("Vertical:\n{}", VerticalRenderer::new(&full));

    results.push(benchmark_circuit(
        "3-qubit Hadamard + Measure",
        full_builder,
    ));
    let mut full_display = full_builder();
    full_display.compute();
    println!("{}\n", full_display);

    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ Complex Circuit                                             │");
    println!("└─────────────────────────────────────────────────────────────┘\n");

    let complex_builder = || {
        let mut circuit = QuantumCircuit::with_classical(4, 2);
        circuit
            .h(0)
            .h(1)
            .cnot(0, 2)
            .cnot(1, 3)
            .cz(2, 3)
            .swap(0, 1)
            .measure(0, 0)
            .measure(1, 1);
        circuit
    };

    let complex = complex_builder();
    println!("Horizontal:\n{}", HorizontalRenderer::new(&complex));
    println!("Vertical:\n{}", VerticalRenderer::new(&complex));

    results.push(benchmark_circuit("Complex (4 qubits)", complex_builder));
    let mut complex_display = complex_builder();
    complex_display.compute();
    println!("{}\n", complex_display);

    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ Custom Gate: Bell Pair Creator                              │");
    println!("└─────────────────────────────────────────────────────────────┘\n");

    let bell_gate = CustomGateBuilder::new("BELL", 2).h(0).cnot(0, 1).build();
    let bell_gate_clone = bell_gate.clone();

    let custom_bell_builder = move || {
        let mut circuit = QuantumCircuit::new(4);
        circuit
            .apply_custom(bell_gate_clone.clone(), &[0, 1])
            .apply_custom(bell_gate_clone.clone(), &[2, 3]);
        circuit
    };

    let custom_bell = {
        let mut circuit = QuantumCircuit::new(4);
        circuit
            .apply_custom(bell_gate.clone(), &[0, 1])
            .apply_custom(bell_gate.clone(), &[2, 3]);
        circuit
    };
    println!("Horizontal:\n{}", HorizontalRenderer::new(&custom_bell));
    println!("Vertical:\n{}", VerticalRenderer::new(&custom_bell));

    results.push(benchmark_circuit(
        "Custom BELL (4 qubits)",
        custom_bell_builder,
    ));
    let mut custom_bell_display = {
        let mut circuit = QuantumCircuit::new(4);
        circuit
            .apply_custom(bell_gate.clone(), &[0, 1])
            .apply_custom(bell_gate.clone(), &[2, 3]);
        circuit
    };
    custom_bell_display.compute();
    println!("{}\n", custom_bell_display);

    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ Custom Gate: Swap via CNOTs                                 │");
    println!("└─────────────────────────────────────────────────────────────┘\n");

    let swap_gate = CustomGateBuilder::new("MYSWAP", 2)
        .cnot(0, 1)
        .cnot(1, 0)
        .cnot(0, 1)
        .build();
    let swap_gate_clone = swap_gate.clone();

    let custom_swap_builder = move || {
        let mut circuit = QuantumCircuit::new(2);
        circuit.x(0).apply_custom(swap_gate_clone.clone(), &[0, 1]);
        circuit
    };

    let custom_swap = {
        let mut circuit = QuantumCircuit::new(2);
        circuit.x(0).apply_custom(swap_gate.clone(), &[0, 1]);
        circuit
    };
    println!("Horizontal:\n{}", HorizontalRenderer::new(&custom_swap));
    println!("Vertical:\n{}", VerticalRenderer::new(&custom_swap));

    results.push(benchmark_circuit(
        "Custom SWAP (2 qubits)",
        custom_swap_builder,
    ));
    let mut custom_swap_display = {
        let mut circuit = QuantumCircuit::new(2);
        circuit.x(0).apply_custom(swap_gate.clone(), &[0, 1]);
        circuit
    };
    custom_swap_display.compute();
    println!("{}\n", custom_swap_display);

    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ Custom Gate: Matrix-defined √X gate                         │");
    println!("└─────────────────────────────────────────────────────────────┘\n");

    let sqrt_x_matrix = matrix!(
        [complex!(0.5, 0.5), complex!(0.5, -0.5)];
        [complex!(0.5, -0.5), complex!(0.5, 0.5)]
    );
    let sqrt_x = CustomGate::from_matrix("√X", sqrt_x_matrix);
    let sqrt_x_clone = sqrt_x.clone();

    let sqrt_x_builder = move || {
        let mut circuit = QuantumCircuit::new(1);
        circuit
            .apply_custom(sqrt_x_clone.clone(), &[0])
            .apply_custom(sqrt_x_clone.clone(), &[0]);
        circuit
    };

    let sqrt_x_circuit = {
        let mut circuit = QuantumCircuit::new(1);
        circuit
            .apply_custom(sqrt_x.clone(), &[0])
            .apply_custom(sqrt_x.clone(), &[0]);
        circuit
    };
    println!("Horizontal:\n{}", HorizontalRenderer::new(&sqrt_x_circuit));
    println!("Vertical:\n{}", VerticalRenderer::new(&sqrt_x_circuit));

    results.push(benchmark_circuit("√X gate (1 qubit)", sqrt_x_builder));
    let mut sqrt_x_display = {
        let mut circuit = QuantumCircuit::new(1);
        circuit
            .apply_custom(sqrt_x.clone(), &[0])
            .apply_custom(sqrt_x.clone(), &[0]);
        circuit
    };
    sqrt_x_display.compute();
    println!("{}", sqrt_x_display);
    println!("(Two √X gates should equal X, so |0⟩ becomes |1⟩)\n");

    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ Larger Circuits (for benchmark comparison)                  │");
    println!("└─────────────────────────────────────────────────────────────┘\n");

    let large_8_builder = || {
        let mut circuit = QuantumCircuit::new(8);
        for i in 0..8 {
            circuit.h(i);
        }
        for i in 0..7 {
            circuit.cnot(i, i + 1);
        }
        circuit
    };
    println!("8-qubit entangled:");
    println!("{}", HorizontalRenderer::new(&large_8_builder()));
    results.push(benchmark_circuit("8-qubit entangled", large_8_builder));

    let large_10_builder = || {
        let mut circuit = QuantumCircuit::new(10);
        for i in 0..10 {
            circuit.h(i);
        }
        for i in 0..9 {
            circuit.cnot(i, i + 1);
        }
        circuit.cz(0, 9);
        circuit
    };
    println!("10-qubit entangled:");
    println!("{}", HorizontalRenderer::new(&large_10_builder()));
    results.push(benchmark_circuit("10-qubit entangled", large_10_builder));

    let large_12_builder = || {
        let mut circuit = QuantumCircuit::new(12);
        for i in 0..12 {
            circuit.h(i);
        }
        for i in 0..11 {
            circuit.cnot(i, i + 1);
        }
        circuit.cz(0, 11);
        circuit.swap(5, 6);
        circuit
    };
    println!("12-qubit entangled:");
    println!("{}", HorizontalRenderer::new(&large_12_builder()));
    results.push(benchmark_circuit("12-qubit entangled", large_12_builder));

    let large_14_builder = || {
        let mut circuit = QuantumCircuit::new(14);
        for i in 0..14 {
            circuit.h(i);
        }
        for i in 0..13 {
            circuit.cnot(i, i + 1);
        }
        circuit
    };
    println!("14-qubit entangled:");
    println!("{}", HorizontalRenderer::new(&large_14_builder()));
    results.push(benchmark_circuit("14-qubit entangled", large_14_builder));

    print_benchmark_table(&results);

    let all_match = results.iter().all(|r| r.results_match);
    println!("\n");
    if all_match {
        println!("✓ All circuits produced identical results with both runtimes!");
    } else {
        println!("✗ WARNING: Some circuits produced different results!");
    }

    let total_basic: Duration = results.iter().map(|r| r.basic_time).sum();
    let total_mt: Duration = results.iter().map(|r| r.mt_time).sum();
    let overall_speedup = total_basic.as_secs_f64() / total_mt.as_secs_f64();

    println!(
        "\nTotal time - BasicRT: {} | BasicRTMT: {} | Overall speedup: {:.2}x",
        format_duration(total_basic),
        format_duration(total_mt),
        overall_speedup
    );
}
