use std::ops::Mul;  // 实现矩形面积运算
use std::convert::{Into};
struct Rect<T, U>   // 为结构体添加两个泛型
{
    width: T,       // 宽和高是不同的泛型
    height: U
}

impl<T, U> Rect<T, U> {   // 为泛型实现方法，impl后也要添加<T>
    fn area(&self) -> T     
    where T: Mul<Output = T> + Copy,     // Mul的第一个参数，表示让这个类型和自身相乘，Output表示输出值的类型
          U: Into<T> + Copy {
        self.width.mul(self.height.into())
    }
}

fn main() {
    let rect2 = Rect{width:3.1, height:4};
    println!("{}", rect2.area());
}

