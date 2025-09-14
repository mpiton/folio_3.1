use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use futures_util::future::BoxFuture;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashSet;
use tower::Service;

static MONGO_OPERATORS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("$where");
    set.insert("$ne");
    set.insert("$gt");
    set.insert("$lt");
    set.insert("$gte");
    set.insert("$lte");
    set.insert("$regex");
    set.insert("$in");
    set.insert("$nin");
    set.insert("$all");
    set.insert("$or");
    set.insert("$and");
    set.insert("$exists");
    set.insert("$type");
    set.insert("$mod");
    set.insert("$text");
    set.insert("$search");
    set
});

#[derive(Clone)]
pub struct MongoSanitizer;

impl Default for MongoSanitizer {
    fn default() -> Self {
        Self::new()
    }
}

impl MongoSanitizer {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn contains_mongo_injection(value: &Value) -> bool {
        match value {
            Value::Object(map) => {
                for (key, val) in map {
                    if key.starts_with('$') && MONGO_OPERATORS.contains(key.as_str()) {
                        return true;
                    }
                    if Self::contains_mongo_injection(val) {
                        return true;
                    }
                }
                false
            }
            Value::Array(arr) => arr.iter().any(Self::contains_mongo_injection),
            Value::String(s) => s.contains("$where") || s.contains("$ne"),
            _ => false,
        }
    }
}

impl<S> tower::Layer<S> for MongoSanitizer {
    type Service = MongoSanitizerMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        MongoSanitizerMiddleware { inner: service }
    }
}

#[derive(Clone)]
pub struct MongoSanitizerMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for MongoSanitizerMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            // Extraire et vérifier le corps de la requête
            let (parts, body) = request.into_parts();
            let bytes = match axum::body::to_bytes(body, usize::MAX).await {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Impossible de lire le corps de la requête"))
                        .unwrap())
                }
            };

            if !bytes.is_empty() {
                if let Ok(value) = serde_json::from_slice::<Value>(&bytes) {
                    if MongoSanitizer::contains_mongo_injection(&value) {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from("Tentative d'injection MongoDB détectée"))
                            .unwrap());
                    }
                }
            }

            // Reconstruire la requête
            let request = Request::from_parts(parts, Body::from(bytes));
            inner.call(request).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_safe_json() {
        let json = r#"{"name": "test", "value": 123}"#;
        let request = Request::builder()
            .method("POST")
            .body(Body::from(json))
            .unwrap();

        let service = tower::service_fn(|req: Request<Body>| async move {
            Ok::<_, std::convert::Infallible>(Response::new(req.into_body()))
        });

        let mut middleware = MongoSanitizerMiddleware { inner: service };
        let response = middleware
            .ready()
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_injection_attempt() {
        let json = r#"{"$where": "this.password.length > 0"}"#;
        let request = Request::builder()
            .method("POST")
            .body(Body::from(json))
            .unwrap();

        let service = tower::service_fn(|req: Request<Body>| async move {
            Ok::<_, std::convert::Infallible>(Response::new(req.into_body()))
        });

        let mut middleware = MongoSanitizerMiddleware { inner: service };
        let response = middleware
            .ready()
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_nested_injection_attempt() {
        let json = r#"{"query": {"nested": {"$ne": null}}}"#;
        let request = Request::builder()
            .method("POST")
            .body(Body::from(json))
            .unwrap();

        let service = tower::service_fn(|req: Request<Body>| async move {
            Ok::<_, std::convert::Infallible>(Response::new(req.into_body()))
        });

        let mut middleware = MongoSanitizerMiddleware { inner: service };
        let response = middleware
            .ready()
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
