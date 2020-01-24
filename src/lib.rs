pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost{ content: String::new(),}
    }
    pub fn content(&self) -> String {
        self.content.clone()
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(&self) -> PendingReviewPost {
        PendingReviewPost{
            content: self.content.clone(),
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(&self) -> Post {
        Post{
            content: self.content.clone(),
        }
    }
}
