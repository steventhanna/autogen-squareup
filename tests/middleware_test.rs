#![cfg(feature = "locations")]

use std::sync::{Arc, Mutex};

use autogen_squareup::{apis::locations_api, Environment, SquareClient};
use reqwest_middleware::{Middleware, Next};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Clone, Default)]
struct Recorder {
    seen: Arc<Mutex<Vec<(String, String)>>>,
}

#[async_trait::async_trait]
impl Middleware for Recorder {
    async fn handle(
        &self,
        req: reqwest::Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        self.seen.lock().unwrap().push((
            req.method().to_string(),
            req.url().host_str().unwrap_or_default().to_string(),
        ));
        next.run(req, extensions).await
    }
}

#[tokio::test]
async fn middleware_sees_every_request() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/locations"))
        .and(header("Authorization", "Bearer test-token"))
        .and(header("Square-Version", "2026-07-15"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "locations": []
        })))
        .mount(&server)
        .await;

    let recorder = Recorder::default();
    let client = SquareClient::builder("test-token")
        .environment(Environment::Sandbox)
        .with(recorder.clone())
        .build();

    let mut config = client.config().clone();
    config.base_path = server.uri();

    let result = locations_api::list_locations(&config).await;
    assert!(result.is_ok());

    assert_eq!(
        *recorder.seen.lock().unwrap(),
        vec![("GET".to_string(), "127.0.0.1".to_string())]
    );
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}
