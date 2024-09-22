use ignore::{self, DirEntry, Walk, WalkBuilder};
use std::path::{Path, PathBuf};

pub fn find_files_recursively<'a>(
    root: impl AsRef<Path>,
    extensions: &'a [&'a str],
) -> impl Iterator<Item = PathBuf> + 'a {
    let does_entry_match = move |entry: Result<DirEntry, ignore::Error>| {
        let entry = entry.ok()?;

        if !entry.file_type().is_some_and(|entry| entry.is_file()) {
            return None;
        }

        let extension = entry
            .path()
            .extension()
            .and_then(|extension| extension.to_str())?;

        if !extensions.iter().any(|ext| *ext == extension) {
            return None;
        }

        Some(entry.into_path())
    };

    WalkBuilder::new(root)
        .follow_links(true)
        .hidden(true)
        .max_depth(None)
        .build()
        .filter_map(does_entry_match)
}
