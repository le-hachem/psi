use crate::common::{print_section, states_equal, BenchmarkResult};
use libpsi_core::{get_simd_info, QuantumCircuit, Runtime};
use std::f64::consts::PI;
use std::time::Instant;

pub fn run_all(results: &mut Vec<BenchmarkResult>) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    SIMD ACCELERATION TESTS");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Detected: {}\n", get_simd_info());

    test_simd_correctness(results);
    test_simd_vs_batched(results);
    test_simd_large_circuits(results);
}

pub fn test_simd_correctness(results: &mut Vec<BenchmarkResult>) {
    print_section("SIMD Correctness Verification");

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
            "GHZ-3",
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
            "Mixed Single-Qubit",
            Box::new(|| {
                let mut c = QuantumCircuit::new(4);
                c.h(0).t(0).s(0).x(0).h(1).y(1).z(1).h(2).t(2).h(3).s(3);
                c
            }),
        ),
    ];

    for (name, builder) in test_cases {
        let mut basic = builder();
        basic.compute_with(Runtime::BasicRT);

        let mut simd = builder();
        simd.compute_with(Runtime::SimdRT);

        let match_result = states_equal(basic.state(), simd.state());

        println!(
            "{}: {}",
            name,
            if match_result {
                "✓ Match"
            } else {
                "✗ MISMATCH"
            }
        );

        results.push(BenchmarkResult {
            name: format!("SIMD verify: {}", name),
            basic_time: std::time::Duration::from_micros(0),
            mt_time: std::time::Duration::from_micros(0),
            results_match: match_result,
        });
    }
    println!();
}

pub fn test_simd_vs_batched(results: &mut Vec<BenchmarkResult>) {
    print_section("SIMD vs Batched Runtime Comparison");

    let test_cases: Vec<(&str, Box<dyn Fn() -> QuantumCircuit>)> = vec![
        (
            "Single-Qubit Heavy (6q)",
            Box::new(|| {
                let mut c = QuantumCircuit::new(6);
                for q in 0..6 {
                    c.h(q).t(q).s(q).x(q).y(q).z(q);
                }
                c
            }),
        ),
        (
            "Rotation Circuit (5q)",
            Box::new(|| {
                let mut c = QuantumCircuit::new(5);
                for q in 0..5 {
                    c.rx(q, PI / 4.0).ry(q, PI / 3.0).rz(q, PI / 6.0);
                }
                c
            }),
        ),
        (
            "Deep Single-Qubit (4q)",
            Box::new(|| {
                let mut c = QuantumCircuit::new(4);
                for _ in 0..10 {
                    for q in 0..4 {
                        c.h(q).t(q);
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

        let mut simd = builder();
        let start = Instant::now();
        simd.compute_with(Runtime::SimdRT);
        let simd_time = start.elapsed();

        let match_result = states_equal(batched.state(), simd.state());

        let speedup = batched_time.as_secs_f64() / simd_time.as_secs_f64();
        println!(
            "{}: Batched={:.2}μs, SIMD={:.2}μs, Speedup={:.2}x, Match={}",
            name,
            batched_time.as_secs_f64() * 1_000_000.0,
            simd_time.as_secs_f64() * 1_000_000.0,
            speedup,
            if match_result { "✓" } else { "✗" }
        );

        results.push(BenchmarkResult {
            name: format!("SIMD: {}", name),
            basic_time: batched_time,
            mt_time: simd_time,
            results_match: match_result,
        });
    }
    println!();
}

pub fn test_simd_large_circuits(results: &mut Vec<BenchmarkResult>) {
    print_section("SIMD on Large Circuits (Multi-threaded)");

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
            for i in 0..n {
                circuit.t(i).s(i);
            }
            circuit
        };

        let mut batched_mt = builder();
        let start = Instant::now();
        batched_mt.compute_with(Runtime::BatchedRTMT);
        let batched_time = start.elapsed();

        let mut simd_mt = builder();
        let start = Instant::now();
        simd_mt.compute_with(Runtime::SimdRTMT);
        let simd_time = start.elapsed();

        let match_result = states_equal(batched_mt.state(), simd_mt.state());

        let speedup = batched_time.as_secs_f64() / simd_time.as_secs_f64();
        println!(
            "{}-qubit: BatchedMT={:.3}ms, SIMD_MT={:.3}ms, Speedup={:.2}x, Match={}",
            n,
            batched_time.as_secs_f64() * 1000.0,
            simd_time.as_secs_f64() * 1000.0,
            speedup,
            if match_result { "✓" } else { "✗" }
        );

        results.push(BenchmarkResult {
            name: format!("{}-qubit SIMD", n),
            basic_time: batched_time,
            mt_time: simd_time,
            results_match: match_result,
        });
    }
    println!();
}
