use actix_web::{web, HttpResponse};
use crate::gcd::gcd;
use crate::model::GcdParameters;

pub async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(r#"
            <title>GCD Calculator</title>
            <form action="/gcd2" method="post">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit">Compute GCD</button>
            </form>
        "#
        )
}

pub async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 { 
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring");
    };
    
    let response = format!(r#"
       The greatest common divisor of the numbers {} and {} is <b>{}</b>
    "#, form.n, form.m, gcd(form.n, form.m));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response)
}