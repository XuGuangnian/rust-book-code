use blog::Post;
use rust_book_code::Post as RustPost;

pub fn state_pattern() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    post.add_text("==");
    assert_eq!("", post.content());

    post.approve();
    post.approve();
    post.add_text("==");
    assert_eq!("I ate a salad for lunch today", post.content());
}

pub fn state_pattern_rust() {
    let mut post = RustPost::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
