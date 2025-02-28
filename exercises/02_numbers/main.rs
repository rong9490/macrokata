////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// 一个宏内部允许多个匹配的分支! 第一项匹配成功的
macro_rules! num {
    // 这里传入一个标志token用于匹配, 不是rust变量或者宏变量
    // one, two, three 是宏标识符, 标志token
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
    // one, two, three 是标志token, 不是rust变量或者宏变量
    // 特别需要纠正过来, 这不是变量, 不需要提前声明
    print_result(num!(one) + num!(two) + num!(three));
}
