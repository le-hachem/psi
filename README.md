# $\psi$: A Quantum Computational Toolkit

$\psi$ is a powerful quantum computing toolkit designed for simulating quantum circuits and wavefunction dynamics on classical hardware.

## Features

### Quantum Gates

**Clifford Gates:**
- Single-qubit: H, X, Y, Z, S
- Two-qubit: CNOT, CZ, SWAP
- Three-qubit: CCNOT (Toffoli), CSWAP (Fredkin)

**Non-Clifford Gates:**
- Fixed: T, $S^\dagger$, $T^\dagger$, $\sqrt{X}$, $\sqrt{X}^\dagger$
- Parametric rotations: $R_x(\theta)$, $R_y(\theta)$, $R_z(\theta)$, $P(\theta)$
- General unitaries: $U_1(\lambda)$, $U_2(\phi, \lambda)$, $U_3(\theta, \phi, \lambda)$
- Controlled parametric: $CR_x(\theta)$, $CR_y(\theta)$, $CR_z(\theta)$, $CP(\theta)$

**Custom Gates:**
- Define gates from unitary matrices
- Build composite gates from sequences of operations

### Runtimes

| Runtime | Description |
|---------|-------------|
| `BasicRT` | Single-threaded state vector simulation |
| `BasicRTMT` | Multi-threaded parallel simulation |
| `BatchedRT` | Kernel batching with gate fusion optimisation |
| `BatchedRTMT` | Multi-threaded kernel batching |
| `SimdRT` | SIMD-accelerated simulation (AVX2/AVX-512/NEON) |
| `SimdRTMT` | Multi-threaded SIMD acceleration |
| `WFEvolution` | Wave function time evolution (planned) |
| `GPUAccelerated` | CUDA GPU acceleration (planned) |

### SIMD Acceleration

Automatic detection and use of platform-specific SIMD instructions:
- **AVX-512**: Modern Intel/AMD processors
- **AVX2+FMA**: Older x86_64 processors  
- **NEON**: ARM processors (Apple Silicon, etc.)
- **Scalar fallback**: Universal compatibility

### Kernel Batching

Optimisation system that:
- Groups consecutive single-qubit gates on the same qubit
- Fuses gate matrices to reduce operations
- Typically achieves 30–50% kernel reduction and significantly reduces memory bandwidth pressure.

## Project Structure

- **`libpsi-core`**: Core quantum simulation library
  - `core`: Quantum gates, circuits, registers, and runtimes
  - `maths`: Complex numbers, vectors, matrices, SIMD operations
- **`libpsi-visualizer`**: Circuit visualisation (ASCII horizontal/vertical)
- **`tester`**: Comprehensive test suite and benchmarks

## Quick Start

```rust
use libpsi_core::{QuantumCircuit, Runtime};
use std::f64::consts::PI;

fn main() {
    // Create a 3-qubit circuit
    let mut circuit = QuantumCircuit::new(3);
    
    // Build a GHZ state
    circuit
        .h(0)
        .cnot(0, 1)
        .cnot(0, 2);
    
    // Execute with SIMD acceleration
    circuit.compute_with(Runtime::SimdRT);
    
    // Print the quantum state
    println!("{}", circuit.state());
}
```

### Parametric Gates

```rust
let mut circuit = QuantumCircuit::new(2);
circuit
    .rx(0, PI / 4.0)      // Rotate around X
    .ry(0, PI / 3.0)      // Rotate around Y
    .rz(1, PI / 2.0)      // Rotate around Z
    .crz(0, 1, PI / 4.0); // Controlled-Rz
```

### Custom Gates

```rust
use libpsi_core::{CustomGateBuilder, complex, matrix};

// Build from operations
let bell_gate = CustomGateBuilder::new("BELL", 2)
    .h(0)
    .cnot(0, 1)
    .build();

// Or from a matrix
let sqrt_x_matrix = matrix!(
    [complex!(0.5, 0.5), complex!(0.5, -0.5)];
    [complex!(0.5, -0.5), complex!(0.5, 0.5)]
);
let sqrt_x = CustomGate::from_matrix("√X", sqrt_x_matrix);
```

## Running Tests

```bash
# Run all tests
cargo run --package tester --release

# Run specific test modules
cargo run --package tester --release -- clifford
cargo run --package tester --release -- non-clifford
cargo run --package tester --release -- kernels
cargo run --package tester --release -- simd
cargo run --package tester --release -- bench

# Show help
cargo run --package tester --release -- help
```

## Disclaimer

This project is under active development. Features and APIs may change. Some planned features may not arrive as scheduled due to technical challenges and research priorities.

## License

This project is made available under the Apache License, Version 2.0, allowing free use, modification, and distribution with proper attribution. Community contributions, improvements, and research collaborations are encouraged. Full licensing terms can be found in [LICENSE](LICENSE).