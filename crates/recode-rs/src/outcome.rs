use crate::Meta;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Outcome {
    pub src: Meta,
    pub dest: Meta,
    pub data: Vec<u8>,
}

impl Outcome {
    pub fn new(src: Meta, dest: Meta, data: Vec<u8>) -> Self {
        Outcome { src, dest, data }
    }
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Recoder ( src: {}, dest: {}, data: {}b )",
            self.src,
            self.dest,
            self.data.len()
        )
    }
}
