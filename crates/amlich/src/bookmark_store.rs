use std::io;
use std::path::PathBuf;

use crate::history::HistoryEntry;

fn bookmarks_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("amlich").join("bookmarks.json"))
}

pub fn normalize_bookmarks(entries: Vec<HistoryEntry>) -> Vec<HistoryEntry> {
    let mut valid: Vec<HistoryEntry> = entries.into_iter().filter(|e| e.is_valid()).collect();
    valid.sort();
    valid.dedup();
    valid
}

pub fn load_bookmarks() -> Vec<HistoryEntry> {
    let Some(path) = bookmarks_path() else {
        return Vec::new();
    };

    let Ok(content) = std::fs::read_to_string(path) else {
        return Vec::new();
    };

    let Ok(loaded) = serde_json::from_str::<Vec<HistoryEntry>>(&content) else {
        return Vec::new();
    };

    normalize_bookmarks(loaded)
}

pub fn save_bookmarks(bookmarks: &[HistoryEntry]) -> io::Result<()> {
    let Some(path) = bookmarks_path() else {
        return Ok(());
    };

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let normalized = normalize_bookmarks(bookmarks.to_vec());
    let json = serde_json::to_vec_pretty(&normalized)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let tmp_path = path.with_extension(format!("json.tmp.{}", std::process::id()));
    std::fs::write(&tmp_path, &json)?;
    std::fs::rename(&tmp_path, &path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::history::HistoryEntry;

    use super::normalize_bookmarks;

    #[test]
    fn drops_invalid_and_dedups() {
        let normalized = normalize_bookmarks(vec![
            HistoryEntry {
                year: 2025,
                month: 1,
                day: 1,
            },
            HistoryEntry {
                year: 2025,
                month: 13,
                day: 1,
            },
            HistoryEntry {
                year: 2025,
                month: 1,
                day: 1,
            },
        ]);

        assert_eq!(normalized.len(), 1);
        assert_eq!(normalized[0].month, 1);
    }
}
