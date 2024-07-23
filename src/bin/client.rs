use tari_rpc::Empty;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client =
        tari_rpc::base_node_client::BaseNodeClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(Empty {});

    let response = client.get_network_status(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
