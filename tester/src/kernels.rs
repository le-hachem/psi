use crate::common::{print_section, states_equal, BenchmarkResult};
use libpsi_core::{QuantumCircuit, Runtime, RuntimeConfig};
use std::f64::consts::PI;
use std::time::Instant;

pub fn run_all(results: &mut Vec<BenchmarkResult>) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    KERNEL BATCHING TESTS");
    println!("═══════════════════════════════════════════════════════════════\n");

    test_kernel_fusion(results);
    test_batched_vs_basic(results);
    test_batched_large_circuits(results);
    test_structure_aware(results);
    test_composable_runtime(results);
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

pub fn test_structure_aware(results: &mut Vec<BenchmarkResult>) {
    print_section("Structure-Aware Kernel Optimisation");

    let commute_test = || {
        let mut c = QuantumCircuit::new(3);
        c.t(0).h(1).t(0).h(2).s(0).t(1).rz(0, PI / 4.0);
        c
    };

    let circuit = commute_test();
    let mut batch = Runtime::build_structure_aware_batch(3, circuit.operations());
    let original = batch.len();
    println!("Original operations: {}", original);
    for (i, k) in batch.kernels().iter().enumerate() {
        println!("  {}: {} on {:?} ({:?})", i, k.name, k.targets, k.gate_type);
    }

    batch.optimise();
    let optimised = batch.len();
    println!("\nAfter optimisation: {}", optimised);
    for (i, k) in batch.kernels().iter().enumerate() {
        println!("  {}: {} on {:?}", i, k.name, k.targets);
    }

    println!("\nExecution layers: {}", batch.num_layers());
    for (i, layer) in batch.layers().iter().enumerate() {
        let names: Vec<_> = layer.kernels.iter().map(|k| k.name.as_str()).collect();
        println!("  Layer {}: {:?}", i, names);
    }

    let stats = batch.stats();
    println!("\nStats: {}", stats);

    let mut basic = commute_test();
    let start = Instant::now();
    basic.compute_with(Runtime::BasicRT);
    let basic_time = start.elapsed();

    let mut sa = commute_test();
    let start = Instant::now();
    sa.compute_with(Runtime::StructureAwareRT);
    let sa_time = start.elapsed();

    let match_result = states_equal(basic.state(), sa.state());
    println!(
        "\nBasic={:.2}μs, StructureAware={:.2}μs, Match={}",
        basic_time.as_secs_f64() * 1_000_000.0,
        sa_time.as_secs_f64() * 1_000_000.0,
        if match_result { "✓" } else { "✗" }
    );

    results.push(BenchmarkResult {
        name: format!("SA: Commuting ({}→{})", original, optimised),
        basic_time,
        mt_time: sa_time,
        results_match: match_result,
    });

    println!();
    print_section("Structure-Aware vs Other Runtimes");

    let test_cases: Vec<(&str, Box<dyn Fn() -> QuantumCircuit>)> = vec![
        (
            "Diagonal-heavy (5q)",
            Box::new(|| {
                let mut c = QuantumCircuit::new(5);
                for q in 0..5 {
                    c.t(q).s(q).rz(q, PI / 4.0).t(q);
                }
                c
            }),
        ),
        (
            "Interleaved (4q)",
            Box::new(|| {
                let mut c = QuantumCircuit::new(4);
                c.h(0).h(1).h(2).h(3);
                c.t(0).t(1).t(2).t(3);
                c.cnot(0, 1).cnot(2, 3);
                c.s(0).s(1).s(2).s(3);
                c
            }),
        ),
        (
            "Deep rotation (3q)",
            Box::new(|| {
                let mut c = QuantumCircuit::new(3);
                for _ in 0..5 {
                    for q in 0..3 {
                        c.rx(q, PI / 8.0).ry(q, PI / 8.0).rz(q, PI / 8.0);
                    }
                }
                c
            }),
        ),
    ];

    for (name, builder) in test_cases {
        let mut batched = builder();
        let start = Instant::now();
        batched.compute_with(Runtime::BatchedRT);
        let batched_time = start.elapsed();

        let mut sa = builder();
        let start = Instant::now();
        sa.compute_with(Runtime::StructureAwareRT);
        let sa_time = start.elapsed();

        let match_result = states_equal(batched.state(), sa.state());

        let speedup = batched_time.as_secs_f64() / sa_time.as_secs_f64();
        println!(
            "{}: Batched={:.2}μs, SA={:.2}μs, Speedup={:.2}x, Match={}",
            name,
            batched_time.as_secs_f64() * 1_000_000.0,
            sa_time.as_secs_f64() * 1_000_000.0,
            speedup,
            if match_result { "✓" } else { "✗" }
        );

        results.push(BenchmarkResult {
            name: format!("SA: {}", name),
            basic_time: batched_time,
            mt_time: sa_time,
            results_match: match_result,
        });
    }
    println!();
}

pub fn test_composable_runtime(results: &mut Vec<BenchmarkResult>) {
    print_section("Composable Runtime Configurations");

    let builder = || {
        let mut c = QuantumCircuit::new(6);
        for q in 0..6 {
            c.h(q).t(q).s(q);
        }
        for q in 0..5 {
            c.cnot(q, q + 1);
        }
        for q in 0..6 {
            c.rx(q, PI / 4.0).rz(q, PI / 4.0);
        }
        c
    };

    let configs: Vec<(&str, RuntimeConfig)> = vec![
        ("Basic", RuntimeConfig::new()),
        ("Batched", RuntimeConfig::new().batched()),
        ("SIMD", RuntimeConfig::new().simd()),
        ("Batched+SIMD", RuntimeConfig::new().batched().simd()),
        ("SA+SIMD", RuntimeConfig::new().structure_aware().simd()),
        (
            "SA+SIMD+Parallel",
            RuntimeConfig::new().structure_aware().simd().parallel(),
        ),
        ("Optimal", Runtime::optimal()),
    ];

    let mut reference = builder();
    reference.compute_with(Runtime::BasicRT);
    let ref_state = reference.state().clone();

    println!("Testing 6-qubit circuit with different runtime configurations:\n");

    for (name, config) in &configs {
        let mut circuit = builder();
        let start = Instant::now();
        circuit.compute_with_config(*config);
        let time = start.elapsed();

        let match_result = states_equal(&ref_state, circuit.state());

        println!(
            "{:20} : {:.2}μs, Match={}",
            name,
            time.as_secs_f64() * 1_000_000.0,
            if match_result { "✓" } else { "✗" }
        );

        results.push(BenchmarkResult {
            name: format!("Config: {}", name),
            basic_time: time,
            mt_time: time,
            results_match: match_result,
        });
    }

    println!("\nConfiguration Display Examples:");
    println!("  {}", RuntimeConfig::new());
    println!("  {}", RuntimeConfig::new().batched().simd());
    println!(
        "  {}",
        RuntimeConfig::new().structure_aware().simd().parallel()
    );
    println!("  {}", Runtime::optimal());
    println!();
}
