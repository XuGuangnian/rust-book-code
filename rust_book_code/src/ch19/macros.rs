#[macro_export]
macro_rules! vec_def {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub fn macro_rules() {
    let v: Vec<u32> = vec![1, 2, 3];
    let v_def: Vec<u32> = vec_def![1, 2, 3];
    println!("{:?}", v);
    println!("{:?}", v_def);
    assert_eq!(v, v_def);
}

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
pub fn proc_macro_custom_derive() {
    #[derive(HelloMacro)]
    struct Pancakes;

    Pancakes::hello_macro();
}
pub fn proc_macro_attribute_like() {
    // #[route(GET, "/")]
    // fn index() {}

    // #[proc_macro_attribute]
    // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
}
pub fn proc_macro_function_like() {
    // let sql = sql!(SELECT * FROM posts WHERE id=1);

    // #[proc_macro]
    // pub fn sql(input: TokenStream) -> TokenStream {}
}
