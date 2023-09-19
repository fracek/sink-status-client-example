use color_eyre::Result;

use self::status::status_client::StatusClient;
use self::status::GetStatusRequest;

pub mod status {
    tonic::include_proto!("apibara.sink.v1");
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = StatusClient::connect("http://localhost:8118").await?;

    let request = tonic::Request::new(GetStatusRequest {});
    let response = client.get_status(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}
