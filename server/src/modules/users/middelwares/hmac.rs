use actix_service::{Service, Transform};
use actix_web::error::PayloadError;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use bytes::BytesMut;
use futures::future::{ok, FutureResult};
use futures::stream::Stream;
use futures::{Future, Poll};
use std::cell::RefCell;
use std::rc::Rc;
use serde_json::Value;


pub struct CheckHmac;

impl<S: 'static, B> Transform<S> for CheckHmac
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
        ok(CheckHmacMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct CheckHmacMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for CheckHmacMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
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

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let mut svc = self.service.clone();

        Box::new(
            req.take_payload()
                .fold(BytesMut::new(), move |mut body, chunk| {
                    body.extend_from_slice(&chunk);
                    Ok::<_, PayloadError>(body)
                })
                .map_err(|e| e.into())
                .and_then(move |bytes| {
                    let mut result: Value = serde_json::from_str(std::str::from_utf8(&bytes).unwrap())
                    .expect("JSON was not well-formatted");

                    /* if (result["hmac"].is_empty()) {
                        println!("Missing auth info");
                    } else {

                    } */
                    println!("request body: {:?}", result["hmac"].take());
                    svc.call(req).and_then(|res| Ok(res))
                }),
        )
    }
}