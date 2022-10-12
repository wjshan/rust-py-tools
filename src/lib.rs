use pyo3::prelude::*;
mod string_tools;

// 使用pyo3
#[pymodule]
fn rust_py_tools_d(_py: Python, m: &PyModule) -> PyResult<()> {
    string_tools::register(_py, m)?;
    Ok(())
}
