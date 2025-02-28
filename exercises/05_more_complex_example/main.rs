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

// 复杂的宏, 对宏参数进行逐步拆解分析
macro_rules! for_2d {
    // 宏参数: 标识符<宏变量类型>
    // $x_name: ident <$x_type:ty> 冒号ty表示是type类型 "这是宏指示符"
    // 常见的宏指示符: ty, ident, expr, block, literal, meta, path, lifetime, tt
    ($x_name: ident <$x_type:ty> in $x_expr: expr, $y_name: ident <$y_type:ty> in $y_expr: expr, $block: block) => {
        // 上面的传参只是宏的传参, 不是代码块类型
        for $x_name in $x_expr {
            let $x_name: $x_type = $x_name; // 注意这里必须"重新定义"一遍 x变量才能在代码中使用, 区别宏声明与代码块声明!!
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
