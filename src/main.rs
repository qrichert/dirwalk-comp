#![allow(unused_imports, dead_code, unused_variables)]

mod carbon;
mod carbon_par;
mod ignore;
mod ignore_par;
mod jwalk;
mod walkdir;
mod walkdir_par;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time;

#[cfg(target_os = "linux")]
fn clear_file_cache() {
    _ = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg("echo 3 > /proc/sys/vm/drop_caches")
        .status();
}

#[cfg(target_os = "macos")]
fn clear_file_cache() {
    _ = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg("sync && sudo purge");
}

const PATH: &str = "/Users/Quentin/Developer";

fn main() {
    let path: &Path = Path::new(PATH);
    bench_carbon(path);
    bench_carbon_par(path);
    bench_walkdir(path);
    bench_walkdir_par(path);
    bench_ignore(path);
    bench_ignore_par(path);
    bench_jwalk(path);
    bench_fd(path);
    bench_find(path);
}

fn bench_carbon(path: &Path) {
    clear_file_cache();
    let start = time::Instant::now();
    let files = carbon::find_files_recursively(PATH, &["py", "rs"]).unwrap();
    let mut counter = 0;
    for file in files {
        counter += 1;
    }
    let end = start.elapsed().as_secs_f64();
    println!("Carbon:   Found {counter} files in {end}");
}

fn bench_carbon_par(path: &Path) {
    clear_file_cache();
    let start = time::Instant::now();
    let files = carbon_par::find_files_recursively(PATH, &["py", "rs"]);
    let mut counter = 0;
    for file in files {
        counter += 1;
    }
    let end = start.elapsed().as_secs_f64();
    println!("CarbonP:  Found {counter} files in {end}");
}

fn bench_walkdir(path: &Path) {
    clear_file_cache();
    let start = time::Instant::now();
    let files: Vec<PathBuf> = walkdir::find_files_recursively(PATH, &["py", "rs"]).collect();
    let mut counter = 0;
    for file in files {
        counter += 1;
    }
    let end = start.elapsed().as_secs_f64();
    println!("WalkDir:  Found {counter} files in {end}");
}

fn bench_walkdir_par(path: &Path) {
    clear_file_cache();
    let start = time::Instant::now();
    let counter = AtomicUsize::new(0);
    walkdir_par::find_files_recursively(PATH, &["py", "rs"], |p| {
        counter.fetch_add(1, Ordering::Relaxed);
    });
    let end = start.elapsed().as_secs_f64();
    println!(
        "WalkDirP: Found {} files in {end}",
        counter.fetch_or(0, Ordering::SeqCst)
    );
}

fn bench_ignore(path: &Path) {
    clear_file_cache();
    let start = time::Instant::now();
    let counter = ignore::find_files_recursively(PATH, &["py", "rs"]).count();
    let end = start.elapsed().as_secs_f64();
    println!("Ignore:   Found {counter} files in {end}");
}

fn bench_ignore_par(path: &Path) {
    clear_file_cache();
    let start = time::Instant::now();
    let counter = AtomicUsize::new(0);
    ignore_par::find_files_recursively(PATH, &["py", "rs"], |p| {
        counter.fetch_add(1, Ordering::Relaxed);
    });
    let end = start.elapsed().as_secs_f64();
    println!(
        "IgnoreP:  Found {} files in {end}",
        counter.fetch_or(0, Ordering::SeqCst)
    );
}

fn bench_jwalk(path: &Path) {
    clear_file_cache();
    let start = time::Instant::now();
    let files: Vec<PathBuf> = jwalk::find_files_recursively(PATH, &["py", "rs"]).collect();
    let mut counter = 0;
    for file in files {
        counter += 1;
    }
    let end = start.elapsed().as_secs_f64();
    println!("JWalk:    Found {counter} files in {end}");
}

fn bench_fd(path: &Path) {
    clear_file_cache();
    let start = time::Instant::now();
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"
fd . --extension rs --extension py --exclude 'node_modules' --hidden --exclude '.*' "{}" | wc -l
        "#,
            path.display()
        ))
        .arg(path) // Ensure path is specified
        .output()
        .expect("Failed to execute fd");
    let end = start.elapsed().as_secs_f64();
    let count = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    println!("fd:       Found {count} files in {end}");
}

fn bench_find(path: &Path) {
    clear_file_cache();
    let start = time::Instant::now();
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"
find "{}" -type f \( -name "*.rs" -o -name "*.py" \) \
  -not -path "*/.*" -not -path "*/node_modules/*" | wc -l
        "#,
            path.display()
        )) // Ensure it's a file
        .output()
        .expect("Failed to execute find");
    let end = start.elapsed().as_secs_f64();
    let count = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    println!("find:     Found {count} files in {end}");
}
