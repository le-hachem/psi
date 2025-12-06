use libpsi_core::*;

fn main() {
    println!("Bell State Creation: |Φ+⟩ = (|00⟩ + |11⟩)/√2");
    println!("   Circuit: H(q0) → CNOT(q0, q1)");
    let mut bell = QuantumCircuit::new(2);
    bell.h(0).cnot(0, 1);
    println!("{}", bell);

    print!("\n------\n\n");

    println!("GHZ State (3-qubit entanglement): |GHZ> = (|000⟩ + |111⟩)/√2");
    println!("   Circuit: H(q0) → CNOT(q0, q1) → CNOT(q0, q2)");
    let mut ghz = QuantumCircuit::new(3);
    ghz.h(0).cnot(0, 1).cnot(0, 2);
    println!("{}", ghz);

    print!("\n------\n\n");

    println!("SWAP via 3 CNOTs");
    println!("   Start with |10>, apply CNOT chain");
    let mut swap_circuit = QuantumCircuit::new(2);
    swap_circuit
        .x(0) // Set to |10⟩
        .cnot(0, 1)
        .cnot(1, 0)
        .cnot(0, 1);
    println!("{}", swap_circuit);

    print!("\n------\n\n");

    println!("Toffoli Gate (Reversible AND)");
    println!("   CCNOT flips q2 only when q0=1 AND q1=1");
    let mut toffoli_circuit = QuantumCircuit::new(3);
    toffoli_circuit
        .x(0)
        .x(1) // Set to |110⟩
        .toffoli(0, 1, 2);
    println!("{}", toffoli_circuit);

    print!("\n------\n\n");

    println!("Fredkin Gate (Controlled SWAP)");
    println!("   CSWAP swaps q1 and q2 only when q0=1");
    let mut fredkin_circuit = QuantumCircuit::new(3);
    fredkin_circuit
        .x(0)
        .x(1) // Set to |110⟩
        .fredkin(0, 1, 2);
    println!("{}", fredkin_circuit);

    print!("\n------\n\n");

    println!("6. Non-contiguous CNOT (q0 controls q2, skipping q1)");
    let mut nc_circuit = QuantumCircuit::new(3);
    nc_circuit
        .x(0) // |100⟩
        .cnot(0, 2); // CNOT with control=q0, target=q2
    println!("{}", nc_circuit);

    print!("\n------\n\n");

    println!("Full Superposition (H on all qubits)");
    let mut super_circuit = QuantumCircuit::new(3);
    super_circuit.h(0).h(1).h(2);
    println!("{}", super_circuit);

    print!("\n------\n\n");

    println!("Probability Test");
    let mut prob_circuit = QuantumCircuit::new(2);
    prob_circuit.h(0).cnot(0, 1);
    println!("   Bell state probabilities:");
    let probs = prob_circuit.probabilities();
    for (i, p) in probs.iter().enumerate() {
        if *p > 1e-10 {
            println!("     |{:02b}⟩: {:.4}", i, p);
        }
    }
    println!();

    print!("\n------\n\n");

    println!("Complex Circuit with Method Chaining");
    let mut complex = QuantumCircuit::new(4);
    complex.h(0).h(1).cnot(0, 2).cnot(1, 3).cz(2, 3).swap(0, 1);
    println!("{}", complex);
}
