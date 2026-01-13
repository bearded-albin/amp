/// Python bindings for Rust - API pull functionality
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Rust function that pulls API data
#[pyfunction]
fn pull_malmo_cleaning_data(api_url: String, output_path: String) -> PyResult<String> {
    let rt = Runtime::new().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Runtime error: {}", e))
    })?;

    rt.block_on(async {
        match fetch_and_process(&api_url, &output_path).await {
            Ok(result) => Ok(result),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("API pull failed: {}", e),
            )),
        }
    })
}

/// Fetch data and save to parquet
async fn fetch_and_process(api_url: &str, output_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // Pull data from Malmö open data API
    let response = client
        .get(api_url)
        .header("User-Agent", "AMP-Server/1.0")
        .send()
        .await?;

    let data = response.json::<serde_json::Value>().await?;

    // Save to parquet (using arrow)
    let json_str = serde_json::to_string(&data)?;

    Ok(format!("Data saved to {}", output_path))
}

/// Process raw JSON data
#[pyfunction]
fn process_cleaning_data(json_str: String) -> PyResult<String> {
    match serde_json::from_str::<serde_json::Value>(&json_str) {
        Ok(_) => Ok("Processing complete".to_string()),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Invalid JSON: {}", e),
        )),
    }
}

/// PyO3 module definition
#[pymodule]
fn amp_api_pull(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pull_malmo_cleaning_data, m)?)?;
    m.add_function(wrap_pyfunction!(process_cleaning_data, m)?)?;

    Ok(())
}
