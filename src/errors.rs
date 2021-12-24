use rocket::http::Status;
use rocket::response::Responder;
use rocket::{Request, Response};
use std::io::Cursor;

// todo thiserror?
#[derive(Debug)]
pub enum Error {
    Net(reqwest::Error),
    Template(tera::Error),
    Json((serde_json::Error, String)), // error + json
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Net(e)
    }
}

impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Self {
        Self::Template(e)
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
        let body = match self {
            Error::Json((error, json)) => format!("{:?}\n\n{}", error, json),
            v => format!("{:?}", v),
        };
        Response::build()
            .status(Status::InternalServerError)
            .sized_body(Cursor::new(body))
            .ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;

    #[test]
    fn from1() {
        // fn get_error() -> Result<(), Error> {}
        assert_eq!(2 + 2, 4);
    }
}
