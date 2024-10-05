use crate::Format;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Outcome {
    pub src: Format,
    pub dest: Format,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Outcome {
    pub fn new(src: Format, dest: Format, width: u32, height: u32, data: Vec<u8>) -> Self {
        Outcome { src, dest, width, height, data }
    }
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Recoder ( src: {:?}, dest: {:?}, w: {}, h: {}, data: {}b )",
            self.src,
            self.dest,
            self.width,
            self.height,
            self.data.len()
        )
    }
}
