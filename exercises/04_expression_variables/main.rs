////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! math {
    // expr表达式, 不能使用 literal字面量
    ($a: expr, plus, $b: expr) => {
        $a + $b
    };
    (square $a: expr) => {
        $a * $a
    };
}

macro_rules! broken_macro {
    // 需要有个逗号, 否则表达式无法解析 'a + b please'
    ($a:expr, please) => { $a }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let var = 5;
    print_result(math!((2 * 3), plus, var));
    print_result(math!(square var));
    print_result(broken_macro!(1, please));
}
