use configler_core;
use pyo3::prelude::*;

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

#[cfg(test)]
mod tests {
    use pyo3::py_run;

    use super::*;

    #[test]
    fn test_sum_as_string() {
        pyo3::append_to_inittab!(configler_pyo3);
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let configler_pyo3 = py.import_bound("configler_pyo3").unwrap();
            py_run!(py, configler_pyo3, r#"
                assert configler_pyo3.sum_as_string(3,5) == '8'
            "#)
        });
    }
}