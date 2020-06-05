use failure::Fail;

#[derive(Fail, Debug)]
pub enum FetchError {
    #[fail(display = "hyper::Error")]
    Http(hyper::Error),

    #[fail(display = "std::io::Error")]
    Io(std::io::Error),

    #[fail(display = "String")]
    Msg(String),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<std::io::Error> for FetchError {
    fn from(err: std::io::Error) -> FetchError {
        FetchError::Io(err)
    }
}

impl From<String> for FetchError {
    fn from(err: String) -> FetchError {
        FetchError::Msg(err)
    }
}