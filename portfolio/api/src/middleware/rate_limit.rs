use axum::{
    extract::ConnectInfo,
    http::{Request, Response, StatusCode},
};
use futures_util::future::BoxFuture;
use std::{
    collections::HashMap,
    net::IpAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use tower::Service;

#[derive(Debug, Clone)]
struct RateLimitState {
    attempts: u32,
    first_attempt: Instant,
}

#[derive(Clone)]
pub struct RateLimiter {
    // IP -> (attempts, first_attempt)
    state: Arc<Mutex<HashMap<IpAddr, RateLimitState>>>,
    max_requests: u32,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    async fn is_rate_limited(&self, ip: IpAddr) -> bool {
        let mut state = self.state.lock().await;
        let now = Instant::now();

        // Nettoyer les anciennes entrées
        state.retain(|_, v| now.duration_since(v.first_attempt) < self.window);

        if let Some(rate_state) = state.get_mut(&ip) {
            if now.duration_since(rate_state.first_attempt) < self.window {
                if rate_state.attempts >= self.max_requests {
                    return true;
                }
                rate_state.attempts += 1;
            } else {
                *rate_state = RateLimitState {
                    attempts: 1,
                    first_attempt: now,
                };
            }
        } else {
            state.insert(
                ip,
                RateLimitState {
                    attempts: 1,
                    first_attempt: now,
                },
            );
        }

        false
    }
}

impl<S> tower::Layer<S> for RateLimiter {
    type Service = RateLimitMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        RateLimitMiddleware {
            inner: service,
            limiter: self.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RateLimitMiddleware<S> {
    inner: S,
    limiter: RateLimiter,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for RateLimitMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static + From<String>,
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

    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let limiter = self.limiter.clone();

        Box::pin(async move {
            // Extraire l'IP du client
            let ip = if let Some(ConnectInfo(addr)) = request
                .extensions()
                .get::<ConnectInfo<std::net::SocketAddr>>()
            {
                addr.ip()
            } else {
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Impossible de déterminer l'IP du client".to_string().into())
                    .unwrap());
            };

            // Vérifier le rate limit
            if limiter.is_rate_limited(ip).await {
                return Ok(Response::builder()
                    .status(StatusCode::TOO_MANY_REQUESTS)
                    .body(
                        "Trop de requêtes. Veuillez réessayer plus tard."
                            .to_string()
                            .into(),
                    )
                    .unwrap());
            }

            inner.call(request).await
        })
    }
}
