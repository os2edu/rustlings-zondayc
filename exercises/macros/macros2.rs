// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

/*
* 宏定义需要在使用之前定义
*/

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
