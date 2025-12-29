use freedesktop_desktop_entry::DesktopEntry;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DesktopApp {
    pub id: String,
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    pub keywords: Vec<String>,
    pub description: Option<String>,
    pub path: PathBuf,
}

impl DesktopApp {
    /// Parse exec command, removing field codes (%f, %F, %u, %U, etc.)
    pub fn launch_command(&self) -> String {
        let mut cmd = self.exec.clone();
        // Remove common field codes
        for code in &["%f", "%F", "%u", "%U", "%d", "%D", "%n", "%N", "%i", "%c", "%k", "%v", "%m"] {
            cmd = cmd.replace(code, "");
        }
        cmd.trim().to_string()
    }
}

/// Get XDG data directories for .desktop files
fn get_applications_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    // User applications
    if let Some(data_home) = std::env::var_os("XDG_DATA_HOME") {
        dirs.push(PathBuf::from(data_home).join("applications"));
    } else if let Some(home) = std::env::var_os("HOME") {
        dirs.push(PathBuf::from(home).join(".local/share/applications"));
    }

    // System applications
    if let Some(data_dirs) = std::env::var_os("XDG_DATA_DIRS") {
        for dir in std::env::split_paths(&data_dirs) {
            dirs.push(dir.join("applications"));
        }
    } else {
        dirs.push(PathBuf::from("/usr/local/share/applications"));
        dirs.push(PathBuf::from("/usr/share/applications"));
    }

    dirs
}

/// Scan all .desktop files from XDG data directories
pub fn scan_applications() -> Vec<DesktopApp> {
    let mut apps = Vec::new();
    let mut seen_ids = std::collections::HashSet::new();

    for dir in get_applications_dirs() {
        if !dir.exists() {
            continue;
        }

        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();

            // Only process .desktop files
            if path.extension().and_then(|s| s.to_str()) != Some("desktop") {
                continue;
            }

            // Get app ID from filename
            let id = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            // Skip duplicates (first one found wins)
            if seen_ids.contains(&id) {
                continue;
            }

            // Parse the desktop entry
            let Ok(desktop) = DesktopEntry::from_path::<&str>(&path, None) else {
                continue;
            };

            // Skip entries marked as hidden or no display
            if desktop.no_display() || desktop.hidden() {
                continue;
            }

            // Only include Application type
            if desktop.type_().map(|t| t != "Application").unwrap_or(true) {
                continue;
            }

            // Must have a name and exec
            let locales: &[&str] = &[];
            let Some(name) = desktop.name(locales) else {
                continue;
            };
            let Some(exec) = desktop.exec() else {
                continue;
            };

            let icon = desktop.icon().map(|s| s.to_string());

            let keywords: Vec<String> = desktop
                .keywords(locales)
                .map(|kw| kw.into_iter().map(|s| s.to_string()).collect())
                .unwrap_or_default();

            let description = desktop.comment(locales).map(|s| s.to_string());

            seen_ids.insert(id.clone());

            apps.push(DesktopApp {
                id,
                name: name.to_string(),
                exec: exec.to_string(),
                icon,
                keywords,
                description,
                path: path.clone(),
            });
        }
    }

    // Sort alphabetically by default
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_applications() {
        let apps = scan_applications();
        // Should find at least some applications on most systems
        println!("Found {} applications", apps.len());
        for app in apps.iter().take(5) {
            println!("  - {} ({})", app.name, app.id);
        }
    }

    #[test]
    fn test_launch_command() {
        let app = DesktopApp {
            id: "test".to_string(),
            name: "Test".to_string(),
            exec: "firefox %u".to_string(),
            icon: None,
            keywords: vec![],
            description: None,
            path: PathBuf::new(),
        };
        assert_eq!(app.launch_command(), "firefox");
    }
}
