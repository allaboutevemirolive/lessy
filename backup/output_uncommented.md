use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::os::unix::fs::MetadataExt;
use std::io;

struct Totals {
    dirs: usize,
    files: usize,
    size: u64,
}

















fn walk(dir: &str, prefix: &str, counts: &mut Totals) -> io::Result<()> {
    let mut paths: Vec<_> = fs::read_dir(dir)?
        .map(|entry| entry.unwrap().path())
        .collect();
    let mut index = paths.len();

    paths.sort_by(|a, b| {
        let aname = a.file_name().unwrap().to_str().unwrap();
        let bname = b.file_name().unwrap().to_str().unwrap();
        aname.cmp(bname)
    });

    for path in paths {
        let name = path.file_name().unwrap().to_str().unwrap();
        index -= 1;

        if index == 0 {
            println!("{}└── {}", prefix, name);
            if path.is_dir() {
                walk(
                    &format!("{}/{}", dir, name),
                    &format!("{}    ", prefix),
                    counts,
                )?;
            }
        } else {
            println!("{}├── {}", prefix, name);
            if path.is_dir() {
                walk(
                    &format!("{}/{}", dir, name),
                    &format!("{}│   ", prefix),
                    counts,
                )?;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let dirname = "/home/nemesis/Documents/Github/test/trie"; 


    let mut counts = Totals {
        dirs: 0,
        files: 0,
        size: 0,
    };

    walk(&dirname, "", &mut counts)?;


    println!("=========================================================");



    println!("=========================================================");

    Ok(())
}
