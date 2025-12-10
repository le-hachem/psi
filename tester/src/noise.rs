use crate::common::{print_section, BenchmarkResult};
use libpsi_core::{
    complex, DensityMatrix, NoiseChannel, QuantumCircuit, Runtime, Vector,
};
use std::time::Instant;

pub fn run_all(results: &mut Vec<BenchmarkResult>) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    NOISE CHANNEL TESTS");
    println!("═══════════════════════════════════════════════════════════════\n");

    test_density_matrix_basics(results);
    test_noise_channels(results);
    test_noisy_circuit(results);
}

pub fn test_density_matrix_basics(results: &mut Vec<BenchmarkResult>) {
    print_section("Density Matrix Basics");

    let dm = DensityMatrix::new(2);
    println!("Initial |00⟩ state:");
    println!("{}", dm);

    let mut circuit = QuantumCircuit::new(2);
    circuit.h(0).cnot(0, 1);
    circuit.compute_with(Runtime::BasicRT);
    let state = circuit.state();

    let state_vec: Vec<_> = (0..state.size())
        .map(|i| state.get(i))
        .collect();

    let dm_bell = DensityMatrix::from_state_vector(&state_vec);
    println!("Bell state |Φ+⟩:");
    println!("{}", dm_bell);
    println!("Full matrix:");
    println!("{:?}", dm_bell);

    let is_pure = dm_bell.is_pure(1e-10);
    println!("Purity check: {}\n", if is_pure { "✓ Pure" } else { "✗ Mixed" });

    results.push(BenchmarkResult {
        name: "DM: Bell state".to_string(),
        basic_time: std::time::Duration::from_micros(0),
        mt_time: std::time::Duration::from_micros(0),
        results_match: is_pure,
    });
}

pub fn test_noise_channels(results: &mut Vec<BenchmarkResult>) {
    print_section("Noise Channel Effects");

    let channels: Vec<(&str, NoiseChannel)> = vec![
        ("Depolarising (p=0.1)", NoiseChannel::depolarising(0.1)),
        ("Amplitude Damping (γ=0.2)", NoiseChannel::amplitude_damping(0.2)),
        ("Phase Damping (γ=0.2)", NoiseChannel::phase_damping(0.2)),
        ("Bit Flip (p=0.1)", NoiseChannel::bit_flip(0.1)),
        ("Phase Flip (p=0.1)", NoiseChannel::phase_flip(0.1)),
        ("Bit-Phase Flip (p=0.1)", NoiseChannel::bit_phase_flip(0.1)),
    ];

    let plus_state = vec![
        complex!(1.0 / 2.0_f64.sqrt(), 0.0),
        complex!(1.0 / 2.0_f64.sqrt(), 0.0),
    ];

    println!("Starting with |+⟩ state: (|0⟩ + |1⟩)/√2\n");

    for (name, channel) in channels {
        let mut dm = DensityMatrix::from_state_vector(&plus_state);
        let initial_purity = dm.purity();

        let start = Instant::now();
        dm.apply_noise_channel(&channel, 0);
        let elapsed = start.elapsed();

        let final_purity = dm.purity();
        let fidelity = dm.fidelity_with_pure_state(&plus_state);

        println!("{:30}", name);
        println!("  Purity: {:.4} → {:.4}", initial_purity, final_purity);
        println!("  Fidelity with |+⟩: {:.4}", fidelity);
        println!("  Probabilities: {:?}", dm.probabilities());
        println!("  Time: {:.2}μs\n", elapsed.as_secs_f64() * 1_000_000.0);

        let purity_decreased = final_purity <= initial_purity + 1e-10;

        results.push(BenchmarkResult {
            name: format!("Noise: {}", name),
            basic_time: elapsed,
            mt_time: elapsed,
            results_match: purity_decreased,
        });
    }
}

pub fn test_noisy_circuit(results: &mut Vec<BenchmarkResult>) {
    print_section("Noisy Circuit Simulation");

    let mut circuit = QuantumCircuit::new(2);
    circuit.h(0).cnot(0, 1);
    circuit.compute_with(Runtime::BasicRT);
    let state = circuit.state();
    let state_vec: Vec<_> = (0..state.size()).map(|i| state.get(i)).collect();

    let mut dm = DensityMatrix::from_state_vector(&state_vec);
    println!("Bell state before noise:");
    println!("{}", dm);

    let depol = NoiseChannel::depolarising(0.05);

    let start = Instant::now();
    dm.apply_noise_channel(&depol, 0);
    dm.apply_noise_channel(&depol, 1);
    let elapsed = start.elapsed();

    println!("Bell state after 5% depolarising on both qubits:");
    println!("{}", dm);

    let fidelity = dm.fidelity_with_pure_state(&state_vec);
    println!("Fidelity with ideal Bell state: {:.4}", fidelity);
    println!("Time: {:.2}μs\n", elapsed.as_secs_f64() * 1_000_000.0);

    let mut dm2 = DensityMatrix::from_state_vector(&state_vec);
    let amp_damp = NoiseChannel::amplitude_damping(0.1);

    dm2.apply_noise_channel(&amp_damp, 0);
    dm2.apply_noise_channel(&amp_damp, 1);

    println!("Bell state after 10% amplitude damping on both qubits:");
    println!("{}", dm2);
    println!("Probabilities show decay towards |00⟩: {:?}", dm2.probabilities());

    results.push(BenchmarkResult {
        name: "Noisy Bell circuit".to_string(),
        basic_time: elapsed,
        mt_time: elapsed,
        results_match: fidelity > 0.8 && fidelity < 1.0,
    });

    println!();
    print_section("T1/T2 Relaxation Simulation");

    let one_state = vec![complex!(0.0, 0.0), complex!(1.0, 0.0)];
    let mut dm_t1 = DensityMatrix::from_state_vector(&one_state);

    println!("Simulating T1 decay of |1⟩ state:");
    println!("  Initial: P(0)={:.4}, P(1)={:.4}", dm_t1.probabilities()[0], dm_t1.probabilities()[1]);

    let t1_channel = NoiseChannel::amplitude_damping(0.3);
    for step in 1..=5 {
        dm_t1.apply_noise_channel(&t1_channel, 0);
        println!(
            "  Step {}: P(0)={:.4}, P(1)={:.4}, Purity={:.4}",
            step,
            dm_t1.probabilities()[0],
            dm_t1.probabilities()[1],
            dm_t1.purity()
        );
    }

    let decayed = dm_t1.probabilities()[0] > 0.8;
    println!("  Decay complete: {}\n", if decayed { "✓" } else { "✗" });

    results.push(BenchmarkResult {
        name: "T1 decay simulation".to_string(),
        basic_time: std::time::Duration::from_micros(0),
        mt_time: std::time::Duration::from_micros(0),
        results_match: decayed,
    });
}

