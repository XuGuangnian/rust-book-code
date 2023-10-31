pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    number_of_approvals: u32,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            number_of_approvals: 0,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content
            .push_str(self.state.as_ref().unwrap().add_text(text));
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve(self))
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    // self: Box<Self>: 只可在持有这个类型的Box上被调用
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn add_text<'a>(&self, text: &'a str) -> &'a str;
}

struct Draft {}

struct PendingReview {}

struct Published {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>, _post: &mut Post) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text<'a>(&self, text: &'a str) -> &'a str {
        text
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State> {
        if post.number_of_approvals > 0 {
            Box::new(Published {})
        } else {
            post.number_of_approvals += 1;
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn add_text<'a>(&self, _text: &'a str) -> &'a str {
        ""
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, _post: &mut Post) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text<'a>(&self, _text: &'a str) -> &'a str {
        ""
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
