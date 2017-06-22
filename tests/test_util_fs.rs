extern crate env_logger;
extern crate tempdir;

extern crate xflow;

use xflow::util::fs::{model_from_fs, model_to_fs};

use tempdir::TempDir;
use std::env;

mod helper;

#[test]
fn test_load_basic_project() {
    let _ = env_logger::init();
    let model = model_from_fs(&"resource/projects/basic");
    println!("Model is {:?}", model);
}


#[test]
fn test_model_to_and_from_fs() {
    let _ = env_logger::init();
    let root_a = env::current_dir();

    let model_a = model_from_fs(&"resource/projects/basic").unwrap();

    let root_b = TempDir::new("model_b");
    let root_b = root_b.ok().expect("Should have created a temp directory");

    println!("CREATED {}", root_b.path().display());
    let root_b_path = format!("{}", root_b.path().display());
    model_to_fs(&model_a, &root_b_path);

    let model_b = model_from_fs(&root_b_path).unwrap();

    assert_eq!(model_a.doc.xflows.len(), model_b.doc.xflows.len());


}
