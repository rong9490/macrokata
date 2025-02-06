////////// DO NOT CHANGE BELOW HERE /////////
fn print_vec<V: std::fmt::Debug>(vec: &Vec<V>) {
    println!("{vec:#?}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! graph {
    // 最外层表示子表达式次数0次1次或多次
    // 内层表达式, 构造出一个元组的模式, 用于匹配(逗号分隔)
    ($($from: literal -> ($($to: literal),*);)*) => {
        {
            let mut vec = Vec::new();

            // 重复执行子表达式
            $( $(vec.push(($from, $to));)* )*

            vec
        }
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

#[allow(clippy::vec_init_then_push)]
fn main() {
    let my_graph = graph!(
        1 -> (2, 3, 4, 5);
        2 -> (1, 3);
        3 -> (2);
        4 -> ();
        5 -> (1, 2, 3);
    );

    print_vec(&my_graph);
}
