use std::{fs, path::Path};

fn main() {
    let tmp = Path::new("tmp");

    let _ = fs::remove_dir_all(tmp);

    fs::create_dir(tmp).unwrap();

    unsafe { redb::Database::create(tmp.join("database.redb"), 1 << 30).unwrap() };

    // initialize redb database
    // initialize lmdb database
}
