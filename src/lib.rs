#[derive(Debug)]
pub struct Post {
    state: String,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: String::from("abc"),
            content: String::from("abc"),
        }
    }
}
