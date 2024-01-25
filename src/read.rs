use pyo3::exceptions::{PyRuntimeError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyDict, PyTuple};
use pyo3::{intern, pyfunction, PyAny, PyResult, Python};

use crate::error::UESaveError;

// TODO: better docstrings

/// Loads save file (.sav) returning Save
#[pyfunction]
pub fn read_save<'a>(py: Python<'a>, fd: &'a PyAny, types: Option<&PyDict>) -> PyResult<&'a PyAny> {
    fd.getattr(intern!(py, "read")).map_err(|_| {
        PyTypeError::new_err(format!(
            "expected BinaryIO like object (file, BytesIO), not {} (missing read() method)",
            fd.get_type()
        ))
    })?;

    if let Ok(mode) = fd.getattr(intern!(py, "mode")) {
        if mode.to_string() != "rb" {
            println!("warning: possibly invalid file mode {}", mode.to_string())
        }
    }

    let mut reader = pyo3_filelike::PyBinaryFile::from(fd);

    let save: uesave::Save;
    match types {
        None => {
            save = uesave::Save::read(&mut reader)
                .map_err(|err| UESaveError::new_err(err.to_string()))?; // TODO: expose error
        }
        Some(dict) => {
            let mut types_map = uesave::Types::new();
            crate::generic::types_from_dict(&mut types_map, dict)?;
            save = uesave::Save::read_with_types(&mut reader, &types_map)
                .map_err(|err| UESaveError::new_err(err.to_string()))?; // TODO: expose error
        }
    };

    Ok(py_transfer_save(py, save)?)
}

/// Transfer `uesave::Save` into python uesave_py.save.Save
fn py_transfer_save(py: Python<'_>, save: uesave::Save) -> PyResult<&'_ PyAny> {
    let save_module = PyModule::import(py, "uesave_py.save")?;

    let header_instance = {
        let header_class = save_module.getattr("Header")?;
        header_class.call(
            PyTuple::empty(py),
            Some(pythonize::pythonize(py, &save.header)?.downcast::<PyDict>(py)?),
        )
    }
    .map_err(|err| {
        PyRuntimeError::new_err(format!(
            "failed transferring root header from rust: {}",
            err
        ))
    })?;

    let root_instance = {
        let root_class = save_module.getattr("Root")?;
        root_class.call(
            PyTuple::empty(py),
            Some(pythonize::pythonize(py, &save.root)?.downcast::<PyDict>(py)?),
        )
    }
    .map_err(|err| {
        PyRuntimeError::new_err(format!(
            "failed transferring root header from rust: {}",
            err
        ))
    })?;

    let init_kw = PyDict::new(py);
    init_kw.set_item("header", header_instance)?;
    init_kw.set_item("root", root_instance)?;
    init_kw.set_item("extra", PyByteArray::new(py, &save.extra))?;

    Ok(save_module
        .getattr("Save")?
        .call(PyTuple::empty(py), Some(init_kw))?)
}
