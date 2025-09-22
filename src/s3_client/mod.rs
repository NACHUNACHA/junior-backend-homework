use aws_sdk_s3::Client;
use aws_config::meta::region::RegionProviderChain;
use std::error::Error;
use std::path::Path;
use aws_sdk_s3::primitives::ByteStream;

pub async fn get_client() -> Result<Client, Box<dyn Error>> {
    let region_provider = RegionProviderChain::default_provider().or_else("ap-southeast-2");
    let config = aws_config::from_env().region(region_provider).load().await;

    Ok(Client::new(&config))
}

pub async fn upload(
    client: &Client,
    bucket: &str,
    key: &str,
    file_path: &Path,
    mime_type: &str,
) -> Result<(), Box<dyn Error>> {
    let body = ByteStream::from_path(file_path).await?;
    
    println!("{:?}", body);
    
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .content_type(mime_type)
        .body(body)
        .send()
        .await?;
    Ok(())
}


