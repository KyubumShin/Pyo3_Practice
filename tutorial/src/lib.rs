use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_vec(i: u64) -> PyResult<u64> {
    let mut ret:u64 = 0;
    for i in 0..i {
        ret += i
    };
    Ok(ret)
}

/// A Python module implemented in Rust.
#[pymodule]
fn tutorial(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_vec, m)?)?;
    Ok(())
}