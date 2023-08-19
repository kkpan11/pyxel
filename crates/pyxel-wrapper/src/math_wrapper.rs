use pyo3::prelude::*;

#[pyfunction]
fn ceil(x: f64) -> i32 {
    pyxel_engine::ceil(x)
}

#[pyfunction]
fn floor(x: f64) -> i32 {
    pyxel_engine::floor(x)
}

#[pyfunction]
fn sgn(x: f64) -> f64 {
    pyxel_engine::sgn(x)
}

#[pyfunction]
fn sqrt(x: f64) -> f64 {
    pyxel_engine::sqrt(x)
}

#[pyfunction]
fn sin(deg: f64) -> f64 {
    pyxel_engine::sin(deg)
}

#[pyfunction]
fn cos(deg: f64) -> f64 {
    pyxel_engine::cos(deg)
}

#[pyfunction]
fn atan2(y: f64, x: f64) -> f64 {
    pyxel_engine::atan2(y, x)
}

#[pyfunction]
fn rseed(seed: u32) {
    pyxel_engine::rseed(seed);
}

#[pyfunction]
fn rndi(a: i32, b: i32) -> i32 {
    pyxel_engine::rndi(a, b)
}

#[pyfunction]
fn rndf(a: f64, b: f64) -> f64 {
    pyxel_engine::rndf(a, b)
}

#[pyfunction]
fn nseed(seed: u32) {
    pyxel_engine::nseed(seed);
}

#[pyfunction]
fn noise(x: f64, y: Option<f64>, z: Option<f64>) -> f64 {
    let y = y.unwrap_or(0.0);
    let z = z.unwrap_or(0.0);
    pyxel_engine::noise(x, y, z)
}

pub fn add_math_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ceil, m)?)?;
    m.add_function(wrap_pyfunction!(floor, m)?)?;
    m.add_function(wrap_pyfunction!(sgn, m)?)?;
    m.add_function(wrap_pyfunction!(sqrt, m)?)?;
    m.add_function(wrap_pyfunction!(sin, m)?)?;
    m.add_function(wrap_pyfunction!(cos, m)?)?;
    m.add_function(wrap_pyfunction!(atan2, m)?)?;
    m.add_function(wrap_pyfunction!(rseed, m)?)?;
    m.add_function(wrap_pyfunction!(rndi, m)?)?;
    m.add_function(wrap_pyfunction!(rndf, m)?)?;
    m.add_function(wrap_pyfunction!(nseed, m)?)?;
    m.add_function(wrap_pyfunction!(noise, m)?)?;
    Ok(())
}
