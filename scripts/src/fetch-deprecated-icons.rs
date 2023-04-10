use dotenv;
use reqwest;
use serde_json;
use simple_icons::DEPRECATED_ICONS_FILE_NAME;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

const GRAPHQL_QUERY: &str = "{
  repository(owner: \\\"simple-icons\\\", name: \\\"simple-icons\\\") {
    milestones(states: [OPEN], first:10) {
      nodes{
        title
        description
        url
        dueOn
        number
        pullRequests(states:[OPEN], first:100){
          nodes{
            url
            number
            files(first:30) {
              edges {
                node {
                  path
                  changeType
                }
              }
            }
          }
        }
      }
    }
  }
}";

#[tokio::main]
async fn main() {
    let tmp_file_path =
        Path::new(&env::temp_dir()).join(DEPRECATED_ICONS_FILE_NAME);
    // Don't execute the request if the cache exists
    if tmp_file_path.exists() {
        return;
    }

    dotenv::dotenv().ok();

    let client = reqwest::Client::new();
    let query = format!("{{\"query\":\"{}\"}}", GRAPHQL_QUERY);
    let resp: serde_json::Value = client
        .post("https://api.github.com/graphql")
        .bearer_auth(env::var("GITHUB_TOKEN").expect(concat!(
            "GITHUB_TOKEN must be set to get information from",
            " the Github API at build time."
        )))
        .header("User-Agent", "simple-icons-website")
        .body(query.replace("\n", ""))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    if let Some(message) = resp.get("message") {
        fs::remove_file(tmp_file_path).unwrap();
        panic!("Error retrieving data from GITHUB Graphql API: {}", message);
    }
    let mut tmp_file = fs::File::create(tmp_file_path).unwrap();
    writeln!(&mut tmp_file, "{}", resp.to_string()).unwrap();
}
