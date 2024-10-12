use crate::VirtualFileSystem;
use std::path::PathBuf;

#[test]
fn empty_by_default() {
    let vfs = VirtualFileSystem::new();

    assert_eq!(vfs.files.len(), 0);
}

#[test]
fn file_exists_after_write() {
    let mut vfs = VirtualFileSystem::new();
    let contents = &[1, 2, 3, 4];

    vfs.write(PathBuf::from("./data/file"), contents);

    assert!(vfs.exists(PathBuf::from("./data/file")));
    assert!(vfs.exists(PathBuf::from("./data/../data/file")));

    assert_eq!(
        vfs.read(PathBuf::from("./data/file")),
        Some(contents.to_vec())
    );
}

#[test]
fn remove_file() {
    let mut vfs = VirtualFileSystem::new();
    let contents = &[1, 2, 3, 4];

    vfs.write(PathBuf::from("./data/file"), contents);

    assert!(vfs.exists(PathBuf::from("./data/file")));

    vfs.remove(PathBuf::from("./data/../data/file"));

    assert!(!vfs.exists(PathBuf::from("./data/file")));
}
