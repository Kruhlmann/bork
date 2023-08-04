#[derive(Clone)]
pub struct Dialog {
    pub contents: String,
    pub age_milliseconds: i32,
}

impl Dialog {
    pub fn new(contents: String) -> Self {
        Self {
            contents,
            age_milliseconds: 0,
        }
    }
}
