use rusoto_core::{Region, RusotoError};
use rusoto_s3::{PutObjectRequest, DeleteObjectRequest, S3, S3Client};

pub fn create_s3_client() -> S3Client {
    let access_key = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not set");
    let secret_key = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not set");

    let region = Region::UsEast1;

    S3Client::new_with(
        rusoto_core::request::HttpClient::new().expect("Failed to create HTTP client"),
        rusoto_core::credential::StaticProvider::new_minimal(access_key, secret_key),
        region,
    )
}

pub fn insert_file_to_s3(s3_client: &S3Client, bucket_name: &str, file_name: &str, file_content: Vec<u8>) -> Result<(), RusotoError<rusoto_s3::PutObjectError>>{
    let request = PutObjectRequest {
        bucket: bucket_name.to_string(),
        key: file_name.to_string(),
        body: Some(file_content.into()),
        ..Default::default()
    };

    s3_client.put_object(request).sync()?;

    Ok(())

}