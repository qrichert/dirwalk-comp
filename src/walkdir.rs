use std::path::{Path, PathBuf};
use walkdir::{self, DirEntry, WalkDir};

pub fn find_files_recursively<'a>(
    root: impl AsRef<Path>,
    extensions: &'a [&'a str],
) -> impl Iterator<Item = PathBuf> + 'a {
    let does_entry_match = move |entry: walkdir::Result<DirEntry>| {
        let entry = entry.ok()?;

        if !entry.file_type().is_file() {
            return None;
        }

        let extension = entry.path().extension()?.to_str()?;
        if !extensions.contains(&extension) {
            return None;
        }

        Some(entry.into_path())
    };

    WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(does_entry_match)
}

#[cfg(unix)]
fn is_hidden(entry: &DirEntry) -> bool {
    use std::os::unix::ffi::OsStrExt;
    let bytes = entry.file_name().as_bytes();
    bytes.first().is_some_and(|first_char| *first_char == b'.') || bytes == b"node_modules"
}
