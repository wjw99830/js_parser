pub struct ReadonlyString {
    string: String,
    pub length: usize,
}

impl ReadonlyString {
    pub fn new(str: &str) -> Self {
        ReadonlyString {
            string: str.to_string(),
            length: utf8_slice::len(str),
        }
    }

    pub fn slice(&self, begin: usize, end: usize) -> &str {
        utf8_slice::slice(&self.string, begin, end)
    }
}
