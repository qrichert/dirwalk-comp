use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub fn find_files_recursively(root: impl AsRef<Path>, extensions: &[&str]) -> Vec<PathBuf> {
    let does_match_extensions = |entry: &Path| -> bool {
        let Some(extension) = entry.extension().and_then(|extension| extension.to_str()) else {
            return false;
        };
        extensions.contains(&extension)
    };

    let root = root.as_ref();
    if root.is_file() {
        return if does_match_extensions(root) {
            vec![root.to_owned()]
        } else {
            vec![]
        };
    }

    let results: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::new()));

    rayon::scope(|s| {
        walkdir_filtered(root, &does_match_extensions, &results, s);
    });

    let results = results.lock().unwrap().to_vec();

    results
}

fn walkdir_filtered<'s>(
    dir: &Path,
    filter: &'s (impl Fn(&Path) -> bool + Sync + 's),
    results: &Arc<Mutex<Vec<PathBuf>>>,
    scope: &rayon::Scope<'s>,
) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let entries: Vec<PathBuf> = entries
        .filter_map(|x| x.ok().map(|entry| entry.path()))
        .collect();
    // entries.sort_unstable(); // unfair comparison

    let mut res = Vec::with_capacity(entries.len());

    for entry in entries {
        if entry.is_dir() && !is_hidden(&entry) {
            let results = Arc::clone(results);
            scope.spawn(move |s| {
                walkdir_filtered(&entry, filter, &results, s);
            });
        } else if !is_hidden(&entry) && filter(&entry) {
            res.push(entry);
        }
    }

    results.lock().unwrap().extend(res);
}

#[cfg(unix)]
fn is_hidden(entry: &Path) -> bool {
    use std::os::unix::ffi::OsStrExt;
    entry.file_name().is_some_and(|file_name| {
        let bytes = file_name.as_bytes();
        bytes.first().is_some_and(|first_char| *first_char == b'.') || bytes == b"node_modules"
    })
}
