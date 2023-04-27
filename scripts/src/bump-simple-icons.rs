use std::fs;
use std::process;

const NPM_REGISTRY_URL: &str = "https://registry.npmjs.org/simple-icons";

fn get_latest_version_from_registry() -> String {
    reqwest::blocking::get(NPM_REGISTRY_URL)
        .unwrap()
        .json::<serde_json::Value>()
        .unwrap()
        .get("dist-tags")
        .unwrap()
        .as_object()
        .unwrap()
        .get("latest")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

fn get_simple_icons_version_from_package_json(package_json: &str) -> String {
    package_json
        .lines()
        .find(|line| line.starts_with("    \"simple-icons\": \""))
        .unwrap()
        .split('"')
        .nth(3)
        .unwrap()
        .to_string()
}

fn main() {
    let latest_version = get_latest_version_from_registry();

    let package_json = fs::read_to_string("package.json").unwrap();
    let current_version =
        get_simple_icons_version_from_package_json(&package_json);

    if latest_version != current_version {
        let new_package_json = package_json.replacen(
            &format!("    \"simple-icons\": \"{}\"", current_version),
            &format!("    \"simple-icons\": \"{}\"", latest_version),
            1,
        );
        fs::write("package.json", new_package_json).unwrap();

        // Run npm to update package-lock.json
        match process::Command::new("npm")
            .arg("install")
            // Only update package-lock.json
            .arg("--package-lock-only")
            // Ignore postinstall script
            .arg("--ignore-scripts")
            .output()
        {
            Ok(output) => {
                if !output.status.success() {
                    panic!(
                        "Failed to execute npm install bumping simple-icons version: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
                println!("{}", latest_version);
            }
            Err(error) => {
                panic!(
                    "Failed to execute npm install bumping simple-icons version: {}",
                    error
                );
            }
        }
    }
}
