use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: findfile (path) (keyword)");
        return;
    }

    let target_dir = &args[1];
    let keyword = &args[2];

    let target = path::PathBuf::from(target_dir);
    findfile(&target, keyword);
}

fn findfile(target: &path::PathBuf, keyword: &str) {
    let files = target.read_dir().expect("path not found");
    for dir_entry in files {
        let path = dir_entry.unwrap().path();
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }

        let fname = path.file_name().unwrap().to_string_lossy();
        if fname.find(keyword).is_none() {
            continue;
        }
        println!("{}", path.to_string_lossy());
    }
}
