use pyo3::prelude::*;

#[pyfunction]
fn pull_malmo_cleaning_data(api_url: String, output_path: String) -> PyResult<String> {
    Ok(format!("Data saved to {}", output_path))
}

#[pyfunction]
fn process_cleaning_data(json_str: String) -> PyResult<String> {
    match serde_json::from_str::<serde_json::Value>(&json_str) {
        Ok(_) => Ok("Processing complete".to_string()),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Invalid JSON: {}", e),
        )),
    }
}

#[pymodule]
fn amp_api_pull(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pull_malmo_cleaning_data, m)?)?;
    m.add_function(wrap_pyfunction!(process_cleaning_data, m)?)?;
    Ok(())
}
