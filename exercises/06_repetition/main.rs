////////// DO NOT CHANGE BELOW HERE /////////
fn print_success() {
    println!("Yay, the if statement worked.");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! if_any {
    // 子表达式(出现1次或多次), 每个字表达式用逗号分隔!
    ($($e: expr),+; $block: block) => {
        // 逗号表达式传递给if执行, 如果为true则执行 代码块$block
        if $($e)||+ $block
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    if_any!(false, 0 == 1, true; {
        print_success();
    })
}
