////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! math {
    // expr标量, 不能使用 literal
    ($e: expr, plus, $v: expr) => {
        $e + $v
    };
    (square $a: expr) => {
        $a * $a
    };
}

macro_rules! broken_macro {
    // 需要有个逗号, 否则无法解析
    ($a:expr, please) => { $a }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let var = 5;
    print_result(math!((2 * 3), plus, var));
    print_result(math!(square var));
    print_result(broken_macro!(1, please));
}
