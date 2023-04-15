use config::DEPRECATED_ICONS_FILE_NAME;
use serde_json;
use std::env;
use std::fs;
use std::path::Path;

/// Deprecated icons for next versions of Simple Icons
pub struct DeprecatedIcon {
    pub slug: String,
    pub removal_at_version: String,
    pub milestone_number: String,
    pub milestone_due_on: String,
    pub milestone_url: String,
    pub pull_request_number: String,
    pub pull_request_url: String,
}

/**
 * Get all the icons that will be removed in the next major versions
 * ordered by version.
 **/
pub fn fetch_deprecated_simple_icons() -> Vec<DeprecatedIcon> {
    let tmp_file_path =
        Path::new(&env::temp_dir()).join(DEPRECATED_ICONS_FILE_NAME);
    if !tmp_file_path.exists() {
        panic!("You need to run the script fetch-deprecated-icons.rs before building")
    }

    let resp: serde_json::Value = serde_json::from_str::<serde_json::Value>(
        &fs::read_to_string(&tmp_file_path).unwrap(),
    )
    .unwrap();

    if let Some(message) = resp.get("message") {
        fs::remove_file(tmp_file_path).unwrap();
        panic!("Error retrieving data from GITHUB Graphql API: {}", message);
    }

    let milestones_data = resp
        .get("data")
        .unwrap()
        .get("repository")
        .unwrap()
        .get("milestones")
        .unwrap()
        .get("nodes")
        .unwrap()
        .as_array()
        .unwrap();

    let mut deprecated_icons: Vec<DeprecatedIcon> = Vec::new();

    for milestone_data in milestones_data.into_iter() {
        let title = milestone_data.get("title").unwrap().as_str().unwrap();
        let removal_at_version = title.replace("v", "");
        let milestone_url = milestone_data.get("url").unwrap();
        let milestone_number = milestone_data.get("number").unwrap();
        let milestone_due_on = milestone_data.get("dueOn").unwrap();

        let pull_requests_data = milestone_data
            .get("pullRequests")
            .unwrap()
            .get("nodes")
            .unwrap()
            .as_array()
            .unwrap();
        for pull_request_data in pull_requests_data.into_iter() {
            let pull_request_url = pull_request_data.get("url").unwrap();
            let pull_request_number = pull_request_data.get("number").unwrap();

            let files_data = pull_request_data
                .get("files")
                .unwrap()
                .get("edges")
                .unwrap()
                .as_array()
                .unwrap();
            for file_data in files_data.into_iter() {
                let change_type =
                    file_data.get("node").unwrap().get("changeType").unwrap();
                if change_type != "DELETED" {
                    continue;
                }

                let path = file_data.get("node").unwrap().get("path").unwrap();
                let slug =
                    path.to_string().replace("icons/", "").replace(".svg", "");

                let deprecated_icon = DeprecatedIcon {
                    slug,
                    removal_at_version: removal_at_version.to_string(),
                    milestone_number: milestone_number.to_string(),
                    milestone_due_on: milestone_due_on.to_string(),
                    milestone_url: milestone_url.to_string(),
                    pull_request_number: pull_request_number.to_string(),
                    pull_request_url: pull_request_url.to_string(),
                };
                deprecated_icons.push(deprecated_icon);
            }
        }
    }

    deprecated_icons
}
