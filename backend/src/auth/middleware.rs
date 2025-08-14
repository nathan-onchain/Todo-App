use actix_web::{dev::{ServiceRequest, ServiceResponse, forward_ready}, Error, HttpMessage};
use actix_web::body::EitherBody;
use actix_service::{Service, Transform};
use futures_util::future::{ready, Ready, LocalBoxFuture};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;
use std::rc::Rc;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String, // user ID or email
    exp: usize,
}

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService { service: Rc::new(service) }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization").and_then(|h| h.to_str().ok());

        if let Some(header) = auth_header {
            if let Some(token) = header.strip_prefix("Bearer ") {
                let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
                let validation = Validation::new(Algorithm::HS256);

                if let Ok(token_data) = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret.as_ref()),
                    &validation
                ) {
                    req.extensions_mut().insert(token_data.claims.sub);
                    let fut = self.service.call(req);
                    return Box::pin(async move {
                        fut.await.map(ServiceResponse::map_into_left_body)
                    });
                }
            }
        }

        Box::pin(async move {
            Ok(req.into_response(
                actix_web::HttpResponse::Unauthorized()
                    .body("Unauthorized")
                    .map_into_right_body(),
            ))
        })
    }
}
