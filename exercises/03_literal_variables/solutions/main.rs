////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! math {
    // 美元$表示宏变量, 类型是 literal字面量
    ($first:literal plus $second:literal) => {
        $first + $second
    };
    (square $first:literal) => {
        $first * $first
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    print_result(math!(3 plus 5));
    print_result(math!(square 2));
}
