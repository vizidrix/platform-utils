use ring::digest;

static SHA1: &str = "SHA1";
static SHA256: &str = "SHA256";
static SHA384: &str = "SHA384";
static SHA512: &str = "SHA512";
static SHA512_256: &str = "SHA512_256";

#[derive(Copy, Clone, Debug)]
pub enum Algorithm {
    SHA1,
    SHA256,
    SHA384,
    SHA512,
    SHA512_256,
}

impl From<&digest::Algorithm> for Algorithm {
    fn from(src: &digest::Algorithm) -> Self {
        if *src == digest::SHA256 {
            Algorithm::SHA256
        } else if *src == digest::SHA512 {
            Algorithm::SHA512
        } else if *src == digest::SHA384 {
            Algorithm::SHA384
        } else if *src == digest::SHA512_256 {
            Algorithm::SHA512_256
        } else {
            Algorithm::SHA1
        }
    }
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Algorithm::SHA1 => SHA1,
                Algorithm::SHA256 => SHA256,
                Algorithm::SHA384 => SHA384,
                Algorithm::SHA512 => SHA512,
                Algorithm::SHA512_256 => SHA512_256,
            }
        )
    }
}
