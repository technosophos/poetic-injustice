extern crate actix_web;
extern crate rand;

use actix_web::{server, App, HttpRequest, HttpResponse, Responder};
use rand::prelude::*;
use std::env::var as envar;

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(spout_poetry))
            .resource("/healthz", |r| r.f(mete_injustice))
    })
    .bind("0.0.0.0:8000")
    .expect("cannot bind to host")
    .run();
}

const POET: &'static str = "T S Eliot";
const POEM: &'static str = "The Hollow Men";
const QUOTE: &'static str = r#"
    This is the way the world ends
    This is the way the world ends
    This is the way the world ends
    Not with a bang but a whimper.
"#;

fn spout_poetry(_req: &HttpRequest) -> impl Responder {
    let poet = envar("POET").unwrap_or(POET.into());
    let poem = envar("POEM").unwrap_or(POEM.into());
    let quote = envar("QUOTE").unwrap_or(QUOTE.into());

    format!("{}\n--'{}' by {}\n", quote, poem, poet)
}
fn mete_injustice(_req: &HttpRequest) -> HttpResponse {
    let x: u8 = random();

    // Fail about 1% of the time
    if x % 100 == 0 {
        return HttpResponse::NotFound().body("Oops, I did it again");
    }
    HttpResponse::Ok().body("success")
}
