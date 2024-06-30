use tonic::transport::Channel;
use distributed::node_client::NodeClient;
use distributed::MessageRequest;

pub mod distributed {
    tonic::include_proto!("distributed");
}


pub async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NodeClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(MessageRequest {
        message: "world".into(),
    });

    let response = client.send_message(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
