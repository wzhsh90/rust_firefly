use std::task::{Context, Poll};

use actix_web::dev::{ServiceRequest, ServiceResponse, Transform, Service};
use actix_web::{http, Error, HttpResponse};
use futures::future::{ok, Either, Ready};
use actix_session::UserSession;
use crate::router::{SEESION_USER_NAME, LoginUser};
use serde_json::json;

pub struct CheckLogin;

impl<S, B> Transform<S> for CheckLogin
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckLoginMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckLoginMiddleware { service })
    }
}

pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service for CheckLoginMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut is_logged_in = false;
        let url = req.path();
        let exclude = vec!["login", "static", "file", "doc"];
        for item in exclude {
            if url.contains(item) {
                is_logged_in = true;
                break;
            }
        }
        if !is_logged_in{
            let req_session = req.get_session();
            let login_opt = req_session.get::<LoginUser>(SEESION_USER_NAME);
            if login_opt.is_ok() {
                let login_ok = login_opt.unwrap();
                if !login_ok.is_none() {
                    if !login_ok.unwrap().id.is_empty() {
                        is_logged_in = true;
                    }
                }
            }
        }
        if is_logged_in || url == "/" || url == ""{
            Either::Left(self.service.call(req))
        } else {
            let ajax_req = req.headers().get("X-Requested-With");
            if ajax_req.is_some() {
                let rest = json!({
                        "code":100
                    });
                Either::Right(ok(req.into_response(
                    HttpResponse::Ok()
                        .json(rest)
                        .into_body(),
                )))
            } else {
                Either::Right(ok(req.into_response(
                    HttpResponse::Found()
                        .header(http::header::LOCATION, "/login")
                        .finish()
                        .into_body(),
                )))
            }
        }
    }
}