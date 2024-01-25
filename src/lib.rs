mod error;
mod generic;
mod read;

use crate::error::UESaveError;

use pyo3::prelude::*;

use pyo3::exceptions::PyNotImplementedError;
use pyo3::{pyfunction};



#[pyfunction]
fn save(_py: Python<'_>) -> PyResult<()> {
    Err(PyNotImplementedError::new_err(""))
}

/// A Python module implemented in Rust.
#[pymodule]
fn uesave_py(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("UesaveError", py.get_type::<UESaveError>())?;
    m.add_function(wrap_pyfunction!(read::read_save, m)?)?;
    m.add_function(wrap_pyfunction!(save, m)?)?;
    Ok(())
}
