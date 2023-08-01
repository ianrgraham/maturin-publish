use pyo3::prelude::*;

#[pyfunction]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[pymodule]
fn maturin_publish(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
