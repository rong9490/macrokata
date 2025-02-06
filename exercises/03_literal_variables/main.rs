////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! math {
    // literal 字面量
    // plus 是标志 token
    ($a: literal plus $b: literal) => {
        $a + $b
    };
    (square $a: literal) => {
        $a * $a
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    print_result(math!(3 plus 5));
    print_result(math!(square 2));
}
