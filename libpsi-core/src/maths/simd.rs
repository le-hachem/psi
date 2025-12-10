use crate::{complex, Complex};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdCapability {
    None,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    Avx2,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    Avx512,
    #[cfg(target_arch = "aarch64")]
    Neon,
}

impl SimdCapability {
    pub fn detect() -> Self {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        {
            if is_x86_feature_detected!("avx512f") && is_x86_feature_detected!("avx512dq") {
                return SimdCapability::Avx512;
            }
            if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
                return SimdCapability::Avx2;
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            return SimdCapability::Neon;
        }

        #[allow(unreachable_code)]
        SimdCapability::None
    }

    pub fn name(&self) -> &'static str {
        match self {
            SimdCapability::None => "Scalar",
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            SimdCapability::Avx2 => "AVX2+FMA",
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            SimdCapability::Avx512 => "AVX-512",
            #[cfg(target_arch = "aarch64")]
            SimdCapability::Neon => "NEON",
        }
    }
}

pub fn apply_single_qubit_gate_simd(
    state: &mut [Complex<f64>],
    gate: &[[Complex<f64>; 2]; 2],
    target: usize,
    num_qubits: usize,
) {
    let capability = SimdCapability::detect();

    match capability {
        #[cfg(target_arch = "x86_64")]
        SimdCapability::Avx2 => unsafe {
            apply_single_qubit_avx2(state, gate, target, num_qubits);
        },
        #[cfg(target_arch = "x86_64")]
        SimdCapability::Avx512 => unsafe {
            apply_single_qubit_avx512(state, gate, target, num_qubits);
        },
        #[cfg(target_arch = "aarch64")]
        SimdCapability::Neon => unsafe {
            apply_single_qubit_neon(state, gate, target, num_qubits);
        },
        _ => {
            apply_single_qubit_scalar(state, gate, target, num_qubits);
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2", enable = "fma")]
unsafe fn apply_single_qubit_avx2(
    state: &mut [Complex<f64>],
    gate: &[[Complex<f64>; 2]; 2],
    target: usize,
    num_qubits: usize,
) {
    let target_bit = num_qubits - 1 - target;
    let step = 1 << target_bit;
    let dim = 1 << num_qubits;

    let g00 = gate[0][0];
    let g01 = gate[0][1];
    let g10 = gate[1][0];
    let g11 = gate[1][1];

    let pairs: Vec<(usize, usize)> = (0..dim)
        .filter(|&i| (i >> target_bit) & 1 == 0)
        .map(|i| (i, i | step))
        .collect();

    let chunks = pairs.len() / 2;

    for chunk_idx in 0..chunks {
        let (i0, j0) = pairs[chunk_idx * 2];
        let (i1, j1) = pairs[chunk_idx * 2 + 1];

        let s0_re = _mm256_set_pd(
            state[j1].real,
            state[i1].real,
            state[j0].real,
            state[i0].real,
        );
        let s0_im = _mm256_set_pd(
            state[j1].imaginary,
            state[i1].imaginary,
            state[j0].imaginary,
            state[i0].imaginary,
        );

        let g_re_0 = _mm256_set_pd(g01.real, g00.real, g01.real, g00.real);
        let g_im_0 = _mm256_set_pd(g01.imaginary, g00.imaginary, g01.imaginary, g00.imaginary);
        let g_re_1 = _mm256_set_pd(g11.real, g10.real, g11.real, g10.real);
        let g_im_1 = _mm256_set_pd(g11.imaginary, g10.imaginary, g11.imaginary, g10.imaginary);

        let prod0_re = _mm256_fmsub_pd(s0_re, g_re_0, _mm256_mul_pd(s0_im, g_im_0));
        let prod0_im = _mm256_fmadd_pd(s0_re, g_im_0, _mm256_mul_pd(s0_im, g_re_0));

        let prod1_re = _mm256_fmsub_pd(s0_re, g_re_1, _mm256_mul_pd(s0_im, g_im_1));
        let prod1_im = _mm256_fmadd_pd(s0_re, g_im_1, _mm256_mul_pd(s0_im, g_re_1));

        let mut res0_re = [0.0f64; 4];
        let mut res0_im = [0.0f64; 4];
        let mut res1_re = [0.0f64; 4];
        let mut res1_im = [0.0f64; 4];

        _mm256_storeu_pd(res0_re.as_mut_ptr(), prod0_re);
        _mm256_storeu_pd(res0_im.as_mut_ptr(), prod0_im);
        _mm256_storeu_pd(res1_re.as_mut_ptr(), prod1_re);
        _mm256_storeu_pd(res1_im.as_mut_ptr(), prod1_im);

        state[i0] = complex!(res0_re[0] + res0_re[1], res0_im[0] + res0_im[1]);
        state[j0] = complex!(res1_re[0] + res1_re[1], res1_im[0] + res1_im[1]);
        state[i1] = complex!(res0_re[2] + res0_re[3], res0_im[2] + res0_im[3]);
        state[j1] = complex!(res1_re[2] + res1_re[3], res1_im[2] + res1_im[3]);
    }

    for &(i, j) in pairs.iter().skip(chunks * 2) {
        let s0 = state[i];
        let s1 = state[j];

        let new0 = complex!(
            s0.real * g00.real - s0.imaginary * g00.imaginary + s1.real * g01.real
                - s1.imaginary * g01.imaginary,
            s0.real * g00.imaginary
                + s0.imaginary * g00.real
                + s1.real * g01.imaginary
                + s1.imaginary * g01.real
        );

        let new1 = complex!(
            s0.real * g10.real - s0.imaginary * g10.imaginary + s1.real * g11.real
                - s1.imaginary * g11.imaginary,
            s0.real * g10.imaginary
                + s0.imaginary * g10.real
                + s1.real * g11.imaginary
                + s1.imaginary * g11.real
        );

        state[i] = new0;
        state[j] = new1;
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f", enable = "avx512dq")]
unsafe fn apply_single_qubit_avx512(
    state: &mut [Complex<f64>],
    gate: &[[Complex<f64>; 2]; 2],
    target: usize,
    num_qubits: usize,
) {
    let target_bit = num_qubits - 1 - target;
    let step = 1 << target_bit;
    let dim = 1 << num_qubits;

    let g00 = gate[0][0];
    let g01 = gate[0][1];
    let g10 = gate[1][0];
    let g11 = gate[1][1];

    let pairs: Vec<(usize, usize)> = (0..dim)
        .filter(|&i| (i >> target_bit) & 1 == 0)
        .map(|i| (i, i | step))
        .collect();

    let chunks = pairs.len() / 4;

    for chunk_idx in 0..chunks {
        let base = chunk_idx * 4;
        let (i0, j0) = pairs[base];
        let (i1, j1) = pairs[base + 1];
        let (i2, j2) = pairs[base + 2];
        let (i3, j3) = pairs[base + 3];

        let s0_re = _mm512_set_pd(
            state[j3].real,
            state[i3].real,
            state[j2].real,
            state[i2].real,
            state[j1].real,
            state[i1].real,
            state[j0].real,
            state[i0].real,
        );
        let s0_im = _mm512_set_pd(
            state[j3].imaginary,
            state[i3].imaginary,
            state[j2].imaginary,
            state[i2].imaginary,
            state[j1].imaginary,
            state[i1].imaginary,
            state[j0].imaginary,
            state[i0].imaginary,
        );

        let g_re_0 = _mm512_set_pd(
            g01.real, g00.real, g01.real, g00.real, g01.real, g00.real, g01.real, g00.real,
        );
        let g_im_0 = _mm512_set_pd(
            g01.imaginary,
            g00.imaginary,
            g01.imaginary,
            g00.imaginary,
            g01.imaginary,
            g00.imaginary,
            g01.imaginary,
            g00.imaginary,
        );
        let g_re_1 = _mm512_set_pd(
            g11.real, g10.real, g11.real, g10.real, g11.real, g10.real, g11.real, g10.real,
        );
        let g_im_1 = _mm512_set_pd(
            g11.imaginary,
            g10.imaginary,
            g11.imaginary,
            g10.imaginary,
            g11.imaginary,
            g10.imaginary,
            g11.imaginary,
            g10.imaginary,
        );

        let prod0_re = _mm512_fmsub_pd(s0_re, g_re_0, _mm512_mul_pd(s0_im, g_im_0));
        let prod0_im = _mm512_fmadd_pd(s0_re, g_im_0, _mm512_mul_pd(s0_im, g_re_0));
        let prod1_re = _mm512_fmsub_pd(s0_re, g_re_1, _mm512_mul_pd(s0_im, g_im_1));
        let prod1_im = _mm512_fmadd_pd(s0_re, g_im_1, _mm512_mul_pd(s0_im, g_re_1));

        let mut res0_re = [0.0f64; 8];
        let mut res0_im = [0.0f64; 8];
        let mut res1_re = [0.0f64; 8];
        let mut res1_im = [0.0f64; 8];

        _mm512_storeu_pd(res0_re.as_mut_ptr(), prod0_re);
        _mm512_storeu_pd(res0_im.as_mut_ptr(), prod0_im);
        _mm512_storeu_pd(res1_re.as_mut_ptr(), prod1_re);
        _mm512_storeu_pd(res1_im.as_mut_ptr(), prod1_im);

        state[i0] = complex!(res0_re[0] + res0_re[1], res0_im[0] + res0_im[1]);
        state[j0] = complex!(res1_re[0] + res1_re[1], res1_im[0] + res1_im[1]);
        state[i1] = complex!(res0_re[2] + res0_re[3], res0_im[2] + res0_im[3]);
        state[j1] = complex!(res1_re[2] + res1_re[3], res1_im[2] + res1_im[3]);
        state[i2] = complex!(res0_re[4] + res0_re[5], res0_im[4] + res0_im[5]);
        state[j2] = complex!(res1_re[4] + res1_re[5], res1_im[4] + res1_im[5]);
        state[i3] = complex!(res0_re[6] + res0_re[7], res0_im[6] + res0_im[7]);
        state[j3] = complex!(res1_re[6] + res1_re[7], res1_im[6] + res1_im[7]);
    }

    for &(i, j) in pairs.iter().skip(chunks * 4) {
        let s0 = state[i];
        let s1 = state[j];

        let new0 = complex!(
            s0.real * g00.real - s0.imaginary * g00.imaginary + s1.real * g01.real
                - s1.imaginary * g01.imaginary,
            s0.real * g00.imaginary
                + s0.imaginary * g00.real
                + s1.real * g01.imaginary
                + s1.imaginary * g01.real
        );

        let new1 = complex!(
            s0.real * g10.real - s0.imaginary * g10.imaginary + s1.real * g11.real
                - s1.imaginary * g11.imaginary,
            s0.real * g10.imaginary
                + s0.imaginary * g10.real
                + s1.real * g11.imaginary
                + s1.imaginary * g11.real
        );

        state[i] = new0;
        state[j] = new1;
    }
}

#[cfg(target_arch = "aarch64")]
unsafe fn apply_single_qubit_neon(
    state: &mut [Complex<f64>],
    gate: &[[Complex<f64>; 2]; 2],
    target: usize,
    num_qubits: usize,
) {
    let target_bit = num_qubits - 1 - target;
    let step = 1 << target_bit;
    let dim = 1 << num_qubits;

    let g00 = gate[0][0];
    let g01 = gate[0][1];
    let g10 = gate[1][0];
    let g11 = gate[1][1];

    let pairs: Vec<(usize, usize)> = (0..dim)
        .filter(|&i| (i >> target_bit) & 1 == 0)
        .map(|i| (i, i | step))
        .collect();

    let chunks = pairs.len() / 2;

    for chunk_idx in 0..chunks {
        let (i0, j0) = pairs[chunk_idx * 2];
        let (i1, j1) = pairs[chunk_idx * 2 + 1];

        let s0_0 = state[i0];
        let s1_0 = state[j0];
        let s0_1 = state[i1];
        let s1_1 = state[j1];

        let s0_re = vld1q_f64([s0_0.real, s0_1.real].as_ptr());
        let s0_im = vld1q_f64([s0_0.imaginary, s0_1.imaginary].as_ptr());
        let s1_re = vld1q_f64([s1_0.real, s1_1.real].as_ptr());
        let s1_im = vld1q_f64([s1_0.imaginary, s1_1.imaginary].as_ptr());

        let g00_re = vdupq_n_f64(g00.real);
        let g00_im = vdupq_n_f64(g00.imaginary);
        let g01_re = vdupq_n_f64(g01.real);
        let g01_im = vdupq_n_f64(g01.imaginary);
        let g10_re = vdupq_n_f64(g10.real);
        let g10_im = vdupq_n_f64(g10.imaginary);
        let g11_re = vdupq_n_f64(g11.real);
        let g11_im = vdupq_n_f64(g11.imaginary);

        let new0_re = vaddq_f64(
            vfmsq_f64(vmulq_f64(s0_re, g00_re), s0_im, g00_im),
            vfmsq_f64(vmulq_f64(s1_re, g01_re), s1_im, g01_im),
        );
        let new0_im = vaddq_f64(
            vfmaq_f64(vmulq_f64(s0_re, g00_im), s0_im, g00_re),
            vfmaq_f64(vmulq_f64(s1_re, g01_im), s1_im, g01_re),
        );

        let new1_re = vaddq_f64(
            vfmsq_f64(vmulq_f64(s0_re, g10_re), s0_im, g10_im),
            vfmsq_f64(vmulq_f64(s1_re, g11_re), s1_im, g11_im),
        );
        let new1_im = vaddq_f64(
            vfmaq_f64(vmulq_f64(s0_re, g10_im), s0_im, g10_re),
            vfmaq_f64(vmulq_f64(s1_re, g11_im), s1_im, g11_re),
        );

        state[i0] = complex!(vgetq_lane_f64(new0_re, 0), vgetq_lane_f64(new0_im, 0));
        state[j0] = complex!(vgetq_lane_f64(new1_re, 0), vgetq_lane_f64(new1_im, 0));
        state[i1] = complex!(vgetq_lane_f64(new0_re, 1), vgetq_lane_f64(new0_im, 1));
        state[j1] = complex!(vgetq_lane_f64(new1_re, 1), vgetq_lane_f64(new1_im, 1));
    }

    for &(i, j) in pairs.iter().skip(chunks * 2) {
        let s0 = state[i];
        let s1 = state[j];

        let new0 = complex!(
            s0.real * g00.real - s0.imaginary * g00.imaginary + s1.real * g01.real
                - s1.imaginary * g01.imaginary,
            s0.real * g00.imaginary
                + s0.imaginary * g00.real
                + s1.real * g01.imaginary
                + s1.imaginary * g01.real
        );

        let new1 = complex!(
            s0.real * g10.real - s0.imaginary * g10.imaginary + s1.real * g11.real
                - s1.imaginary * g11.imaginary,
            s0.real * g10.imaginary
                + s0.imaginary * g10.real
                + s1.real * g11.imaginary
                + s1.imaginary * g11.real
        );

        state[i] = new0;
        state[j] = new1;
    }
}

fn apply_single_qubit_scalar(
    state: &mut [Complex<f64>],
    gate: &[[Complex<f64>; 2]; 2],
    target: usize,
    num_qubits: usize,
) {
    let target_bit = num_qubits - 1 - target;
    let step = 1 << target_bit;
    let dim = 1 << num_qubits;

    let g00 = gate[0][0];
    let g01 = gate[0][1];
    let g10 = gate[1][0];
    let g11 = gate[1][1];

    for i in 0..dim {
        if (i >> target_bit) & 1 == 1 {
            continue;
        }

        let j = i | step;
        let s0 = state[i];
        let s1 = state[j];

        let new0 = complex!(
            s0.real * g00.real - s0.imaginary * g00.imaginary + s1.real * g01.real
                - s1.imaginary * g01.imaginary,
            s0.real * g00.imaginary
                + s0.imaginary * g00.real
                + s1.real * g01.imaginary
                + s1.imaginary * g01.real
        );

        let new1 = complex!(
            s0.real * g10.real - s0.imaginary * g10.imaginary + s1.real * g11.real
                - s1.imaginary * g11.imaginary,
            s0.real * g10.imaginary
                + s0.imaginary * g10.real
                + s1.real * g11.imaginary
                + s1.imaginary * g11.real
        );

        state[i] = new0;
        state[j] = new1;
    }
}

pub fn apply_single_qubit_gate_simd_parallel(
    state: &mut [Complex<f64>],
    gate: &[[Complex<f64>; 2]; 2],
    target: usize,
    num_qubits: usize,
) {
    use rayon::prelude::*;

    let target_bit = num_qubits - 1 - target;
    let step = 1 << target_bit;
    let dim = 1 << num_qubits;

    let g00 = gate[0][0];
    let g01 = gate[0][1];
    let g10 = gate[1][0];
    let g11 = gate[1][1];

    let pairs: Vec<(usize, usize)> = (0..dim)
        .filter(|&i| (i >> target_bit) & 1 == 0)
        .map(|i| (i, i | step))
        .collect();

    let results: Vec<(usize, usize, Complex<f64>, Complex<f64>)> = pairs
        .par_iter()
        .map(|&(i, j)| {
            let s0 = state[i];
            let s1 = state[j];

            let new0 = complex!(
                s0.real * g00.real - s0.imaginary * g00.imaginary + s1.real * g01.real
                    - s1.imaginary * g01.imaginary,
                s0.real * g00.imaginary
                    + s0.imaginary * g00.real
                    + s1.real * g01.imaginary
                    + s1.imaginary * g01.real
            );

            let new1 = complex!(
                s0.real * g10.real - s0.imaginary * g10.imaginary + s1.real * g11.real
                    - s1.imaginary * g11.imaginary,
                s0.real * g10.imaginary
                    + s0.imaginary * g10.real
                    + s1.real * g11.imaginary
                    + s1.imaginary * g11.real
            );

            (i, j, new0, new1)
        })
        .collect();

    for (i, j, new0, new1) in results {
        state[i] = new0;
        state[j] = new1;
    }
}

pub fn get_simd_info() -> String {
    let cap = SimdCapability::detect();
    format!("SIMD: {}", cap.name())
}
