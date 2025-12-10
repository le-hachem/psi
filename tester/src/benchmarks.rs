use crate::common::{benchmark_circuit, print_section, BenchmarkResult};
use libpsi_core::QuantumCircuit;
use libpsi_visualizer::HorizontalRenderer;

pub fn run_all(results: &mut Vec<BenchmarkResult>) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    BENCHMARK CIRCUITS");
    println!("═══════════════════════════════════════════════════════════════\n");

    test_8_qubit(results);
    test_10_qubit(results);
    test_12_qubit(results);
    test_14_qubit(results);
}

pub fn test_8_qubit(results: &mut Vec<BenchmarkResult>) {
    print_section("8-qubit Entangled Circuit");

    let builder = || {
        let mut circuit = QuantumCircuit::new(8);
        for i in 0..8 {
            circuit.h(i);
        }
        for i in 0..7 {
            circuit.cnot(i, i + 1);
        }
        circuit
    };

    println!("{}", HorizontalRenderer::new(&builder()));
    results.push(benchmark_circuit("8-qubit entangled", builder));
}

pub fn test_10_qubit(results: &mut Vec<BenchmarkResult>) {
    print_section("10-qubit Entangled Circuit");

    let builder = || {
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

    println!("{}", HorizontalRenderer::new(&builder()));
    results.push(benchmark_circuit("10-qubit entangled", builder));
}

pub fn test_12_qubit(results: &mut Vec<BenchmarkResult>) {
    print_section("12-qubit Entangled Circuit");

    let builder = || {
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

    println!("{}", HorizontalRenderer::new(&builder()));
    results.push(benchmark_circuit("12-qubit entangled", builder));
}

pub fn test_14_qubit(results: &mut Vec<BenchmarkResult>) {
    print_section("14-qubit Entangled Circuit");

    let builder = || {
        let mut circuit = QuantumCircuit::new(14);
        for i in 0..14 {
            circuit.h(i);
        }
        for i in 0..13 {
            circuit.cnot(i, i + 1);
        }
        circuit
    };

    println!("{}", HorizontalRenderer::new(&builder()));
    results.push(benchmark_circuit("14-qubit entangled", builder));
}

