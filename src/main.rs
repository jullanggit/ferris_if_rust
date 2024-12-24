use std::env;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let mut current_dir_slice = current_dir.as_path();

    loop {
        if current_dir_slice.join("Cargo.toml").is_file() {
            print!("🦀 ");
            return;
        }

        if let Some(parent) = current_dir.parent() {
            current_dir_slice = parent;
        } else {
            break;
        }
    }

    // If no Cargo.toml is found, exit silently
}
