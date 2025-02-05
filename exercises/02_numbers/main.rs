////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! num {
    // 这里传入一个标志token用于匹配, 不是rust变量或者宏变量
    (one) => {
        1
    };
    (two) => {
        2
    };
    (three) => {
        3
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    print_result(num!(one) + num!(two) + num!(three));
}
