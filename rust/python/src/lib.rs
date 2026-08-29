use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> String {
    robust_maturin_demo_core::sum_as_string(a, b)
}

#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(sum_as_string, module)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::types::PyModule;

    #[test]
    fn registered_module_calls_sum_as_string() -> PyResult<()> {
        Python::attach(|python| {
            let module = PyModule::new(python, "_native")?;
            _native(&module)?;

            let result: String = module.getattr("sum_as_string")?.call1((2, 3))?.extract()?;

            assert_eq!(result, "5");
            Ok(())
        })
    }
}
