/// # Panics
/// when length of values is too large to be converted to f64.
#[must_use]
pub fn average(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let total = values.iter().fold(0.0, |a, b| a + b);
    total / convert(values.len()).expect("length of values too long to convert into f64")
}

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
const fn convert(x: usize) -> Result<f64, &'static str> {
    let result = x as f64;
    if result as usize != x {
        return Err("cannot convert");
    }
    Ok(result)
}
