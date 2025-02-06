use std::fs;
use std::path::Path;

fn print_tree(dir: &Path, depth: usize) {
    if let Ok(entries) = fs::read_dir(dir) {
        let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| e.path());

        for (index, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let is_last = index == entries.len() - 1;

            let indent = "│   ".repeat(depth);
            let prefix = if is_last { "└── " } else { "├── " };

            if path.is_dir() {
                println!("{}{}📂 {}/", indent, prefix, path.file_name().unwrap().to_string_lossy());
                print_tree(&path, depth + 1);
            } else {
                println!("{}{}📄 {}", indent, prefix, path.file_name().unwrap().to_string_lossy());
            }
        }
    }
}

fn main() {
    let root_dir = Path::new(".");
    println!("📂 Project Structure:");
    print_tree(root_dir, 0);
}
