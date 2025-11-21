use std::fmt;

#[derive(Debug, Clone)]
pub struct OmmaErr {
    body: String,
}

impl fmt::Display for OmmaErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OmmaErr: {}", self.body)
    }
}

impl OmmaErr {
    pub fn new(body: &str) -> Self {
        OmmaErr {
            body: body.to_string(),
        }
    }
}

impl From<std::io::Error> for OmmaErr {
    fn from(e: std::io::Error) -> Self {
        OmmaErr {
            body: e.to_string(),
        }
    }
}
