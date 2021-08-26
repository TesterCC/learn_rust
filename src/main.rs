// fn main() {
//     println!("Hello, world! Rust!");   // println!是宏，去掉!才是函数
// }

// Rust权威指南 第2章：猜数游戏，生成一个1到100之间的随机整数，并让玩家对数字进行猜测，用户猜对则打印信息
// ref: https://e.dangdang.com/pc/reader/index.html?id=1901221748
// rust会将预导入 prelude 模块内的条目自动引入每一段程序的作用域中。如果你需要的类型不在预导入模块内，就必须使用use语句来显示地进行声明。
use std::io;  // 把标准库std的io模块引入当前的作用域中

fn main() {
    println!("Guess the number!");
    print!("Please input your guess.");
    let mut guess = String::new();   // mut 修饰的变量具有可变性； 一个新的String实例，内部使用UTF-8格式编码； String::new 中的 :: 语法表明new是String类型的一个关联函数（在某些语言中称为静态方法）
    io::stdin().read_line(&mut guess).expect("Failed to read line");  // read_line 用于获取用户输入；&mut中&表示点给钱的参数是一个引用，代码可通过引用在不同的地方访问同一份数据，而无需付出多余的拷贝开销
    // rust的引用和变量默认情况下是不可变的，所以要用&mut guess来声明一个可变引用
    println!("You guessed: {}", guess);
    // todo 生成一个随机数 P61
}