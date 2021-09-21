use std::{
    collections::HashMap,
    fmt::{self, Debug, Display, Formatter},
};
use wrap_rs::wrap_fn;

trait Fallable {
    type Output;
    type Error: Into<AnnoyingError>;

    fn annoying(&self) -> Result<Self::Output, Self::Error>;
}

#[derive(Debug)]
struct AnnoyingError {
    message: String,
    meta: Option<HashMap<String, String>>,
}

impl<T: Display> From<T> for AnnoyingError {
    fn from(t: T) -> Self {
        Self {
            message: t.to_string(),
            meta: None,
        }
    }
}

#[derive(Debug)]
enum MyError {
    ClientError(String, u16),
    ServerError(String),
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::ClientError(s, code) => write!(f, "[{}] CLIENT ERROR: {}", code, s),
            Self::ServerError(s) => write!(f, "[500] SERVER ERROR: {}", s),
        }
    }
}

impl MyError {
    fn as_annoying_error(&self) -> AnnoyingError {
        let (message, status_code) = match self {
            Self::ClientError(s, code) => (s.clone(), *code),
            Self::ServerError(s) => (s.clone(), 500),
        };

        let mut meta = HashMap::new();
        meta.insert("statusCode".into(), status_code.to_string());

        AnnoyingError {
            message,
            meta: Some(meta),
        }
    }
}

struct Foo;

impl Fallable for Foo {
    type Output = String;
    type Error = MyError;

    fn annoying(&self) -> Result<String, MyError> {
        Err(MyError::ClientError(
            "need to keep the status code".into(),
            404,
        ))
    }
}

struct Bar;

impl Fallable for Bar {
    type Output = String;
    type Error = AnnoyingError;

    #[wrap_fn(convert_error => Result<String, AnnoyingError>)]
    fn annoying(&self) -> Result<String, MyError> {
        Err(MyError::ClientError(
            "need to keep the status code".into(),
            404,
        ))
    }
}

fn convert_error<T>(res: Result<T, MyError>) -> Result<T, AnnoyingError> {
    res.map_err(|e| e.as_annoying_error())
}

fn call_in_annoying_way<F, T, E>(f: F)
where
    F: Fallable<Output = T, Error = E>,
    T: Debug,
    E: Debug + Into<AnnoyingError>,
{
    let res = f.annoying().map_err(|e| e.into());
    println!("{:?}", res);
}

fn main() {
    call_in_annoying_way(Foo);
    call_in_annoying_way(Bar);
}
