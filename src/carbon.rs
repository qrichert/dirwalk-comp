use std::io;
use std::path::{Path, PathBuf};

pub fn find_files_recursively(
    root: impl AsRef<Path>,
    extensions: &[&str],
) -> io::Result<Vec<PathBuf>> {
    let does_match_extensions = |entry: &Path| -> bool {
        let Some(extension) = entry.extension().and_then(|extension| extension.to_str()) else {
            return false;
        };
        extensions.contains(&extension)
    };

    let root = root.as_ref();
    if root.is_file() {
        return if does_match_extensions(root) {
            Ok(vec![root.to_owned()])
        } else {
            Ok(vec![])
        };
    }
    walkdir_filtered(root, &does_match_extensions)
}

fn walkdir_filtered(dir: &Path, filter: &dyn Fn(&Path) -> bool) -> io::Result<Vec<PathBuf>> {
    let entries: Vec<PathBuf> = std::fs::read_dir(dir)?
        .filter_map(|x| x.ok().map(|entry| entry.path()))
        .collect();
    // entries.sort_unstable(); // unfair comparison

    let mut res = Vec::with_capacity(entries.len());
    for entry in entries {
        if entry.is_dir() && !is_hidden(&entry) {
            res.extend(walkdir_filtered(&entry, filter)?);
        } else if !is_hidden(&entry) && filter(&entry) {
            res.push(entry);
        }
    }
    Ok(res)
}

#[cfg(unix)]
fn is_hidden(entry: &Path) -> bool {
    use std::os::unix::ffi::OsStrExt;
    entry.file_name().is_some_and(|file_name| {
        let bytes = file_name.as_bytes();
        bytes.first().is_some_and(|first_char| *first_char == b'.') || bytes == b"node_modules"
    })
}
