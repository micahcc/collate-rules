use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Serialize, Deserialize, Debug)]
struct InstanceRule {
    id: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct InstanceUrls {
    streaming_api: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct InstanceStats {
    user_count: i64,
    status_count: i64,
    domain_count: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct InstanceStatus {
    max_characters: i64,
    max_media_attachments: i64,
    characters_reserved_per_url: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct MediaAttachmentConfig {
    supported_mime_types: Vec<String>,
    image_size_limit: i64,
    image_matrix_limit: i64,
    video_size_limit: i64,
    video_frame_rate_limit: i64,
    video_matrix_limit: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct PollConfig {
    max_options: i64,
    max_characters_per_option: i64,
    min_expiration: i64,
    max_expiration: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct InstanceConfiguration {
    statuses: InstanceStatus,
    media_attachments: MediaAttachmentConfig,
    polls: PollConfig,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountEmoji {
    shortcode: String,
    url: String,
    static_url: String,
    visible_in_picker: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContactAccountInfo {
    id: String,
    username: String,
    acct: String,
    display_name: String,
    locked: bool,
    bot: bool,
    discoverable: bool,
    group: bool,
    created_at: String,
    note: String,
    url: String,
    avatar: String,
    avatar_static: String,
    header: String,
    header_static: String,
    followers_count: i64,
    following_count: i64,
    statuses_count: i64,
    last_status_at: String,
    emojis: Vec<AccountEmoji>,
    fields: Vec<ContactField>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContactField {
    name: String,
    value: String,
    verified_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InstanceResponse {
    uri: String,
    title: String,
    short_description: String,
    description: String,
    email: String,
    version: String,
    urls: InstanceUrls,
    stats: InstanceStats,
    thumbnail: String,
    languages: Vec<String>,
    registrations: bool,
    approval_required: bool,
    invites_enabled: bool,
    configuration: InstanceConfiguration,
    contact_account: ContactAccountInfo,
    rules: Vec<InstanceRule>,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Server List, one per line in a file
    #[arg(short, long, required = true)]
    manifest: String,

    /// Output CSV of servers and their reason for blocking
    #[arg(short, long, required = true)]
    output: String,
}

async fn pull_data(server: String) -> Result<(), Box<dyn Error>> {
    println!("{}", server);
    let resp = reqwest::get(server)
        .await?
        .json::<InstanceResponse>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let err_msg = format!("Cannot open {}.txt", args.manifest);
    let reader = BufReader::new(File::open(args.manifest).expect(&err_msg));

    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        println!("{}", line);

        let ret = pull_data(line).await;
        match ret {
            Ok(v) => println!("working with version: {v:?}"),
            Err(e) => println!("error parsing header: {e:?}"),
        }
    }
}
