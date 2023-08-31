use comfy_table::{Cell, Row, Table, ContentArrangement};
use dateparser::parse;
use reqwest::Result;
use serde::Deserialize;
use serde_json::json;
use std::env;

const API_URL: &str = "https://zappy.sh";

#[derive(Deserialize, Debug)]
struct CreateResponse {
    created: bool,
    error: Option<String>,
}

#[derive(Deserialize, Debug)]
struct GetRequestsResponse {
    success: bool,
    error: Option<String>,
    data: Option<AllRequests>,
}

#[derive(Deserialize, Debug)]
struct AllRequests {
    count: i32,
    requests: Vec<Request>,
}

#[derive(Deserialize, Debug)]
struct Request {
    ip: String,
    user_agent: String,
    user_id: String,
    referer: String,
    #[serde(rename = "CreatedAt")]
    created_at: String,
}

pub(crate) fn create_alias(alias_name: &str, url: &str) {
    let msg_ending = format!("alias {} with url {}", alias_name, url);
    let client = reqwest::blocking::Client::new();
    let endpoint = format!("{}/alias/create", API_URL);
    let res = client
        .post(endpoint)
        .json(&json!({
            "name": alias_name,
            "url": url,
        }))
        .send()
        .unwrap();
    let json: CreateResponse = res.json().unwrap();
    if json.created {
        println!("Successfully created {}", msg_ending);
    } else {
        println!(
            "Failed to create {} because {}",
            msg_ending,
            json.error.unwrap()
        );
    }
}

pub(crate) fn get_requests(alias_name: &str) -> Result<()> {
    let api_key = env::var("ZAPPY_API_KEY").expect("API_KEY not set");
    let auth_header = format!("Bearer {}", api_key);
    let client = reqwest::blocking::Client::new();
    let endpoint = format!("{}/requests/{}", API_URL, alias_name);
    let res = client
        .get(&endpoint)
        .header("Authorization", auth_header)
        .send()?
        .json::<GetRequestsResponse>()?;
    if res.success {
        let data = res.data.expect("no requests");
        println!("Total of {} requests", data.count);
        print_requests(data.requests);
    } else {
        panic!(
            "Failed to get requests for alias {} because {}",
            alias_name,
            res.error.unwrap()
        );
    }

    Ok(())
}

fn print_requests(requests: Vec<Request>) {
    let mut table = Table::new();
    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(120)
        .set_header(vec!["IP", "User Agent", "User ID", "Referer", "Created At"]);
    for request in requests {
        let created_at = parse(&request.created_at).expect("could not parse date");
        table.add_row(Row::from(vec![
            Cell::new(&request.ip),
            Cell::new(&request.user_agent),
            Cell::new(&request.user_id),
            Cell::new(&request.referer),
            Cell::new(created_at.format("%B %d, %Y, %H:%M:%S").to_string()),
        ]));
    }

    println!("{table}");
}
