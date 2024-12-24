use std::env;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let mut current_dir = current_dir.as_path();

    loop {
        dbg!(current_dir);
        if current_dir.join("Cargo.toml").is_file() {
            print!("🦀 ");
            return;
        }

        if let Some(parent) = current_dir.parent() {
            current_dir = parent;
        } else {
            break;
        }
    }

    // If no Cargo.toml is found, exit silently
}
