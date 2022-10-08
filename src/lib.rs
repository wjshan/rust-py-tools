use pyo3::{prelude::*, wrap_pymodule};

mod string_tools;

/// 使用pyo3
#[pymodule]
fn rust_py_tools(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(string_tools::string))?;
    Ok(())
}
