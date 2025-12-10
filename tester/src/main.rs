mod benchmarks;
mod clifford;
mod common;
mod custom_gates;
mod kernels;
mod non_clifford;

use common::{print_benchmark_table, print_summary, BenchmarkResult};
use std::env;

fn print_header() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    PSI Quantum Simulator");
    println!("═══════════════════════════════════════════════════════════════\n");
}

fn print_usage() {
    println!("Usage: tester [OPTIONS]");
    println!();
    println!("Options:");
    println!("  all          Run all tests (default)");
    println!("  clifford     Run Clifford gate tests only");
    println!("  non-clifford Run non-Clifford gate tests only");
    println!("  custom       Run custom gate tests only");
    println!("  kernels      Run kernel batching tests only");
    println!("  bench        Run benchmark tests only");
    println!("  help         Show this help message");
    println!();
    println!("Examples:");
    println!("  tester                   # Run all tests");
    println!("  tester clifford          # Run only Clifford gate tests");
    println!("  tester non-clifford      # Run only rotation/parametric gate tests");
    println!("  tester kernels           # Run only kernel batching tests");
    println!("  tester custom bench      # Run custom gates and benchmarks");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args
        .iter()
        .any(|a| a == "help" || a == "--help" || a == "-h")
    {
        print_usage();
        return;
    }

    print_header();

    let mut results: Vec<BenchmarkResult> = Vec::new();

    let run_all = args.is_empty() || args.iter().any(|a| a == "all");
    let run_clifford = run_all || args.iter().any(|a| a == "clifford");
    let run_non_clifford = run_all || args.iter().any(|a| a == "non-clifford");
    let run_custom = run_all || args.iter().any(|a| a == "custom");
    let run_kernels = run_all || args.iter().any(|a| a == "kernels");
    let run_bench = run_all || args.iter().any(|a| a == "bench");

    if run_clifford {
        clifford::run_all(&mut results);
    }

    if run_non_clifford {
        non_clifford::run_all(&mut results);
    }

    if run_custom {
        custom_gates::run_all(&mut results);
    }

    if run_kernels {
        kernels::run_all(&mut results);
    }

    if run_bench {
        benchmarks::run_all(&mut results);
    }

    if !results.is_empty() {
        print_benchmark_table(&results);
        print_summary(&results);
    }
}
