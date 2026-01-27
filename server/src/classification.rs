//! Classification data management for AMP testing interface
//! Stores human-reviewed classification data to local JSON files in Documents folder

use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

/// Get the classification data directory (Documents/amp_classifications)
fn get_classification_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir()
        .ok_or_else(|| "Could not determine home directory".to_string())?;
    
    let docs_dir = if cfg!(target_os = "windows") {
        home.join("Documents")
    } else if cfg!(target_os = "macos") {
        home.join("Documents")
    } else if cfg!(target_os = "linux") {
        home.join("Documents")
    } else {
        home
    };

    let class_dir = docs_dir.join("amp_classifications");
    
    // Create directory if it doesn't exist
    if !class_dir.exists() {
        fs::create_dir_all(&class_dir)
            .map_err(|e| format!("Failed to create classifications directory: {}", e))?
        ;
    }

    Ok(class_dir)
}

/// Get path to classification JSON file for a category
fn get_classification_file(category: &str) -> Result<PathBuf, String> {
    let dir = get_classification_dir()?;
    Ok(dir.join(format!("amp_stadsatlas_{}.json", category)))
}

/// Classification entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationEntry {
    pub id: String,
    pub timestamp: String,
    pub address: String,
    pub postal_code: String,
    pub source: String,
    pub matches_html: String,
}

/// Request body for classification
#[derive(Debug, Serialize, Deserialize)]
pub struct ClassificationRequest {
    pub category: String,
    pub data: ClassificationData,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassificationData {
    pub address: String,
    pub postal_code: String,
    pub source: String,
    pub matches_html: String,
}

/// Response body for classification operations
#[derive(Debug, Serialize, Deserialize)]
pub struct ClassificationResponse {
    pub success: bool,
    pub message: String,
    pub id: Option<String>,
}

/// Add classification entry to JSON file
pub fn add_classification(req: &ClassificationRequest) -> Result<String, String> {
    // Validate category
    if req.category != "notMatching" && req.category != "invalid" {
        return Err(format!("Invalid category: {}", req.category));
    }

    let file_path = get_classification_file(&req.category)?;
    
    // Generate unique ID
    let id = format!(
        "{}-{}-{}",
        req.category,
        chrono::Local::now().timestamp_millis(),
        uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("xxx")
    );

    // Create entry
    let entry = ClassificationEntry {
        id: id.clone(),
        timestamp: req.timestamp.clone(),
        address: req.data.address.clone(),
        postal_code: req.data.postal_code.clone(),
        source: req.data.source.clone(),
        matches_html: req.data.matches_html.clone(),
    };

    // Load or create JSON structure
    let mut json: Value = if file_path.exists() {
        let content = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read classification file: {}", e))?
        ;
        serde_json::from_str(&content)
            .unwrap_or_else(|_| json!({ "entries": [] }))
    } else {
        json!({ "entries": [] })
    };

    // Ensure entries array exists
    if !json["entries"].is_array() {
        json["entries"] = json!([])
    }

    // Add new entry
    if let Some(entries) = json["entries"].as_array_mut() {
        entries.push(serde_json::to_value(&entry)
            .map_err(|e| format!("Failed to serialize entry: {}", e))?
        );
    }

    // Write back to file
    let json_str = serde_json::to_string_pretty(&json)
        .map_err(|e| format!("Failed to serialize JSON: {}", e))?
    ;

    fs::write(&file_path, json_str)
        .map_err(|e| format!("Failed to write classification file: {}", e))?
    ;

    Ok(id)
}

/// Undo the last classification for a category and address
pub fn undo_classification(category: &str, address: &str) -> Result<String, String> {
    // Validate category
    if category != "notMatching" && category != "invalid" {
        return Err(format!("Invalid category: {}", category));
    }

    let file_path = get_classification_file(category)?;

    // If file doesn't exist, nothing to undo
    if !file_path.exists() {
        return Err(format!("No classifications found for category: {}", category));
    }

    // Load JSON
    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read classification file: {}", e))?
    ;
    
    let mut json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse classification file: {}", e))?
    ;

    // Get entries array
    if let Some(entries) = json["entries"].as_array_mut() {
        // Find last entry matching this address
        if let Some(last_index) = entries.iter().rposition(|e| {
            e.get("address")
                .and_then(|addr| addr.as_str())
                .map(|addr| addr == address)
                .unwrap_or(false)
        }) {
            entries.remove(last_index);

            // Write back to file
            let json_str = serde_json::to_string_pretty(&json)
                .map_err(|e| format!("Failed to serialize JSON: {}", e))?
            ;

            fs::write(&file_path, json_str)
                .map_err(|e| format!("Failed to write classification file: {}", e))?
            ;

            return Ok(format!("Undid last classification for '{}' in category '{}'", address, category));
        }
    }

    Err(format!("No classifications found for '{}' in category '{}'", address, category))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_classification_dir() {
        let result = get_classification_dir();
        assert!(result.is_ok());
        let dir = result.unwrap();
        assert!(dir.ends_with("amp_classifications"));
    }
}
