#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use axum::{routing::any, Router, http::{Request, StatusCode}, body::Body, response::Response};
use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy::Fatal, ThreadsafeFunction},
};
use tower_http::services::ServeDir;
use std::{sync::Mutex, collections::HashMap};

#[napi]
struct AppFactory {
  engine: Mutex<Router>,
}

#[napi]
impl AppFactory {
  #[napi]
  pub fn initialize(
    astro_handler: ThreadsafeFunction<serde_json::Value, Fatal>,
    client_path: String,
  ) -> Self {
    let mut router = Router::new();
    let astro_handler_all = astro_handler.clone();
    router = router.route("/", any(|req| async move { handler(req, astro_handler).await }));
    let _astro = format!("{}/_astro", client_path);
    router = router.nest_service("/_astro", ServeDir::new(_astro));
    router = router.nest_service("/favicon", ServeDir::new(client_path));
    router = router.route("/*all", any(|req| async move { handler(req, astro_handler_all).await }));
    AppFactory {
      engine: Mutex::new(router),
    }
  }

  #[napi]
  pub async fn serve(&self, port: u16, host: Option<String>) -> Result<bool> {
    let engine = self.engine.lock().unwrap().clone();
    let host = host.unwrap_or("127.0.0.1".to_string());
    let addr = format!("{}:{}", host, port);
    axum::Server::bind(&addr.parse().unwrap())
      .serve(engine.into_make_service())
      .await
      .unwrap();
    Ok(true)
  }
}

async fn handler(req: Request<Body>, astro_handler: ThreadsafeFunction<serde_json::Value, Fatal>) -> Response<Body> {
  let mut url = req.uri().path().to_string();
  if req.uri().query().is_some() {
    let query = req.uri().query().unwrap();
    url = format!("{}?{}", url, query);
  }
  let request = serde_json::json!({
    "method": req.method().as_str(),
    "url": url,
    "headers": req.headers().iter().map(|(k, v)| (k.as_str(), v.to_str().unwrap())).collect::<serde_json::Value>(),
  });
  let response = serde_json::json!({
    "headersSent": false,
  });
  let js_res = astro_handler.call_async::<Promise<(u16, HashMap<String, String>, Vec<u8>)>>(serde_json::json!({
    "request": request,
    "response": response,
  }));
  let mut res = Response::builder();
  let js_data = match js_res.await {
    Ok(async_body) => match async_body.await {
      Ok(body) => body,
      Err(_) => (
        StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        HashMap::new(),
        vec![],
      )
    }
    Err(_) => (
      StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
      HashMap::new(),
      vec![],
    )
  };
  for (k, v) in js_data.1 {
    res = res.header(k, v);
  }
  res = res.header("Powered-By", "AstroSurf");
  let status = StatusCode::from_u16(js_data.0).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
  res = res.status(status);
  let body = res.body(Body::from(js_data.2));
  body.unwrap()
}
