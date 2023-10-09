use schemas::frontend::profile_page_service_server::ProfilePageService;
use schemas::frontend::{GetProfilePageRequest, ProfilePage};
use tonic::{Request, Response, Status};

pub struct ProfilePageServiceImpl;

#[tonic::async_trait]
impl ProfilePageService for ProfilePageServiceImpl {
    async fn get_profile_page(
        &self,
        request: Request<GetProfilePageRequest>,
    ) -> Result<Response<ProfilePage>, Status> {
        unimplemented!()
    }
}
