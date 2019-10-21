use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use chrono::{Utc};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};

use crate::modules::users::controllers::{ get_user_by_id };

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct CheckHmac;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for CheckHmac
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckHmacMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckHmacMiddleware { service })
    }
}

pub struct CheckHmacMiddleware<S> {
    service: S,
}

impl<S, B> Service for CheckHmacMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // Get 'Authorization' data from headers
        let auth_vec: Vec<&str> = req.headers().get("Authorization").unwrap().to_str().unwrap().split(':').collect();

        println!("{:#?}", req.head());
        //println!("{:#?}", auth_vec);
        let user_secret = get_user_by_id(&auth_vec[0]).unwrap().access_secret;
        println!("{}", construct_hmac_body(&req));
        Box::new(self.service.call(req).and_then(|res| {
            Ok(res)
        }))
    }
}

/* 
    Creates the request body which will be encrypted as hmac

    GET /photos/puppy.jpg HTTP/1.1
    Host: johnsmith.s3.amazonaws.com
    Date: Mon, 26 Mar 2007 19:37:58 +0000}

    Authorization: AKIAIOSFODNN7EXAMPLE:frJIUN8DYpKDtOLCwo//yllqDzg= 
 */
fn construct_hmac_body(req: &ServiceRequest) -> String {

    format!("{method} {uri} {version}\nHost: {host}\nTimestamp: {timestamp}", 
        method=req.head().method.as_str(),
        uri=&req.head().uri.to_string(),
        version=version_to_string(req.head().version),
        host=req.headers().get("host").unwrap().to_str().unwrap(),
        timestamp=&Utc::now().timestamp().to_string()
    )

}

fn version_to_string(ver: actix_web::http::Version) -> String {
    if ver == actix_web::http::Version::HTTP_09 {
        "HTTP/0.9".to_owned()
    } else if ver == actix_web::http::Version::HTTP_10 {
        "HTTP/1.0".to_owned()
    } else if ver == actix_web::http::Version::HTTP_11 {
        "HTTP/1.1".to_owned()
    } else if ver == actix_web::http::Version::HTTP_2 {
        "HTTP/2.0".to_owned()
    } else {
        "unknown".to_owned()
    }
}
