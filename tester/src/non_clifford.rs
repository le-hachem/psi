use crate::common::{benchmark_circuit, print_circuit, print_section, BenchmarkResult};
use libpsi_core::QuantumCircuit;
use std::f64::consts::PI;

pub fn run_all(results: &mut Vec<BenchmarkResult>) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("                   NON-CLIFFORD GATES TESTS");
    println!("═══════════════════════════════════════════════════════════════\n");

    test_fixed_gates(results);
    test_rotation_gates(results);
    test_phase_gates(results);
    test_general_unitaries(results);
    test_controlled_rotations(results);
    test_variational_circuit(results);
}

pub fn test_fixed_gates(results: &mut Vec<BenchmarkResult>) {
    print_section("Non-Clifford Gates: T, T†, √X, S†");

    let builder = || {
        let mut circuit = QuantumCircuit::new(2);
        circuit.h(0).t(0).tdg(0).sx(1).sxdg(1).h(0).s(0).sdg(0);
        circuit
    };

    print_circuit(&builder());
    results.push(benchmark_circuit("Non-Clifford fixed gates", builder));

    let mut display = builder();
    display.compute();
    println!("{}\n", display);
}

pub fn test_rotation_gates(results: &mut Vec<BenchmarkResult>) {
    print_section("Rotation Gates: Rx, Ry, Rz");

    let builder = || {
        let mut circuit = QuantumCircuit::new(3);
        circuit
            .rx(0, PI / 4.0)
            .ry(1, PI / 2.0)
            .rz(2, PI)
            .rx(0, -PI / 4.0);
        circuit
    };

    print_circuit(&builder());
    results.push(benchmark_circuit("Rotation gates (3 qubits)", builder));

    let mut display = builder();
    display.compute();
    println!("{}\n", display);
}

pub fn test_phase_gates(results: &mut Vec<BenchmarkResult>) {
    print_section("Phase Gate: P(θ)");

    let builder = || {
        let mut circuit = QuantumCircuit::new(2);
        circuit.h(0).p(0, PI / 4.0).h(1).p(1, PI / 2.0);
        circuit
    };

    print_circuit(&builder());
    results.push(benchmark_circuit("Phase gates (2 qubits)", builder));

    let mut display = builder();
    display.compute();
    println!("{}\n", display);
}

pub fn test_general_unitaries(results: &mut Vec<BenchmarkResult>) {
    print_section("General Unitaries: U1, U2, U3");

    let builder = || {
        let mut circuit = QuantumCircuit::new(3);
        circuit
            .u1(0, PI / 4.0)
            .u2(1, 0.0, PI)
            .u3(2, PI / 2.0, 0.0, PI);
        circuit
    };

    print_circuit(&builder());
    results.push(benchmark_circuit("General unitaries (3 qubits)", builder));

    let mut display = builder();
    display.compute();
    println!("{}\n", display);
}

pub fn test_controlled_rotations(results: &mut Vec<BenchmarkResult>) {
    print_section("Controlled Rotation Gates: CRx, CRy, CRz, CP");

    let builder = || {
        let mut circuit = QuantumCircuit::new(4);
        circuit
            .x(0)
            .crx(0, 1, PI / 2.0)
            .x(2)
            .cry(2, 3, PI / 4.0)
            .crz(0, 2, PI)
            .cp(1, 3, PI / 2.0);
        circuit
    };

    print_circuit(&builder());
    results.push(benchmark_circuit(
        "Controlled rotations (4 qubits)",
        builder,
    ));

    let mut display = builder();
    display.compute();
    println!("{}\n", display);
}

pub fn test_variational_circuit(results: &mut Vec<BenchmarkResult>) {
    print_section("Variational Circuit (VQE-like)");

    let builder = || {
        let mut circuit = QuantumCircuit::new(3);
        circuit.ry(0, 0.5).ry(1, 0.3).ry(2, 0.7);
        circuit.cnot(0, 1).cnot(1, 2);
        circuit.rx(0, 0.2).rx(1, 0.4).rx(2, 0.6);
        circuit.cz(0, 2);
        circuit
    };

    print_circuit(&builder());
    results.push(benchmark_circuit("Variational circuit (3 qubits)", builder));

    let mut display = builder();
    display.compute();
    println!("{}\n", display);
}
