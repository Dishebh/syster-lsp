//! Test helpers for LspServer tests
//!
//! Provides cached stdlib loading for fast test execution.

use crate::server::LspServer;
use std::path::PathBuf;
use std::sync::LazyLock;
use syster::project::file_loader;
use syster::syntax::SyntaxFile;

/// Pre-parsed stdlib files, loaded once and shared across all tests
pub static CACHED_STDLIB: LazyLock<Vec<(PathBuf, SyntaxFile)>> = LazyLock::new(|| {
    let stdlib_path = discover_stdlib_path();
    parse_stdlib_files(&stdlib_path)
});

/// Discover stdlib path for tests
fn discover_stdlib_path() -> PathBuf {
    // Check for sysml.library in crate directory first (for standalone builds)
    let crate_stdlib = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml.library");
    if crate_stdlib.exists() {
        return crate_stdlib;
    }

    // Fall back to workspace-level stdlib
    PathBuf::from("sysml.library")
}

/// Parse all stdlib files and return as cloneable vec
fn parse_stdlib_files(stdlib_path: &PathBuf) -> Vec<(PathBuf, SyntaxFile)> {
    if !stdlib_path.exists() || !stdlib_path.is_dir() {
        return Vec::new();
    }

    let file_paths = match file_loader::collect_file_paths(stdlib_path) {
        Ok(paths) => paths,
        Err(_) => return Vec::new(),
    };

    file_paths
        .iter()
        .filter_map(|path| {
            file_loader::load_and_parse(path)
                .ok()
                .map(|file| (path.clone(), file))
        })
        .collect()
}

/// Create an LspServer without stdlib (fast, for most unit tests)
pub fn create_server() -> LspServer {
    LspServer::with_config(false, None)
}

/// Create an LspServer with cached stdlib (for tests that need stdlib symbols)
pub fn create_server_with_stdlib() -> LspServer {
    let mut server = LspServer::with_config(false, None);

    // Add pre-parsed stdlib files to workspace
    for (path, file) in CACHED_STDLIB.iter() {
        server.workspace_mut().add_file(path.clone(), file.clone());
    }
    server.workspace_mut().mark_stdlib_loaded();

    server
}
