use pyo3::create_exception;
use pyo3::exceptions::PyException;

create_exception!(
    uesave_py,
    UESaveError,
    PyException,
    "Errors raised from uesave."
);
