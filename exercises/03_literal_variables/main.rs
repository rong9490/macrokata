////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// 宏的匹配规则, 从上到下匹配, 第一项匹配成功的分支生效
macro_rules! math {
    // $开头的是宏变量, 其类型是 literal 字面量类型!
    // plus 是标志 token, 用于站位分隔定位
    ($a: literal plus $b: literal) => {
        // 占位变量 $a 捕获 3, $b 捕获 5
        // 处理表达式
        $a + $b
    };
    (square $a: literal) => {
        $a * $a
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    print_result(math!(3 plus 5)); // 展开为: 3 + 5
    print_result(math!(square 2)); // 展开为: 2 * 2
}
