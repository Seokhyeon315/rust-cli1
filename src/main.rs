use std::fs;
use std::path::Path;

fn main() {
    // 1. Set path for macOS Applications
    // In macOS, there are two different path for applications: system level: `/Applications`  vs User level: `~Applications`
    let application_dir=Path::new("/Applications");
    println!("Target directory: {}", application_dir.display());

    // 2. List the each application name under /Applications
    let entries = fs::read_dir(application_dir).expect("Failed to read /Applications");
    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_dir() && path.extension().map_or(false, |e|e == "app") {
            println!("  {}", path.display());
        }
    }
}