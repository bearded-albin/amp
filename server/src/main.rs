/// Open a new browser window with 2 tabs:
/// Tab 1: StadsAtlas with pre-filled address search
/// Tab 2: Correlation result data
fn open_browser_windows(
    result: &&CorrelationResult,
    _window_idx: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let address = &result.address;

    // Direct StadsAtlas URL with query parameter for address search
    let stadsatlas_url = format!(
        "https://stadsatlas.malmo.se/stadsatlas/?q={}",
        urlencoding::encode(address)
    );

    // Create correlation result data page
    let correlation_data = create_correlation_result_page(result);
    let correlation_data_url = format!(
        "data:text/html;charset=utf-8,{}",
        urlencoding::encode(&correlation_data)
    );

    // Try to open windows using different methods depending on OS
    #[cfg(target_os = "windows")]
    {
        // Windows: Open new browser window with both URLs
        std::process::Command::new("cmd")
            .args(&[
                "/C",
                &format!(
                    "start chrome --new-window \"{}\" && timeout /t 2 && start chrome \"{}\"",
                    stadsatlas_url, correlation_data_url
                ),
            ])
            .output()
            .ok();
    }

    #[cfg(target_os = "macos")]
    {
        // macOS: Open new Safari window with both URLs
        let script = format!(
            r#"open -n '{}' & sleep 1 && open -n '{}' "#,
            stadsatlas_url, correlation_data_url
        );
        std::process::Command::new("bash")
            .args(&["-c", &script])
            .output()
            .ok();
    }

    #[cfg(target_os = "linux")]
    {
        // Linux: Open browser directly with --new-window flag for separate windows
        let browser = get_browser_executable();

        // First window: StadsAtlas
        std::process::Command::new(&browser)
            .args(&["--new-window", &stadsatlas_url])
            .spawn()
            .ok();

        // Small delay before opening second window
        thread::sleep(Duration::from_millis(800));

        // Second window: correlation data
        std::process::Command::new(&browser)
            .args(&["--new-window", &correlation_data_url])
            .spawn()
            .ok();
    }

    Ok(())
}
