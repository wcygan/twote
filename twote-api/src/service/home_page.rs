use schemas::frontend::home_page_service_server::HomePageService;
use schemas::frontend::HomePage;
use tonic::{Request, Response, Status};

pub struct HomePageServiceImpl;

#[tonic::async_trait]
impl HomePageService for HomePageServiceImpl {
    async fn get_home_page(&self, request: Request<()>) -> Result<Response<HomePage>, Status> {
        unimplemented!()
    }
}
