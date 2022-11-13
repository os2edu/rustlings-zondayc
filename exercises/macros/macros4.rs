// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

/*
*宏编程不同的情况之间需要用分号';'间隔开
*/

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:literal) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!("7777");
}
