use tonic::{transport::Server, Request, Response, Status};
use distributed::node_server::{Node, NodeServer};
use distributed::{MessageRequest, MessageResponse};


pub mod distributed {
    tonic::include_proto!("distributed");
}

#[derive(Debug, Default)]
pub struct DSNode {}

#[tonic::async_trait]
impl Node for DSNode {
    async fn send_message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = MessageResponse {
            response: format!("Hello {}!", request.into_inner().message).into(),
        };

        Ok(Response::new(response))
    }
}


pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let node = DSNode::default();

    println!("Node listening on {}", addr);

    Server::builder()
        .add_service(NodeServer::new(node))
        .serve(addr)
        .await?;

    Ok(())
}
