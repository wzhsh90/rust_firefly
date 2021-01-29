use actix_web::HttpResponse;
use serde::Serialize;
use crate::utils::tpl::HTML_TPL;

pub fn to_json<T>(value: &T) -> HttpResponse
    where
        T: ?Sized + Serialize, {
    return HttpResponse::Ok()
        .json(value);
}

pub fn to_html<T>(name: &str, data: &T) -> HttpResponse
    where
        T: Serialize, {
    let body = HTML_TPL.render(name, &data).unwrap();
    HttpResponse::Ok().body(body)
}