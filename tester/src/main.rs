use libpsi_core::*;
use libpsi_visualizer::*;

fn main() {
    println!("Bell State with Measurement\n");
    let mut bell = QuantumCircuit::with_classical(2, 2);
    bell.h(0).cnot(0, 1).measure(0, 0).measure(1, 1);

    println!("Horizontal:");
    println!("{}", HorizontalRenderer::new(&bell));
    println!("Vertical:");
    println!("{}", VerticalRenderer::new(&bell));

    bell.compute();
    println!("{}", bell);

    print!("------\n\n");

    println!("GHZ State\n");
    let mut ghz = QuantumCircuit::new(3);
    ghz.h(0).cnot(0, 1).cnot(0, 2);

    println!("Horizontal:");
    println!("{}", HorizontalRenderer::new(&ghz));
    println!("Vertical:");
    println!("{}", VerticalRenderer::new(&ghz));

    ghz.compute();
    println!("{}", ghz);

    print!("------\n\n");

    println!("SWAP via 3 CNOTs\n");
    let mut swap_circuit = QuantumCircuit::new(2);
    swap_circuit.x(0).cnot(0, 1).cnot(1, 0).cnot(0, 1);

    println!("Horizontal:");
    println!("{}", HorizontalRenderer::new(&swap_circuit));
    println!("Vertical:");
    println!("{}", VerticalRenderer::new(&swap_circuit));

    swap_circuit.compute();
    println!("{}", swap_circuit);

    print!("------\n\n");

    println!("Toffoli Gate\n");
    let mut toffoli_circuit = QuantumCircuit::new(3);
    toffoli_circuit.x(0).x(1).toffoli(0, 1, 2);

    println!("Horizontal:");
    println!("{}", HorizontalRenderer::new(&toffoli_circuit));
    println!("Vertical:");
    println!("{}", VerticalRenderer::new(&toffoli_circuit));

    toffoli_circuit.compute();
    println!("{}", toffoli_circuit);

    print!("------\n\n");

    println!("Full Circuit with Measurements\n");
    let mut full = QuantumCircuit::with_classical(3, 3);
    full.h(0).h(1).h(2).measure_all();

    println!("Horizontal:");
    println!("{}", HorizontalRenderer::new(&full));
    println!("Vertical:");
    println!("{}", VerticalRenderer::new(&full));

    full.compute();
    println!("{}", full);

    print!("------\n\n");

    println!("Complex Circuit\n");
    let mut complex = QuantumCircuit::with_classical(4, 2);
    complex
        .h(0)
        .h(1)
        .cnot(0, 2)
        .cnot(1, 3)
        .cz(2, 3)
        .swap(0, 1)
        .measure(0, 0)
        .measure(1, 1);

    println!("Horizontal:");
    println!("{}", HorizontalRenderer::new(&complex));
    println!("Vertical:");
    println!("{}", VerticalRenderer::new(&complex));

    complex.compute();
    println!("{}", complex);
}
