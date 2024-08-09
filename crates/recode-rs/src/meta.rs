use crate::Format;

#[derive(Clone, Copy, Debug)]
pub struct Meta {
    pub format: Format,
    pub width: u32,
    pub height: u32,
}

impl Meta {
    pub fn new(format: Format, width: u32, height: u32) -> Self {
        Meta {
            format,
            width,
            height,
        }
    }
}

impl std::fmt::Display for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Meta ( fmt: {}, w: {}, h: {} )",
            self.format.to_string(),
            self.width,
            self.height
        )
    }
}
