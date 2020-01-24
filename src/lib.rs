pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post{ content: String::from("abc"),}
    }
}
