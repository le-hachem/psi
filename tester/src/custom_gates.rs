use crate::common::{benchmark_circuit, print_circuit, print_section, BenchmarkResult};
use libpsi_core::{complex, matrix, CustomGate, CustomGateBuilder, QuantumCircuit};

pub fn run_all(results: &mut Vec<BenchmarkResult>) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    CUSTOM GATES TESTS");
    println!("═══════════════════════════════════════════════════════════════\n");

    test_bell_gate(results);
    test_swap_gate(results);
    test_sqrt_x_gate(results);
}

pub fn test_bell_gate(results: &mut Vec<BenchmarkResult>) {
    print_section("Custom Gate: Bell Pair Creator");

    let bell_gate = CustomGateBuilder::new("BELL", 2).h(0).cnot(0, 1).build();
    let gate_clone = bell_gate.clone();

    let builder = move || {
        let mut circuit = QuantumCircuit::new(4);
        circuit
            .apply_custom(gate_clone.clone(), &[0, 1])
            .apply_custom(gate_clone.clone(), &[2, 3]);
        circuit
    };

    let display_circuit = {
        let mut circuit = QuantumCircuit::new(4);
        circuit
            .apply_custom(bell_gate.clone(), &[0, 1])
            .apply_custom(bell_gate.clone(), &[2, 3]);
        circuit
    };
    print_circuit(&display_circuit);
    results.push(benchmark_circuit("Custom BELL (4 qubits)", builder));

    let mut display = {
        let mut circuit = QuantumCircuit::new(4);
        circuit
            .apply_custom(bell_gate.clone(), &[0, 1])
            .apply_custom(bell_gate.clone(), &[2, 3]);
        circuit
    };
    display.compute();
    println!("{}\n", display);
}

pub fn test_swap_gate(results: &mut Vec<BenchmarkResult>) {
    print_section("Custom Gate: Swap via CNOTs");

    let swap_gate = CustomGateBuilder::new("MYSWAP", 2)
        .cnot(0, 1)
        .cnot(1, 0)
        .cnot(0, 1)
        .build();
    let gate_clone = swap_gate.clone();

    let builder = move || {
        let mut circuit = QuantumCircuit::new(2);
        circuit.x(0).apply_custom(gate_clone.clone(), &[0, 1]);
        circuit
    };

    let display_circuit = {
        let mut circuit = QuantumCircuit::new(2);
        circuit.x(0).apply_custom(swap_gate.clone(), &[0, 1]);
        circuit
    };
    print_circuit(&display_circuit);
    results.push(benchmark_circuit("Custom SWAP (2 qubits)", builder));

    let mut display = {
        let mut circuit = QuantumCircuit::new(2);
        circuit.x(0).apply_custom(swap_gate.clone(), &[0, 1]);
        circuit
    };
    display.compute();
    println!("{}\n", display);
}

pub fn test_sqrt_x_gate(results: &mut Vec<BenchmarkResult>) {
    print_section("Custom Gate: Matrix-defined √X gate");

    let sqrt_x_matrix = matrix!(
        [complex!(0.5, 0.5), complex!(0.5, -0.5)];
        [complex!(0.5, -0.5), complex!(0.5, 0.5)]
    );
    let sqrt_x = CustomGate::from_matrix("√X", sqrt_x_matrix);
    let gate_clone = sqrt_x.clone();

    let builder = move || {
        let mut circuit = QuantumCircuit::new(1);
        circuit
            .apply_custom(gate_clone.clone(), &[0])
            .apply_custom(gate_clone.clone(), &[0]);
        circuit
    };

    let display_circuit = {
        let mut circuit = QuantumCircuit::new(1);
        circuit
            .apply_custom(sqrt_x.clone(), &[0])
            .apply_custom(sqrt_x.clone(), &[0]);
        circuit
    };
    print_circuit(&display_circuit);
    results.push(benchmark_circuit("√X gate (1 qubit)", builder));

    let mut display = {
        let mut circuit = QuantumCircuit::new(1);
        circuit
            .apply_custom(sqrt_x.clone(), &[0])
            .apply_custom(sqrt_x.clone(), &[0]);
        circuit
    };
    display.compute();
    println!("{}", display);
    println!("(Two √X gates should equal X, so |0⟩ becomes |1⟩)\n");
}

