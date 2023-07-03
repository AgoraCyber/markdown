use std::{fs::read_to_string, path::PathBuf};

pub fn read_test_data<F>(mut f: F)
where
    F: FnMut(&str),
{
    let dir: PathBuf = env!("CARGO_MANIFEST_DIR").into();

    let dir = dir.join("tests/data");

    let paths = dir.read_dir().unwrap();

    for path in paths {
        let path = path.unwrap().path();

        let md = read_to_string(path.clone()).unwrap();

        log::debug!("load test markdown document: {}", path.display());

        f(&md);
    }
}
