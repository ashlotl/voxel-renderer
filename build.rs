use std::{fs, time::Duration};

pub const RELOAD_PATH: &'static str = "_reload.ron";

fn main() {
    let cmd = cargo_metadata::MetadataCommand::new();
    let metadata = cmd.exec().unwrap();
    let mut path = metadata.target_directory.clone();
    assert_eq!(path.pop(), true);
    path.push(RELOAD_PATH);
    let reload_path_full = String::from(path.as_os_str().to_str().unwrap());

    let reloads_string = fs::read_to_string(&reload_path_full).unwrap();
    let mut reloads: Vec<String> = ron::from_str(&reloads_string).unwrap();
    let add = String::from(env!("CARGO_PKG_NAME"));
    if !reloads.contains(&add) {
        reloads.push(add);
    }

    let mut write = String::from("//This file is written to and managed automatically\n");
    let mut ron_string = ron::to_string(&reloads).unwrap();
    if ron_string.len() == 0 {
        ron_string = String::from("[]");
    }
    write.push_str(ron_string.as_str());

    let mut target_path = metadata.target_directory.clone();

    #[cfg(debug_assertions)]
    let mode = "debug";
    #[cfg(not(debug_assertions))]
    let mode = "release";

    let mut libname = String::from("lib");
    libname.push_str(env!("CARGO_PKG_NAME"));
    libname = libname.replace("-", "_");

    #[cfg(target_os = "linux")]
    let extension = ".so";

    libname.push_str(extension);

    target_path.push(mode);
    target_path.push(libname.as_str());

    println!("#{}", target_path);

    fs::write(
        target_path,
        "this file has been overwritten to prevent linking an old version",
    )
    .unwrap();

    fs::write(reload_path_full, write).unwrap();
}
