use tonic::{Request, Response, Status};

use schemas::hello::{hello_service_server::HelloService, HelloReply, HelloRequest};

pub struct HelloServiceImpl;

#[tonic::async_trait]
impl HelloService for HelloServiceImpl {
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
