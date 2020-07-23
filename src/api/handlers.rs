///// 404 handler
// pub(crate) async fn handle404(req: HttpRequest) -> Result<HttpResponse> {
//     info!("404: {:?}", req);

//     Ok(HttpResponse::build(StatusCode::NOT_FOUND)
//         .content_type("application/json; charset=utf-8")
//         .body("{ \"error\": \"404: NOT FOUND\" }"))
// }
