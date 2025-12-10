use crate::common::{benchmark_circuit, print_circuit, print_section, BenchmarkResult};
use libpsi_core::QuantumCircuit;

pub fn run_all(results: &mut Vec<BenchmarkResult>) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("                     CLIFFORD GATES TESTS");
    println!("═══════════════════════════════════════════════════════════════\n");

    test_bell_state(results);
    test_ghz_state(results);
    test_swap_via_cnots(results);
    test_toffoli(results);
    test_hadamard_measure(results);
    test_complex_circuit(results);
}

pub fn test_bell_state(results: &mut Vec<BenchmarkResult>) {
    print_section("Bell State with Measurement");

    let builder = || {
        let mut circuit = QuantumCircuit::with_classical(2, 2);
        circuit.h(0).cnot(0, 1).measure(0, 0).measure(1, 1);
        circuit
    };

    print_circuit(&builder());
    results.push(benchmark_circuit("Bell State (2 qubits)", builder));

    let mut display = builder();
    display.compute();
    println!("{}\n", display);
}

pub fn test_ghz_state(results: &mut Vec<BenchmarkResult>) {
    print_section("GHZ State");

    let builder = || {
        let mut circuit = QuantumCircuit::new(3);
        circuit.h(0).cnot(0, 1).cnot(0, 2);
        circuit
    };

    print_circuit(&builder());
    results.push(benchmark_circuit("GHZ State (3 qubits)", builder));

    let mut display = builder();
    display.compute();
    println!("{}\n", display);
}

pub fn test_swap_via_cnots(results: &mut Vec<BenchmarkResult>) {
    print_section("SWAP via 3 CNOTs");

    let builder = || {
        let mut circuit = QuantumCircuit::new(2);
        circuit.x(0).cnot(0, 1).cnot(1, 0).cnot(0, 1);
        circuit
    };

    print_circuit(&builder());
    results.push(benchmark_circuit("SWAP via CNOTs (2 qubits)", builder));

    let mut display = builder();
    display.compute();
    println!("{}\n", display);
}

pub fn test_toffoli(results: &mut Vec<BenchmarkResult>) {
    print_section("Toffoli Gate");

    let builder = || {
        let mut circuit = QuantumCircuit::new(3);
        circuit.x(0).x(1).toffoli(0, 1, 2);
        circuit
    };

    print_circuit(&builder());
    results.push(benchmark_circuit("Toffoli (3 qubits)", builder));

    let mut display = builder();
    display.compute();
    println!("{}\n", display);
}

pub fn test_hadamard_measure(results: &mut Vec<BenchmarkResult>) {
    print_section("Full Circuit with Measurements");

    let builder = || {
        let mut circuit = QuantumCircuit::with_classical(3, 3);
        circuit.h(0).h(1).h(2).measure_all();
        circuit
    };

    print_circuit(&builder());
    results.push(benchmark_circuit("3-qubit Hadamard + Measure", builder));

    let mut display = builder();
    display.compute();
    println!("{}\n", display);
}

pub fn test_complex_circuit(results: &mut Vec<BenchmarkResult>) {
    print_section("Complex Circuit");

    let builder = || {
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

    print_circuit(&builder());
    results.push(benchmark_circuit("Complex (4 qubits)", builder));

    let mut display = builder();
    display.compute();
    println!("{}\n", display);
}

