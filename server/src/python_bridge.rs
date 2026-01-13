/// Bridge to Python API pull functionality
use crate::{config::Config, AppState};
use pyo3::{Python, PyModule};
use std::path::Path;
use tracing::info;

pub async fn pull_initial_data(config: &Config, state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    tokio::task::spawn_blocking({
        let config = config.clone();
        let state = state.clone();

        move || {
            Python::with_gil(|py| {
                let sys = PyModule::import(py, "sys")?;
                let path: &mut pyo3::types::PyList = sys.getattr("path")?.downcast()?;
                path.append("./py-bridge/python")?;

                let module = PyModule::import(py, "amp_api_pull")?;
                let result = module.call_method1("pull_malmo_cleaning_data", (&config.malmo_api_base,))?;

                info!("✅ Python API pull complete: {:?}", result);
                Ok::<(), pyo3::PyErr>(())
            })
        }
    })
        .await??;

    Ok(())
}

pub async fn pull_and_process(config: &Config, state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    pull_initial_data(config, state).await
}
