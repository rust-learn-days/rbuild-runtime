use rust_embed::Embed;

#[derive(Embed)]
#[folder = "files/"]
struct Asset;

fn main() {
    println!("Hello, world!");
    let asset = Asset::get("sealos").unwrap();
    println!("asset: {:?}", asset.data.len());
}
