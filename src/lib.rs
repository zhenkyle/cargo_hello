#[derive(Debug)]
pub struct Post {
    state: Box<dyn State>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: ???,
            content: String::from("abc"),
        }
    }
}

trait State {}

struct Draft {}

impl trait State for Draft {
}
