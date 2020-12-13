use std::error::Error;
use std::path::Path;
use futures_util::stream::TryStreamExt;
use hyper::{Body, Client};
use hyperlocal::{UnixClientExt, Uri};

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

    println!("{}", String::from_utf8(bytes)?);

    Ok(())
}
