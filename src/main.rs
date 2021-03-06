extern crate hyper;
extern crate futures;
extern crate serde_json;

use std::str;

use futures::future::Future;
use futures::Stream;

use hyper::{Method, StatusCode};
use hyper::server::{Http, Request, Response, Service};


use serde_json::Value;


struct RecieveEvent;

impl Service for RecieveEvent {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Method::Post, "/") => {
                Box::new(req.body().concat2().and_then(|foo| {
                    let json = str::from_utf8(&foo).unwrap();

                    let v: Value = serde_json::from_str(json).unwrap();
                    if v["event_type"].is_object() {
                        if v["event_type"]["id"].is_number() {
                            if v["event_type"]["id"] == 6 {
                                println!("!");
                            } else {
                                println!(".")
                            }
                        }
                    }
                    futures::future::ok(Response::new())
                }))
            }
            _ => {
                let mut response = Response::new();
                response.set_status(StatusCode::NotFound);
                response.set_body("");
                Box::new(futures::future::ok(response))
            }
        }
    }
}

fn main() {
    let addr = "127.0.0.1:6666".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(RecieveEvent)).unwrap();
    server.run().unwrap();
}
