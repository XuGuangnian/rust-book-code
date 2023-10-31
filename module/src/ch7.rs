pub mod modules {
    use module::{eat_at_restaurant, hosting};

    use crate::ch7_garden::vegetables::Asparagus;

    pub fn module() {
        // 声明模块 garden_inline, ch7_garden, ch7_garden_mod
        // 声明子模块 in ch7_garden mod (src/ch7_garden.rs)
        // 模块中的代码路径
        let plant = crate::ch7_garden::vegetables::Asparagus {};
        println!("I'm growing {:?}!", plant);
    }

    pub fn module_path() {
        // 模块路径：绝对路径 相对路径
        eat_at_restaurant();
    }

    pub(crate) fn pub_use() {
        // 私有 vs 公用 pub
        // use 关键字
        let plant = Asparagus {};
        println!("I'm growing {:?}!", plant);

        // use as 重命名
        // use std::io::Result as IoResult;

        // pub use 重导出
        hosting::add_to_waitlist();
    }
}
