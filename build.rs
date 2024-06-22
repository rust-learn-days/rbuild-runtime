use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

#[allow(clippy::single_component_path_imports)]
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arch = env::consts::ARCH;
    let url = match arch {
        "x86_64" => "https://github.com/labring/sealos/releases/download/v4.3.7/sealos_4.3.7_linux_amd64.tar.gz",
        "arm64" => "https://github.com/labring/sealos/releases/download/v4.3.7/sealos_4.3.7_linux_arm64.tar.gz",
        "aarch64" => "https://github.com/labring/sealos/releases/download/v4.3.7/sealos_4.3.7_linux_arm64.tar.gz",
        _ => "",
    };

    println!("cargo:rustc-env=ARCH={}", arch);
    println!("cargo:rustc-env=URL={}", url);
    let target_dir = Path::new("files");
    let target_file = target_dir.join("sealos");
    if target_file.exists() {
        println!("File already exists: {:?}", target_file);
        return Ok(());
    }
    let output_file = "sealos.tar.gz";
    let response = reqwest::get(url).await?;
    let path = Path::new(output_file);
    let mut file = File::create(path)?;
    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut file)?;
    println!("临时文件已下载到: {:?}", path);
    Command::new("tar")
        .arg("-zxf")
        .arg(output_file)
        .arg("sealos")
        .status()
        .expect("Failed to execute tar");

    // 删除下载的 tar.gz 文件
    fs::remove_file(output_file).expect("Failed to remove sealos.tar.gz");

    Command::new("chmod")
        .arg("a+x")
        .arg("sealos")
        .status()
        .expect("Failed to execute chmod");
    let target_dir = Path::new("files");

    fs::rename("sealos", target_dir.join("sealos")).expect("Failed to move sealos to files/");
    Ok(())
}
