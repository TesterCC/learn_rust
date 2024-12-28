// fn main() {
//     println!("Hello, world! Rust!");   // println!是宏，去掉!才是函数
// }

// Rust权威指南 第2章：猜数游戏，生成一个1到100之间的随机整数，并让玩家对数字进行猜测，用户猜对则打印信息
// ref: https://e.dangdang.com/pc/reader/index.html?id=1901221748
// rust会将预导入 prelude 模块内的条目自动引入每一段程序的作用域中。如果你需要的类型不在预导入模块内，就必须使用use语句来显示地进行声明。
use std::io;   // 把标准库std的io模块引入当前的作用域中
use std::cmp::Ordering;  // 枚举类型，拥有Less、Greater、Equal这3个变体
use rand::Rng;  // Rng是一个trait（特征），定义了随机数生成器需要实现的方法集合。为了使用这些方法，需要显示引入到当前作用域。

fn main() {
    println!("Guess the number!");

    // add random number
    // rand::thread_rng 返回一个特定的随机数生成器：它位于本地线程空间，并通过操作系统获得随机数种子。
    let secret_number = rand::thread_rng().gen_range(1, 101);  //范围左闭右开，含下不含上
    // println!("The secret number is {}", secret_number);  // for debug, default i32

    print!("Please input your guess.");


    let mut guess = String::new();   // mut 修饰的变量具有可变性； 一个新的String实例，内部使用UTF-8格式编码； String::new 中的 :: 语法表明new是String类型的一个关联函数（在某些语言中称为静态方法）
    // rust的引用和变量默认情况下是不可变的，所以要用&mut guess来声明一个可变引用
    io::stdin().read_line(&mut guess).expect("Failed to read line");  // read_line 用于获取用户输入；&mut中&表示点给钱的参数是一个引用，代码可通过引用在不同的地方访问同一份数据，而无需付出多余的拷贝开销


    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    // todo: compare secret number and guess number  P62 2-4 最后
    // cmp方法为可比较的值类型计算出它们比较后的结果。
    // 这里cmp方法接收了被比较值 secret_number 的引用作为参数来与 guess进行比较
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}