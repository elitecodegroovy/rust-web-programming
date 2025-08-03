// src/lib.rs
use pyo3::prelude::*;

/// Multiplies two integers.
#[pyfunction]
fn multiply_numbers(a: i32, b: i32) -> PyResult<i32> {
    Ok(a * b)
}

/// Forms a Python module named 'my_rust_lib'.
#[pymodule]
fn my_rust_lib(_py: Python,  m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply_numbers, m)?)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stokes_baker() {
    }
}