use crate::common::{print_section, states_equal, BenchmarkResult};
use libpsi_core::{QuantumCircuit, Runtime};
use std::f64::consts::PI;
use std::time::Instant;

pub fn run_all(results: &mut Vec<BenchmarkResult>) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    KERNEL BATCHING TESTS");
    println!("═══════════════════════════════════════════════════════════════\n");

    test_kernel_fusion(results);
    test_batched_vs_basic(results);
    test_batched_large_circuits(results);
}

pub fn test_kernel_fusion(results: &mut Vec<BenchmarkResult>) {
    print_section("Kernel Fusion Test");

    let builder = || {
        let mut circuit = QuantumCircuit::new(2);
        circuit.h(0).t(0).s(0).x(0).h(1).z(1);
        circuit
    };

    let circuit = builder();
    let batch = Runtime::build_kernel_batch(2, circuit.operations());
    let original_count = batch.len();
    println!("Original kernels: {}", original_count);
    for (i, k) in batch.kernels().iter().enumerate() {
        println!("  {}: {} on {:?}", i, k.name, k.targets);
    }

    let mut optimized_batch = Runtime::build_kernel_batch(2, circuit.operations());
    optimized_batch.optimize();
    let optimized_count = optimized_batch.len();
    println!("\nOptimized kernels: {}", optimized_count);
    for (i, k) in optimized_batch.kernels().iter().enumerate() {
        println!("  {}: {} on {:?}", i, k.name, k.targets);
    }

    let reduction = ((original_count - optimized_count) as f64 / original_count as f64) * 100.0;
    println!(
        "\nKernel reduction: {} → {} ({:.0}% fewer)",
        original_count, optimized_count, reduction
    );

    let mut basic = builder();
    let start = Instant::now();
    basic.compute_with(Runtime::BasicRT);
    let basic_time = start.elapsed();

    let mut batched = builder();
    let start = Instant::now();
    batched.compute_with(Runtime::BatchedRT);
    let batched_time = start.elapsed();

    let match_result = states_equal(basic.state(), batched.state());
    println!("Results match: {}\n", if match_result { "✓" } else { "✗" });

    results.push(BenchmarkResult {
        name: format!("Fusion ({}→{} kernels)", original_count, optimized_count),
        basic_time,
        mt_time: batched_time,
        results_match: match_result,
    });

    let fusion_heavy = || {
        let mut circuit = QuantumCircuit::new(1);
        circuit.h(0).t(0).s(0).x(0).y(0).z(0).h(0).t(0);
        circuit
    };

    let circuit2 = fusion_heavy();
    let batch2 = Runtime::build_kernel_batch(1, circuit2.operations());
    let orig2 = batch2.len();
    let mut opt_batch2 = Runtime::build_kernel_batch(1, circuit2.operations());
    opt_batch2.optimize();
    let opt2 = opt_batch2.len();

    let mut basic2 = fusion_heavy();
    let start = Instant::now();
    basic2.compute_with(Runtime::BasicRT);
    let basic_time2 = start.elapsed();

    let mut batched2 = fusion_heavy();
    let start = Instant::now();
    batched2.compute_with(Runtime::BatchedRT);
    let batched_time2 = start.elapsed();

    let match2 = states_equal(basic2.state(), batched2.state());

    results.push(BenchmarkResult {
        name: format!("Heavy fusion ({}→{} kernels)", orig2, opt2),
        basic_time: basic_time2,
        mt_time: batched_time2,
        results_match: match2,
    });
}

pub fn test_batched_vs_basic(results: &mut Vec<BenchmarkResult>) {
    print_section("Batched vs Basic Runtime Comparison");

    let test_cases: Vec<(&str, Box<dyn Fn() -> QuantumCircuit>)> = vec![
        (
            "Bell State",
            Box::new(|| {
                let mut c = QuantumCircuit::new(2);
                c.h(0).cnot(0, 1);
                c
            }),
        ),
        (
            "GHZ State",
            Box::new(|| {
                let mut c = QuantumCircuit::new(3);
                c.h(0).cnot(0, 1).cnot(0, 2);
                c
            }),
        ),
        (
            "Rotation Chain",
            Box::new(|| {
                let mut c = QuantumCircuit::new(3);
                c.rx(0, PI / 4.0)
                    .ry(0, PI / 4.0)
                    .rz(0, PI / 4.0)
                    .rx(1, PI / 3.0)
                    .ry(1, PI / 3.0);
                c
            }),
        ),
        (
            "Mixed Gates",
            Box::new(|| {
                let mut c = QuantumCircuit::new(4);
                c.h(0).h(1).h(2).h(3).cnot(0, 1).cnot(2, 3).cz(1, 2);
                c
            }),
        ),
    ];

    for (name, builder) in test_cases {
        let mut basic = builder();
        let start = Instant::now();
        basic.compute_with(Runtime::BasicRT);
        let basic_time = start.elapsed();

        let mut batched = builder();
        let start = Instant::now();
        batched.compute_with(Runtime::BatchedRT);
        let batched_time = start.elapsed();

        let match_result = states_equal(basic.state(), batched.state());

        println!(
            "{}: Basic={:.2}μs, Batched={:.2}μs, Match={}",
            name,
            basic_time.as_secs_f64() * 1_000_000.0,
            batched_time.as_secs_f64() * 1_000_000.0,
            if match_result { "✓" } else { "✗" }
        );

        results.push(BenchmarkResult {
            name: format!("Batched: {}", name),
            basic_time,
            mt_time: batched_time,
            results_match: match_result,
        });
    }
    println!();
}

pub fn test_batched_large_circuits(results: &mut Vec<BenchmarkResult>) {
    print_section("Batched Runtime on Large Circuits");

    let sizes = [8, 10, 12];

    for &n in &sizes {
        let builder = || {
            let mut circuit = QuantumCircuit::new(n);
            for i in 0..n {
                circuit.h(i);
            }
            for i in 0..(n - 1) {
                circuit.cnot(i, i + 1);
            }
            circuit
        };

        let mut basic_mt = builder();
        let start = Instant::now();
        basic_mt.compute_with(Runtime::BasicRTMT);
        let basic_mt_time = start.elapsed();

        let mut batched_mt = builder();
        let start = Instant::now();
        batched_mt.compute_with(Runtime::BatchedRTMT);
        let batched_mt_time = start.elapsed();

        let match_result = states_equal(basic_mt.state(), batched_mt.state());

        println!(
            "{}-qubit: BasicRTMT={:.3}ms, BatchedRTMT={:.3}ms, Match={}",
            n,
            basic_mt_time.as_secs_f64() * 1000.0,
            batched_mt_time.as_secs_f64() * 1000.0,
            if match_result { "✓" } else { "✗" }
        );

        results.push(BenchmarkResult {
            name: format!("{}-qubit batched", n),
            basic_time: basic_mt_time,
            mt_time: batched_mt_time,
            results_match: match_result,
        });
    }
    println!();
}
