
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths( 
    ),
    // components(schemas(User)),
    tags(
        (name = "/", description = "Hello"),
    ),
  
)]
pub struct ApiDoc;
