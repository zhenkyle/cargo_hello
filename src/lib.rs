pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost{ content: String::new(),}
    }
}

pub struct DraftPost {
    content: String,
}
