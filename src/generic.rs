use pyo3::exceptions::PyValueError;
use pyo3::types::PyDict;
use pyo3::PyResult;


/// Parse Save Types from dict
pub fn types_from_dict(types: &mut uesave::Types, dict: &PyDict) -> PyResult<()> {
    for (dict_key, dict_value) in dict.iter() {
        let key: String = dict_key.extract()?;
        let value: String = dict_value.extract()?;
        types.add(
            key.clone(),
            value.try_into().map_err(|err| {
                PyValueError::new_err(format!(
                    "failed type value conversion (key '{}'):  {}",
                    key, err
                ))
            })?,
        );
    }

    return Ok(());
}
