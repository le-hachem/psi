use libpsi_core::{QuantumCircuit, QuantumState, Runtime, Vector};
use libpsi_visualizer::{HorizontalRenderer, VerticalRenderer};
use std::time::{Duration, Instant};

pub struct BenchmarkResult {
    pub name: String,
    pub basic_time: Duration,
    pub mt_time: Duration,
    pub results_match: bool,
}

pub fn benchmark_circuit<F>(name: &str, circuit_builder: F) -> BenchmarkResult
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

pub fn states_equal(a: &QuantumState, b: &QuantumState) -> bool {
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

pub fn format_duration(d: Duration) -> String {
    if d.as_secs() > 0 {
        format!("{:.3}s", d.as_secs_f64())
    } else if d.as_millis() > 0 {
        format!("{:.3}ms", d.as_secs_f64() * 1000.0)
    } else {
        format!("{:.3}us", d.as_secs_f64() * 1_000_000.0)
    }
}

pub fn print_section(title: &str) {
    let width = 61;
    let padding = width - title.len() - 2;
    println!("┌{}┐", "─".repeat(width));
    println!("│ {}{} │", title, " ".repeat(padding));
    println!("└{}┘\n", "─".repeat(width));
}

pub fn print_circuit(circuit: &QuantumCircuit) {
    println!("Horizontal:\n{}", HorizontalRenderer::new(circuit));
    println!("Vertical:\n{}", VerticalRenderer::new(circuit));
}

pub fn print_benchmark_table(results: &[BenchmarkResult]) {
    const C1: usize = 30;
    const C2: usize = 12;
    const C3: usize = 12;
    const C4: usize = 10;
    const C5: usize = 5;

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
        "Circuit", "BasicRT", "BasicRTMT", "Speedup", "Match",
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
        );
    }

    println!("{}", bottom);
}

pub fn print_summary(results: &[BenchmarkResult]) {
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
