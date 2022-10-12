use pyo3::prelude::*;

pub mod edit_distance;
pub mod distinct_subsequences;

pub fn register(py: Python<'_>, parent_module: &PyModule)-> PyResult<()>{
    let string = PyModule::new(py, "string")?;
    distinct_subsequences::register(string)?;
    edit_distance::register( string)?;
    parent_module.add_submodule(string)?;
    Ok(())
}

// #[pymodule]
// fn string(py: Python<'_>,m:&PyModule)->PyResult<()>{
//     register(py, m)?;
//     Ok(())
// }
