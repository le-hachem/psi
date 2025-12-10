use crate::{complex, matrix, Complex, Matrix, QuantumGate};
use std::f64::consts::FRAC_1_SQRT_2;

pub fn rx_matrix(theta: f64) -> Matrix<Complex<f64>> {
    let cos = (theta / 2.0).cos();
    let sin = (theta / 2.0).sin();
    matrix!(
        [complex!(cos, 0.0), complex!(0.0, -sin)];
        [complex!(0.0, -sin), complex!(cos, 0.0)]
    )
}

pub fn ry_matrix(theta: f64) -> Matrix<Complex<f64>> {
    let cos = (theta / 2.0).cos();
    let sin = (theta / 2.0).sin();
    matrix!(
        [complex!(cos, 0.0), complex!(-sin, 0.0)];
        [complex!(sin, 0.0), complex!(cos, 0.0)]
    )
}

pub fn rz_matrix(theta: f64) -> Matrix<Complex<f64>> {
    let half = theta / 2.0;
    matrix!(
        [complex!(half.cos(), -half.sin()), complex!(0.0, 0.0)];
        [complex!(0.0, 0.0), complex!(half.cos(), half.sin())]
    )
}

pub fn p_matrix(theta: f64) -> Matrix<Complex<f64>> {
    matrix!(
        [complex!(1.0, 0.0), complex!(0.0, 0.0)];
        [complex!(0.0, 0.0), complex!(theta.cos(), theta.sin())]
    )
}

pub fn u1_matrix(lambda: f64) -> Matrix<Complex<f64>> {
    p_matrix(lambda)
}

pub fn u2_matrix(phi: f64, lambda: f64) -> Matrix<Complex<f64>> {
    let inv_sqrt2 = FRAC_1_SQRT_2;
    matrix!(
        [complex!(inv_sqrt2, 0.0), complex!(-inv_sqrt2 * lambda.cos(), -inv_sqrt2 * lambda.sin())];
        [complex!(inv_sqrt2 * phi.cos(), inv_sqrt2 * phi.sin()), complex!((phi + lambda).cos() * inv_sqrt2, (phi + lambda).sin() * inv_sqrt2)]
    )
}

pub fn u3_matrix(theta: f64, phi: f64, lambda: f64) -> Matrix<Complex<f64>> {
    let cos = (theta / 2.0).cos();
    let sin = (theta / 2.0).sin();
    matrix!(
        [complex!(cos, 0.0), complex!(-sin * lambda.cos(), -sin * lambda.sin())];
        [complex!(sin * phi.cos(), sin * phi.sin()), complex!(cos * (phi + lambda).cos(), cos * (phi + lambda).sin())]
    )
}

pub fn crx_matrix(theta: f64) -> Matrix<Complex<f64>> {
    let cos = (theta / 2.0).cos();
    let sin = (theta / 2.0).sin();
    matrix!(
        [complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
        [complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(cos, 0.0), complex!(0.0, -sin)];
        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, -sin), complex!(cos, 0.0)]
    )
}

pub fn cry_matrix(theta: f64) -> Matrix<Complex<f64>> {
    let cos = (theta / 2.0).cos();
    let sin = (theta / 2.0).sin();
    matrix!(
        [complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
        [complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(cos, 0.0), complex!(-sin, 0.0)];
        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(sin, 0.0), complex!(cos, 0.0)]
    )
}

pub fn crz_matrix(theta: f64) -> Matrix<Complex<f64>> {
    let half = theta / 2.0;
    matrix!(
        [complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
        [complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(half.cos(), -half.sin()), complex!(0.0, 0.0)];
        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(half.cos(), half.sin())]
    )
}

pub fn cp_matrix(theta: f64) -> Matrix<Complex<f64>> {
    matrix!(
        [complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
        [complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0)];
        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(theta.cos(), theta.sin())]
    )
}

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
    
    pub static ref SDG_GATE: QuantumGate<'static> = QuantumGate {
        name: "S†",
        matrix: matrix!([complex!(1.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.0, -1.0)]),
        num_qubits: 1,
    };
    
    pub static ref TDG_GATE: QuantumGate<'static> = QuantumGate {
        name: "T†",
        matrix: matrix!([complex!(1.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(core::f64::consts::FRAC_1_SQRT_2, -core::f64::consts::FRAC_1_SQRT_2)]),
        num_qubits: 1,
    };
    
    pub static ref SX_GATE: QuantumGate<'static> = QuantumGate {
        name: "√X",
        matrix: matrix!([complex!(0.5, 0.5), complex!(0.5, -0.5)];
                        [complex!(0.5, -0.5), complex!(0.5, 0.5)]),
        num_qubits: 1,
    };
    
    pub static ref SXDG_GATE: QuantumGate<'static> = QuantumGate {
        name: "√X†",
        matrix: matrix!([complex!(0.5, -0.5), complex!(0.5, 0.5)];
                        [complex!(0.5, 0.5), complex!(0.5, -0.5)]),
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
