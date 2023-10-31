mod ch7;

mod ch7_garden_inline {}

// src/ch7_garden.rs
mod ch7_garden;
// src/ch7_garden_mod/ch7_garden_mod
mod ch7_garden_mod;

fn main() {
    ch7::modules::module();
    ch7::modules::module_path();
    ch7::modules::pub_use();
}
