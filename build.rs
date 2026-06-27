//! 将 admin-web 文件图标同步到 Rocket 静态目录，保证前后台 URL 一致（`/static/fileicon/*`）

use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let src = root.join("admin-web/public/fileicon");
    let dst = root.join("static/fileicon");

    println!("cargo:rerun-if-changed={}", src.display());

    if !src.is_dir() {
        println!(
            "cargo:warning=文件图标源目录不存在，已跳过同步: {}",
            src.display()
        );
        return;
    }

    if let Err(e) = sync_dir(&src, &dst) {
        panic!("同步 fileicon 到 static 失败: {e}");
    }
}

fn sync_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        if !entry.file_type()?.is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("svg") {
            continue;
        }
        let name = entry.file_name();
        fs::copy(&path, dst.join(&name))?;
        println!("cargo:rerun-if-changed={}", path.display());
    }

    Ok(())
}
