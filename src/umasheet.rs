use crate::{commands::Error, sheet};
use std::sync::{LazyLock, RwLock};

static UMA_LIST: LazyLock<RwLock<Vec<String>>> = LazyLock::new(|| RwLock::new(Vec::new()));

pub async fn init_uma_list() {
    let list = sheet::get_uma_list().await;
    *UMA_LIST.write().unwrap() = list;
}

pub fn get_uma_list_cached() -> Vec<String> {
    UMA_LIST.read().unwrap().clone()
}

pub async fn is_already_following(username: &str, uma_name: &str) -> Result<bool, Error> {
    let members = sheet::sheet()
        .read("Membres!A:B")
        .await
        .map_err(|e| format!("Failed to read sheet: {}", e))?;

    Ok(members.iter().any(|row| {
        let name = row.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let uma = row.get(1).and_then(|v| v.as_str()).unwrap_or("");
        name == username && uma == uma_name
    }))
}

pub async fn get_user_follow(username: &str) -> Result<Vec<String>, Error> {
    let members = sheet::sheet()
        .read("Membres!A:B")
        .await
        .map_err(|e| format!("Failed to read sheet: {}", e))?;

    Ok(members
        .iter()
        .filter(|row| row.get(0).and_then(|v| v.as_str()).unwrap_or("") == username)
        .filter_map(|row| row.get(1).and_then(|v| v.as_str()).map(|s| s.to_string()))
        .collect())
}

pub async fn get_followers(uma_name: &str) -> Result<Vec<String>, Error> {
    let members = sheet::sheet()
        .read("Membres!A:B")
        .await
        .map_err(|e| format!("Failed to read sheet: {}", e))?;

    Ok(members
        .iter()
        .filter(|row| row.get(1).and_then(|v| v.as_str()).unwrap_or("") == uma_name)
        .filter_map(|row| row.get(0).and_then(|v| v.as_str()).map(|s| s.to_string()))
        .collect())
}

pub async fn get_follow_range(username: &str, uma_name: &str) -> Result<Option<String>, Error> {
    let members = sheet::sheet()
        .read("Membres!A:B")
        .await
        .map_err(|e| format!("Failed to read sheet: {}", e))?;

    for (index, row) in members.iter().enumerate() {
        let name = row.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let uma = row.get(1).and_then(|v| v.as_str()).unwrap_or("");
        if name == username && uma == uma_name {
            let row_number = index + 1; // Sheets are 1-indexed
            return Ok(Some(format!("Membres!A{}:B{}", row_number, row_number)));
        }
    }

    Ok(None)
}
