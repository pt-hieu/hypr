use freedesktop_icons::lookup;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::Mutex;

/// Icon cache using LRU eviction
pub struct IconCache {
    cache: Mutex<LruCache<String, Option<PathBuf>>>,
    size: u16,
    theme: Option<String>,
}

impl IconCache {
    /// Create a new icon cache
    pub fn new(capacity: usize, size: u16, theme: Option<String>) -> Self {
        let cache = LruCache::new(
            NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(100).unwrap())
        );

        Self {
            cache: Mutex::new(cache),
            size,
            theme,
        }
    }

    /// Look up an icon by name, returning cached path if available
    pub fn get(&self, icon_name: &str) -> Option<PathBuf> {
        let mut cache = self.cache.lock().unwrap();

        // Check cache first
        if let Some(cached) = cache.get(icon_name) {
            return cached.clone();
        }

        // Look up icon
        let result = self.lookup_icon(icon_name);

        // Cache the result (even if None, to avoid repeated lookups)
        cache.put(icon_name.to_string(), result.clone());

        result
    }

    /// Perform the actual icon lookup
    fn lookup_icon(&self, icon_name: &str) -> Option<PathBuf> {
        // First, check if it's an absolute path
        if icon_name.starts_with('/') {
            let path = PathBuf::from(icon_name);
            if path.exists() {
                return Some(path);
            }
        }

        // Try with specified theme or default
        let mut builder = lookup(icon_name).with_size(self.size);

        if let Some(ref theme) = self.theme {
            builder = builder.with_theme(theme);
        }

        builder.find()
    }

    /// Clear the cache
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }
}

impl Default for IconCache {
    fn default() -> Self {
        Self::new(200, 48, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_lookup() {
        let cache = IconCache::default();

        // Common icons that should exist on most systems
        let common_icons = ["firefox", "chromium", "terminal", "folder", "application-x-executable"];

        for icon in common_icons {
            if let Some(path) = cache.get(icon) {
                println!("Found {}: {:?}", icon, path);
            }
        }
    }

    #[test]
    fn test_cache_behavior() {
        let cache = IconCache::new(10, 48, None);

        // First lookup
        let _ = cache.get("nonexistent-icon");

        // Second lookup should hit cache (even for None results)
        let _ = cache.get("nonexistent-icon");
    }
}
