macro_rules! digit {
    // 10个分支匹配模式
    (zero) => {
        "0"
    };
    (one) => {
        "1"
    };
    (two) => {
        "2"
    };
    (three) => {
        "3"
    };
    (four) => {
        "4"
    };
    (five) => {
        "5"
    };
    (six) => {
        "6"
    };
    (seven) => {
        "7"
    };
    (eight) => {
        "8"
    };
    (nine) => {
        "9"
    };
}

////////// DO NOT CHANGE ABOVE HERE /////////

// HACK 一个宏可以嵌套调用其他宏: 把token转换为对应的数字, 再连接起来
macro_rules! number {
    ($($num: ident)+) => (concat!(
        // 重复执行子表达式
        $(digit!($num)),+)
    )
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let my_number: u32 = number!(nine three seven two zero).parse::<u32>().unwrap();
    let my_other_number: u32 = number!(one two four six eight zero).parse::<u32>().unwrap();
    println!("{}", my_number + my_other_number); // = 218400
}
