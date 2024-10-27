use pyo3::prelude::*;
use configler_core;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok(configler_core::sum_as_string(a, b))
}

/// A Python module implemented in Rust.
#[pymodule]
fn configler_pyo3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
