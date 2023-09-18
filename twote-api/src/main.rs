use schemas::hello::hello_service_server::HelloServiceServer;
use schemas::hello::{hello_service_server::HelloService, HelloReply, HelloRequest};
use tonic::{transport::Server, Request, Response, Status};
pub struct MyHelloService;

#[tonic::async_trait]
impl HelloService for MyHelloService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let greeting = request.into_inner().greeting;

        let reply = HelloReply {
            reply: format!("Hello, {}!", greeting),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let hello_service = MyHelloService;

    println!("Running on {}", addr);

    Server::builder()
        .add_service(HelloServiceServer::new(hello_service))
        .serve(addr)
        .await?;

    Ok(())
}
