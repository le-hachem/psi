use crate::Complex;

const EPSILON: f64 = 1e-10;
const SQRT_2: f64 = 1.4142135623730951;
const INV_SQRT_2: f64 = 0.7071067811865475;
const INV_SQRT_8: f64 = 0.3535533905932738;
const INV_SQRT_32: f64 = 0.1767766952966369;

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

fn format_real_symbolic(v: f64) -> Option<String> {
    let abs_v = v.abs();
    let sign = if v < 0.0 { "-" } else { "" };

    if approx_eq(abs_v, 0.0) {
        return Some("0".to_string());
    }
    if approx_eq(abs_v, 1.0) {
        return Some(format!("{}1", sign));
    }
    if approx_eq(abs_v, 0.5) {
        return Some(format!("{}½", sign));
    }
    if approx_eq(abs_v, 0.25) {
        return Some(format!("{}¼", sign));
    }
    if approx_eq(abs_v, 0.75) {
        return Some(format!("{}¾", sign));
    }
    if approx_eq(abs_v, 0.125) {
        return Some(format!("{}⅛", sign));
    }
    if approx_eq(abs_v, SQRT_2) {
        return Some(format!("{}√2", sign));
    }
    if approx_eq(abs_v, INV_SQRT_2) {
        return Some(format!("{}¹⁄√2", sign));
    }
    if approx_eq(abs_v, INV_SQRT_8) {
        return Some(format!("{}¹⁄√8", sign));
    }
    if approx_eq(abs_v, INV_SQRT_32) {
        return Some(format!("{}¹⁄√32", sign));
    }
    if approx_eq(abs_v, 2.0) {
        return Some(format!("{}2", sign));
    }
    if approx_eq(abs_v, 1.0 / 3.0) {
        return Some(format!("{}⅓", sign));
    }
    if approx_eq(abs_v, 2.0 / 3.0) {
        return Some(format!("{}⅔", sign));
    }

    None
}

pub fn format_amplitude(c: &Complex<f64>) -> String {
    let re = c.real;
    let im = c.imaginary;

    let re_zero = approx_eq(re.abs(), 0.0);
    let im_zero = approx_eq(im.abs(), 0.0);

    if re_zero && im_zero {
        return "0".to_string();
    }

    if im_zero {
        if let Some(s) = format_real_symbolic(re) {
            return s;
        }
        return format!("{:.4}", re);
    }

    if re_zero {
        if approx_eq(im.abs(), 1.0) {
            return if im > 0.0 {
                "i".to_string()
            } else {
                "-i".to_string()
            };
        }
        if let Some(s) = format_real_symbolic(im) {
            return format!("{}i", s);
        }
        return format!("{:.4}i", im);
    }

    let re_str = format_real_symbolic(re).unwrap_or_else(|| format!("{:.4}", re));
    let im_str = if approx_eq(im.abs(), 1.0) {
        if im > 0.0 {
            "+i".to_string()
        } else {
            "-i".to_string()
        }
    } else {
        let im_sym = format_real_symbolic(im.abs());
        let sign = if im > 0.0 { "+" } else { "-" };
        match im_sym {
            Some(s) => format!("{}{}i", sign, s.trim_start_matches('-')),
            None => format!("{}{:.4}i", sign, im.abs()),
        }
    };

    format!("{}{}", re_str, im_str)
}

pub fn format_probability(p: f64) -> String {
    if approx_eq(p, 0.0) {
        return "0".to_string();
    }
    if approx_eq(p, 1.0) {
        return "1".to_string();
    }
    if approx_eq(p, 0.5) {
        return "½".to_string();
    }
    if approx_eq(p, 0.25) {
        return "¼".to_string();
    }
    if approx_eq(p, 0.75) {
        return "¾".to_string();
    }
    if approx_eq(p, 0.125) {
        return "⅛".to_string();
    }
    if approx_eq(p, 0.0625) {
        return "¹⁄₁₆".to_string();
    }
    if approx_eq(p, 1.0 / 3.0) {
        return "⅓".to_string();
    }
    if approx_eq(p, 2.0 / 3.0) {
        return "⅔".to_string();
    }

    format!("{:.4}", p)
}
