use crate::desktop::DesktopApp;
use crate::history::History;
use nucleo::{Matcher, Config, Utf32Str};
use nucleo::pattern::{Pattern, CaseMatching, Normalization};

/// Match result with score
#[derive(Debug, Clone)]
pub struct MatchResult {
    pub app: DesktopApp,
    pub fuzzy_score: u32,
    pub frecency_score: f64,
    pub combined_score: f64,
}

/// Fuzzy matcher using nucleo
pub struct FuzzyMatcher {
    matcher: Matcher,
    min_score: u32,
}

impl FuzzyMatcher {
    pub fn new() -> Self {
        let config = Config::DEFAULT;
        Self {
            matcher: Matcher::new(config),
            min_score: 0,
        }
    }

    /// Set minimum fuzzy score threshold (0-100+)
    pub fn with_min_score(mut self, score: u32) -> Self {
        self.min_score = score;
        self
    }

    /// Match apps against a query, returning sorted results
    pub fn match_apps(
        &mut self,
        query: &str,
        apps: &[DesktopApp],
        history: &History,
        max_results: usize,
    ) -> Vec<MatchResult> {
        // Empty query returns all apps sorted by frecency
        if query.is_empty() {
            let mut results: Vec<MatchResult> = apps
                .iter()
                .map(|app| {
                    let frecency = history.get_score(&app.id);
                    MatchResult {
                        app: app.clone(),
                        fuzzy_score: 0,
                        frecency_score: frecency,
                        combined_score: frecency,
                    }
                })
                .collect();

            // Sort by frecency descending, then alphabetically
            results.sort_by(|a, b| {
                b.frecency_score
                    .partial_cmp(&a.frecency_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.app.name.to_lowercase().cmp(&b.app.name.to_lowercase()))
            });

            results.truncate(max_results);
            return results;
        }

        let pattern = Pattern::new(
            query,
            CaseMatching::Smart,
            Normalization::Smart,
            nucleo::pattern::AtomKind::Fuzzy,
        );

        let mut results: Vec<MatchResult> = apps
            .iter()
            .filter_map(|app| {
                // Build search haystack from name + keywords
                let mut haystack = app.name.clone();
                for keyword in &app.keywords {
                    haystack.push(' ');
                    haystack.push_str(keyword);
                }

                // Convert to Utf32Str for nucleo
                let mut buf = Vec::new();
                let haystack_utf32 = Utf32Str::new(&haystack, &mut buf);

                // Score the match
                let score = pattern.score(haystack_utf32, &mut self.matcher)?;

                if score < self.min_score {
                    return None;
                }

                let frecency = history.get_score(&app.id);

                // Combined score: fuzzy match dominates, frecency as tiebreaker
                // Normalize fuzzy score (typically 0-1000) and add frecency boost
                let combined = score as f64 + (frecency * 10.0);

                Some(MatchResult {
                    app: app.clone(),
                    fuzzy_score: score,
                    frecency_score: frecency,
                    combined_score: combined,
                })
            })
            .collect();

        // Sort by combined score descending
        results.sort_by(|a, b| {
            b.combined_score
                .partial_cmp(&a.combined_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        results.truncate(max_results);
        results
    }
}

impl Default for FuzzyMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_app(id: &str, name: &str) -> DesktopApp {
        DesktopApp {
            id: id.to_string(),
            name: name.to_string(),
            exec: format!("{} %u", id),
            icon: None,
            keywords: vec![],
            description: None,
            path: PathBuf::new(),
        }
    }

    #[test]
    fn test_fuzzy_match() {
        let apps = vec![
            make_app("firefox", "Firefox"),
            make_app("chromium", "Chromium"),
            make_app("code", "Visual Studio Code"),
        ];

        let history = History::default();
        let mut matcher = FuzzyMatcher::new();

        let results = matcher.match_apps("fire", &apps, &history, 10);
        assert!(!results.is_empty());
        assert_eq!(results[0].app.id, "firefox");
    }

    #[test]
    fn test_empty_query() {
        let apps = vec![
            make_app("firefox", "Firefox"),
            make_app("chromium", "Chromium"),
        ];

        let history = History::default();
        let mut matcher = FuzzyMatcher::new();

        let results = matcher.match_apps("", &apps, &history, 10);
        assert_eq!(results.len(), 2);
    }
}
