use rocket::http::Status;
use rocket::response::Responder;
use rocket::{Request, Response};
use std::io::Cursor;

// todo thiserror?
#[derive(Debug)]
pub enum Error {
    Net(reqwest::Error),
    Json(serde_json::Error),
    Template(tera::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Net(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Self {
        Self::Template(e)
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
        // todo html
        let body = format!("{:?}", self);
        Response::build()
            .status(Status::InternalServerError)
            .sized_body(Cursor::new(body))
            .ok()
    }
}
