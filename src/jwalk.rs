use jwalk::{self, DirEntry, WalkDir};
use std::path::{Path, PathBuf};

pub fn find_files_recursively<'a>(
    root: impl AsRef<Path>,
    extensions: &'a [&'a str],
) -> impl Iterator<Item = PathBuf> + 'a {
    let does_entry_match = move |entry: Result<DirEntry<((), ())>, jwalk::Error>| {
        let entry = entry.ok()?;

        if !entry.file_type().is_file() {
            return None;
        }

        let path = entry.path();

        let extension = path.extension()?.to_str()?;
        if !extensions.contains(&extension) {
            return None;
        }

        Some(path)
    };

    WalkDir::new(root)
        .sort(false)
        .follow_links(true)
        .skip_hidden(true)
        .into_iter()
        .filter_map(does_entry_match)
}
