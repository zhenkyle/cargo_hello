use std::fmt;

#[derive(Debug)]
pub struct Post {
    state: Box<dyn State>,
    pub content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Box::new(Draft{}),
            content: String::from(""),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

trait State {}
impl fmt::Debug for dyn State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "State")
    }
}

#[derive(Debug)]
struct Draft {}

impl State for Draft {
}

impl Draft {
}
