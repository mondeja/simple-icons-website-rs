use config::CONFIG;
use std::fs;
use std::io::Write;
use std::path;

fn main() {
    let cname_asset_path = path::Path::new("app/assets/CNAME");

    if cname_asset_path.exists() {
        fs::remove_file(cname_asset_path).unwrap();
    }
    let mut cname_file = fs::File::create(cname_asset_path).unwrap();

    cname_file
        .write_all(CONFIG.domain.as_bytes())
        .expect("Unable to write CNAME asset file")
}
