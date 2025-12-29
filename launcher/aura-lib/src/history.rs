use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrecencyEntry {
    pub frequency: u32,
    pub last_accessed: u64,
}

impl FrecencyEntry {
    /// Calculate frecency score using exponential decay
    /// Half-life of 7 days: score = frequency * 0.5^(age_days / 7)
    pub fn score(&self) -> f64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let age_secs = now.saturating_sub(self.last_accessed);
        let age_days = age_secs as f64 / 86400.0;
        let decay = 0.5_f64.powf(age_days / 7.0);

        self.frequency as f64 * decay
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct History {
    pub apps: HashMap<String, FrecencyEntry>,
    #[serde(skip)]
    path: PathBuf,
}

impl History {
    /// Load history from XDG data directory
    pub fn load() -> Self {
        let path = Self::history_path();

        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(mut history) = serde_json::from_str::<History>(&contents) {
                history.path = path;
                return history;
            }
        }

        History {
            apps: HashMap::new(),
            path,
        }
    }

    /// Get the history file path
    fn history_path() -> PathBuf {
        let xdg = xdg::BaseDirectories::with_prefix("aura-launcher")
            .expect("Failed to get XDG directories");

        xdg.place_data_file("history.json")
            .expect("Failed to create data directory")
    }

    /// Record an app launch
    pub fn record_launch(&mut self, app_id: &str) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let entry = self.apps.entry(app_id.to_string()).or_insert(FrecencyEntry {
            frequency: 0,
            last_accessed: now,
        });

        entry.frequency += 1;
        entry.last_accessed = now;
    }

    /// Get frecency score for an app (0.0 if never launched)
    pub fn get_score(&self, app_id: &str) -> f64 {
        self.apps
            .get(app_id)
            .map(|e| e.score())
            .unwrap_or(0.0)
    }

    /// Save history to disk
    pub fn save(&self) -> io::Result<()> {
        let contents = serde_json::to_string_pretty(&self)?;

        // Ensure parent directory exists
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.path, contents)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frecency_score() {
        let entry = FrecencyEntry {
            frequency: 10,
            last_accessed: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        // Recent entry should have score close to frequency
        let score = entry.score();
        assert!(score > 9.0 && score <= 10.0, "Score was {}", score);
    }

    #[test]
    fn test_record_launch() {
        let mut history = History::default();
        history.record_launch("firefox");
        history.record_launch("firefox");
        history.record_launch("firefox");

        assert_eq!(history.apps.get("firefox").unwrap().frequency, 3);
    }
}
