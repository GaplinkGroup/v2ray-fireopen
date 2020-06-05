#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate env_logger;
use std::convert::Infallible;
use futures::future::Either;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

mod error;
mod ipset;

use {
    hyper::{
        // Miscellaneous types from Hyper for working with HTTP.
        Body, Client, Request, Response, Server, Uri, StatusCode,

        // This function turns a closure which returns a future into an
        // implementation of the the Hyper `Service` trait, which is an
        // asynchronous function from a generic `Request` to a `Response`.
        service::service_fn,
        service::make_service_fn,
    },
    hyper_tls::HttpsConnector,
    std::net::SocketAddr,
};
use route_recognizer::Router;
use route_recognizer::Params;
use std::env;
use lazy_static::lazy_static;

type HttpClient = Client<hyper::client::HttpConnector>;
type HttpsClient = Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;

lazy_static! {
    static ref EXPECT_AUTH_TOKEN: String = env::var("EXPECT_AUTH_TOKEN").unwrap();
}

async fn response_forbidden() -> Result<Response<Body>, hyper::Error> {
    let mut rep = Response::new(Body::from("{\"message\":\"401 Unauthorized\"}"));
    *rep.status_mut() = StatusCode::FORBIDDEN;
    Ok(rep)
}

async fn serve_req(_client: HttpsClient, req: Request<Body>, params: Params) -> Result<Response<Body>, hyper::Error> {
    // check PRIVATE-TOKEN
    if &params["pwd"] != &*EXPECT_AUTH_TOKEN { return response_forbidden().await; }

    // add ip to ipset
    if let Some(cf_connecting_ip) = req.headers().get("CF-Connecting-IP") {
        if let Ok(ip) = cf_connecting_ip.to_str() {
            if let Ok(client_ip) = IpAddr::from_str(ip) {
                ipset::add_to_ipset(client_ip, "v2rayallow");
                info!("v2rayallow ipset add:{}", client_ip);
            };
        };
    };

    let context = "dm1lc3M6Ly9ldzBLSUNBaWRpSTZJQ0l5SWl3TkNpQWdJbkJ6SWpvZ0lrQlRVMUpUVlVJdDVMK0U1NzJYNXBhdlZqSXdMZVM3bU9pMHVlYU9xT2lOa0RwMExtTnVMMFZIU2tsNWNtd2lMQTBLSUNBaVlXUmtJam9nSW5SaGJucG9aVzR1Y0hjaUxBMEtJQ0FpY0c5eWRDSTZJQ0l6TkRReklpd05DaUFnSW1sa0lqb2dJalJrWVdZd1pqSmhMVEpoTVRndE5EaGlNQzFsWWpsaExUZ3pPRGRsWmpka01EZGtPQ0lzRFFvZ0lDSmhhV1FpT2lBaU1UQXdJaXdOQ2lBZ0ltNWxkQ0k2SUNKM2N5SXNEUW9nSUNKMGVYQmxJam9nSW01dmJtVWlMQTBLSUNBaWFHOXpkQ0k2SUNKMFlXNTZhR1Z1TG5CM0lpd05DaUFnSW5CaGRHZ2lPaUFpTHpRd05DSXNEUW9nSUNKMGJITWlPaUFpZEd4eklnMEtmUT09";

    Ok(Response::new(Body::from(context)))
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut router = Router::new();
    router.add("/v2ray/:pwd/list", "v2ray_list");

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let make_service = make_service_fn(move |_| {
        let client = client.clone();
        let router = router.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| {
            match router.recognize(req.uri().path()) {
                // match only
                Ok(route_recognizer::Match{ handler, params }) => Either::Left(serve_req(client.clone(), req, params)),
                _ => Either::Right(response_forbidden()),
            }
            
        } )) }
    });

    let server = Server::bind(&addr).serve(make_service);


    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
