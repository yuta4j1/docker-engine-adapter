use std::error::Error;
use std::path::Path;

use futures_util::stream::TryStreamExt;
use hyper::{Client};
use hyperlocal::{Uri, UnixClientExt};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct Container {
    #[serde(rename(deserialize = "Id"))]
    id: String,
    #[serde(rename(deserialize = "Names"))]
    names: Vec<String>,
    #[serde(rename(deserialize = "ImageID"))]
    image_id: String,
    #[serde(rename(deserialize = "Command"))]
    command: String,
    #[serde(rename(deserialize = "State"))]
    state: String,
    #[serde(rename(deserialize = "Status"))]
    status: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let path = Path::new("/var/run/docker.sock");
    let url = Uri::new(path, "/v1.40/containers/json");
    let client = Client::unix();
    let response_body = client.get(url.into()).await?.into_body();
    let bytes = response_body
        .try_fold(Vec::default(), |mut buf, bytes| async {
            buf.extend(bytes);
            Ok(buf)
        })
        .await?;
    
    let json_str = String::from_utf8(bytes)?;
    let deserialized: Vec<Container> = serde_json::from_str(&json_str).unwrap();
    println!("{:?}", deserialized);

    Ok(())
}
