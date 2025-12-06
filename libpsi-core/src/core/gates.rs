use crate::{complex, matrix, QuantumGate};

#[rustfmt::skip]
lazy_static::lazy_static! {
    pub static ref HADAMARD: QuantumGate<'static> = QuantumGate {
        name: "H",
        matrix: matrix!([complex!(1.0, 0.0), complex!( 1.0, 0.0)];
                        [complex!(1.0, 0.0), complex!(-1.0, 0.0)]) *
                complex!(1.0/2.0_f64.sqrt(), 0.0),
        num_qubits: 1,
    };

    pub static ref PAULI_X: QuantumGate<'static> = QuantumGate {
        name: "X",
        matrix: matrix!([complex!(0.0, 0.0), complex!(1.0, 0.0)];
                        [complex!(1.0, 0.0), complex!(0.0, 0.0)]),
        num_qubits: 1,
    };

    pub static ref PAULI_Y: QuantumGate<'static> = QuantumGate {
        name: "Y", 
        matrix: matrix!([complex!(0.0, 0.0), complex!(0.0, -1.0)];
                        [complex!(0.0, 1.0), complex!(0.0,  0.0)]),
        num_qubits: 1,
    };

    pub static ref PAULI_Z: QuantumGate<'static> = QuantumGate {
        name: "Z", 
        matrix: matrix!([complex!(1.0, 0.0), complex!( 0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(-1.0, 0.0)]),
        num_qubits: 1,
    };
    
    pub static ref S_GATE: QuantumGate<'static> = QuantumGate {
        name: "S",
        matrix: matrix!([complex!(1.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.0, 1.0)]),
        num_qubits: 1,
    };
    
    pub static ref T_GATE: QuantumGate<'static> = QuantumGate {
        name: "T",
        matrix: matrix!([complex!(1.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(core::f64::consts::FRAC_1_SQRT_2, core::f64::consts::FRAC_1_SQRT_2)]),
        num_qubits: 1,
    };
    
    pub static ref IDENTITY: QuantumGate<'static> = QuantumGate {
        name: "I",
        matrix: matrix!([complex!(1.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(1.0, 0.0)]),
        num_qubits: 1,
    };

    pub static ref CNOT: QuantumGate<'static> = QuantumGate {
        name: "CNOT", 
        matrix: matrix!([complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0)]),
        num_qubits: 2,
    };
    
    pub static ref CZ: QuantumGate<'static> = QuantumGate {
        name: "CZ",
        matrix: matrix!([complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!( 0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!( 0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!( 0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(-1.0, 0.0)]),
        num_qubits: 2,
    };
    
    pub static ref SWAP: QuantumGate<'static> = QuantumGate {
        name: "SWAP",
        matrix: matrix!([complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0)]),
        num_qubits: 2,
    };
    
    pub static ref ISWAP: QuantumGate<'static> = QuantumGate {
        name: "iSWAP",
        matrix: matrix!([complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 1.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.0, 1.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0)]),
        num_qubits: 2,
    };
    
    pub static ref SQRT_SWAP: QuantumGate<'static> = QuantumGate {
        name: "√SWAP",
        matrix: matrix!([complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.5, 0.5), complex!(0.5, -0.5), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.5, -0.5), complex!(0.5, 0.5), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0)]),
        num_qubits: 2,
    };

    pub static ref TOFFOLI: QuantumGate<'static> = QuantumGate {
        name: "CCNOT",
        matrix: matrix!(
            [complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
            [complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
            [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
            [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
            [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
            [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
            [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0)];
            [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0)]
        ),
        num_qubits: 3,
    };
    
    pub static ref FREDKIN: QuantumGate<'static> = QuantumGate {
        name: "CSWAP",
        matrix: matrix!(
            [complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
            [complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
            [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
            [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
            [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
            [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0)];
            [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
            [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0)]
        ),
        num_qubits: 3,
    };
}
