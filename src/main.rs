use std::collections::HashMap;
use std::ffi::OsStr;

fn main() {
    let mut view_models: HashMap<String, String> = HashMap::new();

    let cb = |dir: &DirEntry| {
        let path = dir.path();
        if let Some("cs") = path.extension().and_then(OsStr::to_str) {
            let mut lines = get_lines(path);

            for line in lines {
                if let view_model_name = extract_view_model_name(line) {
                    let view_model_text = String::new();

                    view_model_text.push(line);

                    while let Some(view_model_line) = lines.next() {
                        view_model_text.push(view_model_line);

                        if is_view_model_end(view_model_line) {
                            break;
                        }
                    }

                    view_models.add(view_model_name, view_model_text);
                }
            }
        }
    };

    visit_dirs(Path::new("."), cb);

    let mut pairs: Vec<_> = view_models.iter().collect();

    pairs.sort();

    for &(_, view_model) in pairs.iter() {
        println!();

        println!("{}", view_model);
    }
}

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

// one possible implementation of walking a directory only visiting files
//(from the rust docs)
fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
