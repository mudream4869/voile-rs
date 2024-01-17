use crate::appstate::appstate::SharedAppState;
use actix_web::{post, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginParams {
    #[serde(default)]
    pub password: String,
}

#[post("/login/")]
async fn login_user(
    app_state: web::Data<SharedAppState>,
    login_params: web::Json<LoginParams>,
    session: actix_session::Session,
) -> actix_web::Result<actix_web::HttpResponse> {
    if !app_state
        .lock()
        .unwrap()
        .config_handler
        .auth(&login_params.password)?
    {
        return Ok(actix_web::HttpResponse::Unauthorized().finish());
    }

    let username = app_state
        .lock()
        .unwrap()
        .config_handler
        .get_user_config()?
        .username;

    session.insert("auth_username", &username)?;

    Ok(actix_web::HttpResponse::Ok().finish())
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(login_user);
}

use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::FromRequest;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};

use futures_util::{future::LocalBoxFuture, FutureExt};

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static, // update here
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service), // convert S to Rc<S>
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    // service: S,
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static, // update here
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Do something with the request here
        let (req, pl) = req.into_parts();
        let ses = actix_session::Session::extract(&req).into_inner().unwrap();
        let req = ServiceRequest::from_parts(req, pl);

        if let Ok(Some(_)) = ses.get::<String>("auth_username") {
            // OK
        } else {
            let http_res = HttpResponse::Unauthorized().finish();
            let (http_req, _) = req.into_parts();
            let res = ServiceResponse::new(http_req, http_res);
            // Map to R type
            return (async move { Ok(res.map_into_right_body()) }).boxed_local();
        }

        // Clone the service to keep reference after moving into async block
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Continue with the next middleware / handler
            let res = service.call(req).await?;
            // Map to L type
            Ok(res.map_into_left_body())
        })
    }
}
