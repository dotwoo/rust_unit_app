/*
 * @Author: your name
 * @Date: 2021-03-23 15:14:16
 * @LastEditTime: 2021-03-23 15:16:20
 * @LastEditors: your name
 * @Description: In User Settings Edit
 * @FilePath: /rust_unit_app/src/build.rs
 */

use std::{env, path::PathBuf};

fn main() {
    let library_name = "unit";
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let library_dir = root.join("src");
    println!("cargo:rustc-link-lib=static={}", library_name);
    println!(
        "cargo:rustc-link-search=native={}",
        env::join_paths(&[library_dir]).unwrap().to_str().unwrap()
    );
}
