# $\psi$: A Quantum Computational Toolkit
$\psi$ is a powerful quantum computing toolkit designed for simulating quantum circuits on classical computers.

> :warning: **Warning: Work in Progress**

## Project Prospects
- **`libpsi-core`**: A core library for writing and designing quantum circuits, used across all $\psi$ sub-projects. 
    - **`libpsi-core:runtime`**: A set of four runtimes for executing quantum circuits:
        - **`BasicRuntime`**: A single-threaded runtime that executes a quantum circuit statistically, running it $n$ times.
        - **`BasicRuntimeMT`**: A multi-threaded version of `BasicRuntime`, enabling parallel execution.
        - **`WFEvolution`**: A runtime that applies quantum gates by evolving the quantum wave function over time steps $\Delta t$.
        - **`WFEvolutionMT`**: A multi-threaded version of `WFEvolution` for faster execution.
        - **`GPUAccelerated`**: A GPU-accelerated runtime for parallel execution of quantum circuits using [NVIDIA CUDA](https://developer.nvidia.com/cuda-toolkit).
    - **`libpsi-core:maths`**: A comprehensive mathematics library featuring 32/64-bit complex numbers, vectors (both row and column), matrices, and more.
    - **`libpsi-core:core`**: Contains all core quantum components, including quantum gates, classical/quantum bits, and quantum circuits.
- **`libpsi-visualizer`**: An extension of `libpsi-core` that offers visual representations of quantum circuits. It supports both text-based (ASCII) output in the terminal and graphical output using APIs like OpenGL and Vulkan.
- **`libpsi-qasmc`**: An [OpenQASM](https://openqasm.com/) compiler that enables you to write quantum programs, which are then compiled into native executables for classical computers using an [LLVM](https://llvm.org/) backend.
- **`psi`**: A C-like programming language that integrates quantum and classical computing, making it easier to build and simulate quantum systems alongside classical ones.
- **`psi-gui`**: A graphical interface for designing quantum circuits, allowing you to create circuits visually rather than through code. It integrates `libpsi` and `psi`.

## Disclaimer
This project is a large and ongoing effort, and I try my hardest to deliver the advertised feature, some may not arrive as planned or according to any scheduled timeline. The development process is subject to change based on technical challenges, research priorities and the simultaneous management of multiple on-going projects, spanning both computer science, physics and unrelated domains.
