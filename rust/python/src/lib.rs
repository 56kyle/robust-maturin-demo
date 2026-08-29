use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> String {
    robust_maturin_demo_core::sum_as_string(a, b)
}

#[pymodule]
fn robust_maturin_demo(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(sum_as_string, module)?)?;
    Ok(())
}
