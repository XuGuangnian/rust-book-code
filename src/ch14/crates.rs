use rust_book_code::{mix, PrimaryColor};

pub(crate) fn pub_use() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
