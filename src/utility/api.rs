/*
    the following code is referenced from a post on stackoverflow
    post link : https://stackoverflow.com/a/54867136/13026811
    modifications were made to the original code to match requirements
*/

use rocket::http::hyper::header;
// use hyper::items::{Authorization, Bearer, Headers};
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use serde::Serialize;
use std::marker::PhantomData;

#[derive(Debug, Serialize)]
struct ApiResponseData<D: Serialize, E: Serialize> {
    data: Option<D>,
    error: Option<E>,
}

impl<D: Serialize, E: Serialize> ApiResponseData<D, E> {
    fn success_data(data: D) -> ApiResponseData<D, E> {
        ApiResponseData {
            data: Some(data),
            error: None,
        }
    }

    fn error_data(error: E) -> ApiResponseData<D, E> {
        ApiResponseData {
            data: None,
            error: Some(error),
        }
    }
}

#[derive(Debug)]
pub struct ApiResponse<D: Serialize, E: Serialize> {
    json: Value,
    status: Status,
    response: PhantomData<ApiResponseData<D, E>>,
}

impl<D: Serialize, E: Serialize> ApiResponse<D, E> {
    pub fn success_data(data: D, status: Status) -> ApiResponse<D, E> {
        let res: ApiResponseData<D, E> = ApiResponseData::success_data(data);
        ApiResponse {
            json: json!(res),
            status,
            response: PhantomData,
        }
    }

    pub fn error_data(error: E, status: Status) -> ApiResponse<D, E> {
        let res: ApiResponseData<D, E> = ApiResponseData::error_data(error);
        ApiResponse {
            json: json!(res),
            status,
            response: PhantomData,
        }
    }
}

impl<'r, D: Serialize, E: Serialize> Responder<'r, 'r> for ApiResponse<D, E> {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .header(header::Authorization(header::Bearer {
                token: token.to_owned(),
            }))
            .ok()
    }
}
