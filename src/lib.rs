use std::fmt;

#[derive(Debug)]
pub struct Post {
    state: Box<dyn State>,
    content: String,
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

    pub fn content(&self) -> &str {
        self.state.content(self)
    }
}

trait State {
    fn content<'a>(&self, post: &'a Post) -> &'a str;
}

impl fmt::Debug for dyn State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "State")
    }
}

#[derive(Debug)]
struct Draft {
}

impl State for Draft {
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

#[derive(Debug)]
struct PendingReview {
}

impl State for PendingReview {
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

#[derive(Debug)]
struct Published {
}

impl State for Published {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        & post.content
    }
}
