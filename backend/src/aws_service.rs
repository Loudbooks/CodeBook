use anyhow::Result;
use aws_sdk_s3::config::{Credentials, Region, SharedCredentialsProvider};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{Client, Config};

pub struct AWSService {
    client: Client,
    bucket_name: String,
}

impl AWSService {
    pub async fn new(endpoint: &str, bucket_name: &str, access_key: &str, secret_key: &str) -> Result<Self> {
        let region = Region::new("auto");
        let credentials = Credentials::from_keys(access_key, secret_key, None);
        let shared_credentials = SharedCredentialsProvider::new(credentials);

        let config = Config::builder()
            .region(region)
            .credentials_provider(shared_credentials)
            .endpoint_url(endpoint)
            .build();

        let client = Client::from_conf(config);

        Ok(Self {
            client,
            bucket_name: bucket_name.to_string(),
        })
    }

    pub async fn get_file(&self, key: &str) -> Result<Vec<u8>> {
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await?;

        let data = resp.body.collect().await?;
        Ok(data.into_bytes().to_vec())
    }

    pub async fn put_file(&self, key: &str, data: &[u8]) -> Result<()> {
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(ByteStream::from(data.to_vec()))
            .send()
            .await?;
        Ok(())
    }

    pub async fn delete_file(&self, key: &str) -> Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await?;
        Ok(())
    }

    pub async fn list_files(&self) -> Result<Vec<String>> {
        let resp = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket_name)
            .send()
            .await?;

        let mut keys = Vec::new();
        for object in resp.contents.unwrap_or_default() {
            if let Some(key) = object.key {
                keys.push(key);
            }
        }

        Ok(keys)
    }
}
