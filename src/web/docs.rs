use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(title = "P1anet API", description = "OpenAPI docs for P1anet API."))]
pub struct ApiDoc;
