use std::{env};
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{copy, Cursor};
use std::path::Path;
use std::time::Duration;
use reqwest::{Client, Response};
use clap::{Arg, ArgAction, Command, Parser};
use crate::wh_collection::{Collection};
use crate::wh_collections::Collections;
use crate::wh_search::Search;

pub mod wh_collection;
pub mod wh_collections;
pub mod wh_search;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Uploader username
    #[arg(short, long, required = false)]
    user: String,

    /// Collection name
    #[arg(short, long, required = false)]
    coll: String,

    /// Tag
    #[arg(short, long, required = false)]
    tag: String,
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let matches = Command::new("walrs")
        .arg(Arg::new("user")
            .short('u')
            .required(false)
            .action(ArgAction::Set)
        )
        .arg(Arg::new("collection")
            .short('c')
            .required(false)
            .action(ArgAction::Set)
        )
        .arg(Arg::new("tag")
            .short('t')
            .required(false)
            .action(ArgAction::Set)
        )
        .arg(Arg::new("path")
            .short('p')
            .required(false)
            .action(ArgAction::Set)
        )
        .get_matches();
    let cwd = &String::from("");
    let path = matches.get_one::<String>("path").unwrap_or(cwd);
    create_dir_all(Path::new(path)).unwrap_or_else(|_| panic!("Could not create dir {path}"));
    if let Some(u) = matches.get_one::<String>("user") {
        if let Some(c) = matches.get_one::<String>("collection") {
            println!("Searching collection for {u}");
            wh_collection_download_all(&client, u, c, path).await;
        } else {
            println!("Searching uploads by {u}");
            wh_user_download_all(&client, u, path).await;
        }
    } else if let Some(t) = matches.get_one::<String>("tag") {
        println!("Searching uploads with tag {t}");
        wh_search_download_all(&client, t, path).await;
    }
}


pub async fn download_link(url: &str, folder: &str) -> Result<(), Box<dyn Error>> {
    let download_dir = env::current_dir()?;
    let target = url;
    let response = reqwest::get(target).await?;
    let file_name = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("wh.img");
    let file_path = download_dir.join(folder).join(file_name);
    if file_path.exists() {
        println!("{file_name} already exists");
    } else {
        println!("Downloading {file_name}");
        let mut dest = File::create(file_path)?;
        let mut content = Cursor::new(response.bytes().await?);
        copy(&mut content, &mut dest)?;
    }
    Ok(())
}


async fn get(client: &Client, url: String) -> Result<Response, reqwest::Error> {
    client.get(url)
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await
}

async fn get_collections(client: &Client, url: String) -> Result<Collections, Box<dyn Error>> {
    let response = get(client, url).await?.json::<Collections>().await?;
    Ok(response)
}

async fn get_collection(client: &Client, url: String) -> Result<Collection, Box<dyn Error>> {
    let response = get(client, url).await?.json::<Collection>().await?;
    Ok(response)
}

async fn get_search(client: &Client, url: String) -> Result<Search, Box<dyn Error>> {
    let response = get(client, url).await?.json::<Search>().await?;
    Ok(response)
}

async fn wh_user_collections(client: &Client, user: &str) -> Collections {
    let url = format!("https://wallhaven.cc/api/v1/collections/{user}");
    match get_collections(client, url).await {
        Ok(value) => value,
        Err(_) => Collections::empty()
    }
}

async fn wh_user_collection(client: &Client, user: &str, collection_id: i32, page: i32) -> Collection {
    let url = format!("https://wallhaven.cc/api/v1/collections/{user}/{collection_id}?page={page}");
    match get_collection(client, url).await {
        Ok(value) => value,
        Err(_) => Collection::empty()
    }
}

async fn wh_search(client: &Client, q: &str, page: i32) -> Search {
    let url = format!("https://wallhaven.cc/api/v1/search?q={q}&page={page}");
    match get_search(client, url).await {
        Ok(value) => value,
        Err(_) => Search::empty()
    }
}

fn get_pages(result: Search, q: &str) -> i32 {
    let pages = result.meta.last_page + 1;
    if result.meta.total == 0 {
        println!("No results found for query {q}")
    }
    pages
}

async fn wh_search_download_all(client: &Client, q: &str, folder: &str) {
    let result = wh_search(client, q, 1).await;
    let pages = get_pages(result, q);
    for page in 1..pages {
        for search in wh_search(client, q, page).await.data.iter() {
            download_link(&search.path, folder).await
                .unwrap_or_else(|_| panic!("Error while searching tag: {q}"));
        }
    }
}

async fn wh_user_download_all(client: &Client, u: &str, folder: &str) {
    let result = wh_search(client, &format!("@{u}"), 1).await;
    let pages = get_pages(result, u);
    for page in 1..pages {
        for search in wh_search(client, &format!("@{u}"), page).await.data.iter() {
            download_link(&search.path, folder).await
                .unwrap_or_else(|_| panic!("Error while searching uploads by user: {u}"));
        }
    }
}

async fn wh_collection_download_all(client: &Client, u: &str, c:&str, folder: &str) {
    let mut collection_id=-1;
    for collection in wh_user_collections(client, u).await.data.iter() {
        if c.eq(&collection.label) {
            collection_id = collection.id;
        }
    }
    if collection_id > 0 {
        let result = wh_user_collection(client, u, collection_id, 1).await;
        let pages = result.meta.last_page + 1;
        for page in 1..pages {
            for search in wh_user_collection(client, u, collection_id, page).await.data.iter() {
                download_link(&search.path, folder).await
                    .unwrap_or_else(|_| panic!("Error while searching uploads by user: {u}"));
            }
        }
    }
    else {
        println!("User/Collection not found");
    }
}