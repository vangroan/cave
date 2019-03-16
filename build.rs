extern crate fs_extra;
extern crate glob;
extern crate itertools;

use std::env;
use std::path::Path;

use itertools::concat;

/// Copies all resources to the out directory.
fn copy_resources() {
    use fs_extra::dir::{copy, CopyOptions};

    let out_dir = env::var("OUT_DIR").unwrap();
    let res_dir = "resources";
    let dest_dir = Path::new(&out_dir);

    eprintln!("path: {:?}", dest_dir);

    let mut options = CopyOptions::new();
    options.overwrite = true;

    copy(res_dir, dest_dir, &options).unwrap();
}

/// Copies exported files from the `art` directory to the `resouces` directory.
fn copy_art() {
    use fs_extra::{copy_items, dir, file};
    use std::env;
    use std::path::PathBuf;

    let extensions = vec!["json", "png"];

    let art_dir = "art";
    let res_dir = "resources";

    let src = extensions
        .iter()
        .map(|ext| {
            glob::glob(&format!("art/**/*.{}", ext))
                .unwrap()
                .filter_map(Result::ok)
        })
        .flat_map(|it| it)
        .collect::<Vec<PathBuf>>();

    {
        // Build directory paths for creation
        let mut dest = src
            .iter()
            // Remove `art` from the front of the path.
            .map(|path| path.strip_prefix(art_dir).unwrap())
            .map(|path| Path::new(res_dir).join(path))
            // Remove filename from the back of the path
            .map(|path| {
                let mut p = path.clone();
                p.pop();
                p
            })
            .collect::<Vec<_>>();

        // Dedup requires sorted
        dest.sort_by(|a, b| a.cmp(b));
        dest.dedup_by(|a, b| a == b);

        // Ensure target directories exist.
        for path in dest {
            println!("{:?}", path);
            dir::create_all(&path, false).unwrap();
        }

        let mut options = dir::CopyOptions::new();
        options.overwrite = true;
    }

    // Finally do copy
    for path_src in &src {
        let mut path_dest = path_src.clone();
        // Remove `art` from the front of the path.
        path_dest = path_src.strip_prefix(art_dir).unwrap().to_path_buf();
        path_dest = Path::new(res_dir).join(path_dest);

        let mut options = file::CopyOptions::new();
        options.overwrite = true;

        file::copy(path_src, path_dest, &options).unwrap();
    }
}

fn main() {
    copy_art();
    copy_resources();
}
