use ignore::{self, DirEntry, WalkBuilder, WalkState};
use std::path::Path;

pub fn find_files_recursively(
    root: impl AsRef<Path>,
    extensions: &[&str],
    f: impl Fn(DirEntry) + Sync,
) {
    let does_entry_match = move |entry: &DirEntry| {
        let Some(extension) = entry
            .path()
            .extension()
            .and_then(|extension| extension.to_str())
        else {
            return false;
        };

        extensions.iter().any(|ext| *ext == extension)
    };

    WalkBuilder::new(root)
        .follow_links(true)
        .hidden(true)
        .max_depth(None)
        .build_parallel()
        .run(|| {
            Box::new(|entry| {
                if let Ok(entry) = entry {
                    if is_dir(&entry) {
                        return WalkState::Continue;
                    }
                    if does_entry_match(&entry) {
                        f(entry);
                        return WalkState::Continue;
                    }
                };
                WalkState::Skip
            })
        });
}

fn is_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_some_and(|entry| entry.is_dir())
}
