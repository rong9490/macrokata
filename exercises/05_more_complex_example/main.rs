////////// DO NOT CHANGE BELOW HERE /////////
// This function should be called by the `show_output!()` macro
#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn show(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! for_2d {
    // 这里宏参数比较复杂, 注意梳理这5个宏参数及其类型 (参数类型 与 宏参数类型)
    // ident 是宏变量类型 ; <$x_type:ty> 是普通变量类型
    // block 类型是代码块类型
    ($x_name: ident <$x_type:ty> in $x_expr: expr, $y_name: ident <$y_type:ty> in $y_expr: expr, $block: block) => {
        // 上面的传参只是宏的传参
        for $x_name in $x_expr {
            let $x_name: $x_type = $x_name; // 注意这里必须"重新定义"一遍 x变量才能在代码中使用
            for $y_name in $y_expr {
                let $y_name: $y_type = $y_name;
                $block // block代码块原样放最后执行!!
            }
        }
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    for_2d!(row <i32> in 3..5, col <i32> in 8..10, {
        let coordinate: Coordinate = Coordinate {x: col, y: row};
        coordinate.show()
    });

    // 注意类型转换 u16.into::<i32>(): u16
    let values: [u16; 3] = [1, 3, 5];
    for_2d!(x <u16> in values, y <u16> in values, {
        (Coordinate {x: x.into(), y: y.into()}).show()
    });
}
