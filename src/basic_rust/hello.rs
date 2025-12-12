//! Rust 基础语法
//! 
//! Rust 中注释有 3 种方式：
//!   1. 顶部这种方式 `//!` 一般用于模块级别文档
//!   2. 普通注释，使用 `//` 或者 `/*..*/`，这类注释用于解释代码逻辑，不参与文档生成。
//!   3. 文档注释，使用 `///` 或者 `/**..*/`，这类注释可以生成 HTML 文档
//! 文档注释内可以使用 markdown 语法
fn main() {
    println!("Hello, Rust!");
    vairable_define();
    basic_types();
    array_usage();
    string_usage();
    basic_control();
}


/// Rust变量定义
fn vairable_define() {
    println!("-------- Rust变量定义 --------");
    // Rust使用 let 关键字定义变量，变量类型放在后面
    let n1: i32 = 10;
    // Rust会自动推导变量类型
    let n2 = 20;
    println!("n1: {}, n2: {}", n1, n2);
    // let 定义的变量不可变不允许变量重新赋值，下面编译时会报错
    // n1 = 30;

    // 使用 mut 关键字定义可变变量
    let mut n3 = 30;
    println!("n3: {}", n3);
    n3 = 40;
    println!("changed n3: {}", n3);

    /*
    let 和 let mut 的区别在于“变量绑定”是否可变，而不是“值”本身是否可变。
    核心在于 Rust 区分了：
      - 变量的可变性：能否修改变量绑定的值
      - 类型的可变性：变量的类型是否提供了修改变量内容的方法
    Rust 默认变量不可变，是为了防止意外修改。
    即使一个类型支持修改，也必须使用 mut 显式声明你打算修改它。
     */
}


/// Rust基本类型
fn basic_types() {
    println!("-------- Rust基本类型 --------");
    // i32 有符号整型，类似还有 i8, i16, i64, usize
    let n1: i32 = 10;
    println!("i32: {}", n1);
    // u32 无符号整型，类似还有 u8, u16, u64, usize
    let n2: u32 = 20;
    println!("u32: {}", n2);
    // f32 单精度浮点型，f64 双精度浮点型
    let n3: f32 = 30.0;
    println!("f32: {}", n3);
    let n4: f64 = 40.0;
    println!("f64: {}", n4);
    // bool 布尔型
    let b1: bool = true;
    println!("bool: {}", b1);

    // 字符型
    let c1: char = 'a';
    println!("char: {}", c1);
    // 字符串字面量，类型是 &str，它是不可变的
    let s1: &str= "hello rust string";
    println!("literal string: {}", s1);
    // 字符串 String 类型，它是可变的
    let s2: String = String::from("hello rust string");
    println!("string: {}", s2);
    // 使用 mut 定义后，才能修改字符串
    let mut s3 = String::from("hello rust string");
    println!("string: {}", s3);
    s3.push_str(" world");
    println!("string: {}", s3);

    // 元组
    let t1: (i32, &str) = (10, "hello");
    println!("tuple: {:?}", t1);
    println!("tuple.0: {}", t1.0);
    // 元组下标只能是字面量，不能使用变量
    // let i = 1;
    // println!("tuple.1: {}", t1.i)

    // 数组，类型 [T; N] 表示一个 N 个值的数组，每个值的类型都是 T。
    // 数组的大小 N 是在编译时确定的常量，也是类型自身的一部分。
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array: {:?}", a1);
    println!("array[0]: {}", a1[0]);

    // 向量，类型 Vec<T> 表示一个动态大小数组，每个值的类型都是 T。
    // 使用 vec! 宏创建向量，注意，这里使用了 mut 关键字
    let mut v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    v1.push(6);
    println!("vector: {:?}", v1);
    println!("vector[0]: {}", v1[0]);
    // 调用 vec! 宏等价于调用 Vec::new 创建一个新的空向量，然后再向其中推入元素
    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    println!("vector2: {:?}", v2);
}

/// Rust数组和向量使用
fn array_usage(){
    println!("-------- Rust数组/向量 --------");
    // TODO
}

/// Rust字符串使用
fn string_usage(){
    println!("-------- Rust字符串 --------");
    // TODO
}


/// Rust控制结构
fn basic_control() {
    println!("-------- Rust控制结构 --------");
    // if 表达式，支持多重链式
    // 需要注意的是，if 表达式的所有块都必须产生相同类型的值
    println!(">>> if");
    let n1 = 10;
    if n1 > 5 {
        println!("n2 > 5");
    } else if n1 < 5 {
        println!("n2 < 5");
    } else {
        println!("n2 == 5");
    }

    // match 表达式
    println!(">>> match");
    let n2 = 10;
    match n2 {
        1 => println!("n2 == 1"),
        2 => println!("n2 == 2"),
        3 => println!("n2 == 3"),
        _ => println!("n2 != 1, 2, 3"),
    }

    // if let 表达式：if let 表达式只是对只有一个模式的 match 的简写
    println!(">>> if let");
    let n3 = 10;
    if let 1 = n3 {
        println!("n3 == 1");
    } else if let 2 = n3 {
        println!("n3 == 2");
    } else if let 3 = n3 {
        println!("n3 == 3");
    } else {
        println!("n3 != 1, 2, 3");
    }

    // for 循环
    println!(">>> for");
    for i in 0..10 {
        println!("i: {}", i);
    }

    // while 循环
    println!(">>> while");
    let mut n4 = 0;
    while n4 < 10 {
        println!("n4: {}", n4);
        n4 += 1;
    }

    // loop 循环
    println!(">>> loop");
    let mut n5 = 0;
    loop {
        println!("n5: {}", n5);
        n5 += 1;
        if n5 == 10 {
            break;
        }
    }

}