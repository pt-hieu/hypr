pub mod desktop;
pub mod history;
pub mod icons;
pub mod matcher;

pub use desktop::{DesktopApp, scan_applications};
pub use history::{History, FrecencyEntry};
pub use icons::IconCache;
pub use matcher::FuzzyMatcher;
