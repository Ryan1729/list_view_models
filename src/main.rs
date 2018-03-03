use std::collections::HashMap;
use std::ffi::OsStr;

fn main()  {
    let mut view_models: HashMap<String, String> = HashMap::new();

    {
        let mut cb = |dir: &DirEntry| {
            let path = dir.path();
            if let Some("cs") = path.extension().and_then(OsStr::to_str) {
                let mut lines = get_lines(path).unwrap().map(|result| { result.unwrap() });

                while let Some(line) = lines.next() {
                    if let Some(view_model_name) = extract_view_model_name(&line) {
                        let mut view_model_text = String::new();

                        view_model_text.push_str(&line);
                        view_model_text.push('\n');

                        while let Some(view_model_line) = lines.next() {
                            view_model_text.push_str(&view_model_line);
                            view_model_text.push('\n');

                            if is_view_model_end(&view_model_line) {
                                break;
                            }
                        }

                        view_models.insert(view_model_name, view_model_text);
                    }
                }
            }
        };

        visit_dirs(Path::new("."), &mut cb).unwrap();
    }
    let mut pairs: Vec<_> = view_models.iter().collect();

    pairs.sort();

    for &(_, view_model) in pairs.iter() {
        println!();

        println!("{}", view_model);
    }
}

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

fn extract_view_model_name(line: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"class\s+([A-Za-z_]*ViewModel)(?:[^A-Za-z_]|$)").unwrap();
    }

    RE.captures_iter(line).map(|c| c[1].to_string()).next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_view_model_name_in_best_case() {
        assert_eq!(
            Some("CheeseViewModel".to_string()),
            extract_view_model_name("class CheeseViewModel")
        );
    }

    #[test]
    fn test_extract_view_model_name_with_full_line() {
        assert_eq!(
            Some("CheeseViewModel".to_string()),
            extract_view_model_name("    public class CheeseViewModel {")
        );
    }

    #[test]
    fn test_extract_view_model_name_with_base_class() {
        assert_eq!(
            Some("CheeseViewModel".to_string()),
            extract_view_model_name("    public class CheeseViewModel : MilkViewModelBase {")
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_extract_view_model_name_does_not_extract_ViewModelMapping() {
        assert_eq!(
            None,
            extract_view_model_name("public static class ViewModelMapping")
        );
    }
}

fn is_view_model_end(line: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*\}\s*$").unwrap();
    }

    RE.is_match(line)
}

use std::io::{self, BufReader, Lines};
use std::io::prelude::*;
use std::fs::File;

fn get_lines<P : AsRef<Path>>(path: P) -> io::Result<Lines<BufReader<File>>> {
    let f = File::open(path)?;
    let f = BufReader::new(f);

    Ok(f.lines())
}

use std::fs::{self, DirEntry};
use std::path::Path;

// one possible implementation of walking a directory only visiting files
//(from the rust docs)
fn visit_dirs(dir: &Path, cb: &mut FnMut(&DirEntry)) -> io::Result<()> {
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
