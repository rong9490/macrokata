////////// DO NOT CHANGE BELOW HERE /////////
use std::collections::HashMap;

fn print_hashmap(hashmap: &HashMap<&str, &str>) {
    println!("{hashmap:#?}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! hashmap {
    // * 子表达式出现(0次1次或者多次都可以)
    ($($k: literal => $v: expr,)*) => {
        {
            let mut hm = HashMap::new();
            // 遍历执行插入表达式 $( xxx )*
            $( hm.insert($k, $v); )*
            hm
        }
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let value = "my_string";
    let my_hashmap = hashmap!(
        "hash" => "map",
        "Key" => value,
    );
    print_hashmap(&my_hashmap);

    // 空的hashmap
    let my_hashmap2 = hashmap!();
    print_hashmap(&my_hashmap2);
}
