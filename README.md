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

### Composable Runtime System

Build custom execution pipelines by combining optimisation features:

| Feature | Description |
|---------|-------------|
| `.batched()` | Kernel batching with gate fusion |
| `.simd()` | SIMD acceleration (AVX-512/AVX2/NEON) |
| `.structure_aware()` | Commutation analysis and advanced fusion |
| `.parallel()` | Multi-threaded execution |
| `.with_threshold(n)` | Set parallel threshold (default: 8 qubits) |

**Predefined Runtimes:**
- `Runtime::BasicRT` / `BasicRTMT` — Direct state vector simulation
- `Runtime::BatchedRT` / `BatchedRTMT` — Batched kernel execution
- `Runtime::SimdRT` / `SimdRTMT` — Batched + SIMD
- `Runtime::StructureAwareRT` / `StructureAwareMT` — Structure-aware + SIMD
- `Runtime::optimal()` — Structure-aware + SIMD + parallel

### SIMD Acceleration

Automatic detection and use of platform-specific SIMD instructions:
- **AVX-512**: Modern Intel/AMD processors
- **AVX2+FMA**: Older x86_64 processors  
- **NEON**: ARM processors (Apple Silicon, etc.)
- **Scalar fallback**: Universal compatibility

### Kernel Optimisations

**Batching:**
- Groups consecutive single-qubit gates on the same qubit
- Fuses gate matrices to reduce operations
- Typically achieves 30–50% kernel reduction

**Structure-Aware:**
- Gate type detection (diagonal, non-diagonal, controlled)
- Commutation analysis for reordering
- Multi-pass fusion until convergence
- Execution layer grouping for parallelism

## Project Structure

- **`libpsi-core`**: Core quantum simulation library
  - `core`: Quantum gates, circuits, registers, and runtimes
  - `maths`: Complex numbers, vectors, matrices, SIMD operations
- **`libpsi-visualizer`**: Circuit visualisation (ASCII horizontal/vertical)
- **`tester`**: Comprehensive test suite and benchmarks

## Quick Start

```rust
use libpsi_core::{QuantumCircuit, Runtime};

fn main() {
    let mut circuit = QuantumCircuit::new(3);
    
    // Build a GHZ state
    circuit.h(0).cnot(0, 1).cnot(0, 2);
    
    // Execute with optimal settings
    circuit.compute_with_config(Runtime::optimal());
    
    println!("{}", circuit.state());
}
```

### Composable Runtimes

```rust
use libpsi_core::{QuantumCircuit, RuntimeConfig};

let mut circuit = QuantumCircuit::new(8);
// ... add gates ...

// Combine features as needed
let config = RuntimeConfig::new()
    .structure_aware()
    .simd()
    .parallel();

circuit.compute_with_config(config);
```

### Parametric Gates

```rust
use std::f64::consts::PI;

circuit
    .rx(0, PI / 4.0)       // Rotation around X
    .ry(0, PI / 3.0)       // Rotation around Y
    .rz(1, PI / 2.0)       // Rotation around Z
    .crz(0, 1, PI / 4.0);  // Controlled-Rz
```

### Custom Gates

```rust
use libpsi_core::{CustomGateBuilder, CustomGate, complex, matrix};

// From operations
let bell_gate = CustomGateBuilder::new("BELL", 2)
    .h(0)
    .cnot(0, 1)
    .build();

// From a unitary matrix
let sqrt_x_matrix = matrix!(
    [complex!(0.5, 0.5), complex!(0.5, -0.5)];
    [complex!(0.5, -0.5), complex!(0.5, 0.5)]
);
let sqrt_x = CustomGate::from_matrix("√X", sqrt_x_matrix);
```

## Running Tests

```bash
cargo run --package tester --release           # All tests
cargo run --package tester --release -- clifford
cargo run --package tester --release -- non-clifford
cargo run --package tester --release -- kernels
cargo run --package tester --release -- simd
cargo run --package tester --release -- bench
cargo run --package tester --release -- help
```

## Disclaimer

This project is under active development. Features and APIs may change.

## License

This project is made available under the Apache License, Version 2.0, allowing free use, modification, and distribution with proper attribution. Community contributions, improvements, and research collaborations are encouraged. Full licensing terms can be found in [LICENSE](LICENSE).
