# About

最近因为项目需要，在恶补C and C++，学得累了就写写Rust的Hello World聊以慰藉（其实我是因为rust的非官方吉祥物很可爱才入坑的），估计大概率学着学着就偏离C/C++了。

自娱自乐的自学项目，平时还是用Python和Go的时候比较多。

更新随缘，佛系学习^_^


# Rust Roadmap

学习路线指导

1. 入门

《The Rust Programming Language》，中文出版物是《Rust权威指南》。

强烈推荐配合 [B站 令狐一冲](https://www.bilibili.com/video/BV1xJ411B79h) 的讲解视频。（从入门到进阶到网络编程到异步， 而且也参考Rust死灵书做了视频）

万一上面的组合你觉得不合适，B站也有其它up的Rust入门视频，选一个适合你的学就对了。

要是实在觉得不习惯，还可以通过 [https://tourofrust.com/](https://tourofrust.com/06_zh-cn.html) 来学习下rust的基本语法。

2. 进阶

国内已出版：张老师的《Rust编程之道》。（个人觉得时间有限的2选1，前者极客时间也有配套学习视频）

刷题：rustlings 94

必读资料：

- 《Rust By Example》，这本书可以当精简手册来查询，提供了很多的Example。

- std - Rust 、《The Reference》、《Rust Compiler Error Index 》：标准库、语法手册、编译错误示例，这三大件几乎是日常必备的资料需要时常翻阅和查询，

3. 提高

- 《The Rustonomicon》又翻译为《Rust死灵书》，但是一直没写有写完。但要能建立起对Rust的深入理解，这本书必不可少。

- [B站 令狐一冲的视频](https://www.bilibili.com/video/BV1xp4y1a78D)

- [Writing an OS in Rust](https://os.phil-opp.com/) 这一系列博客是你最好的学习材料。

# Rust Wiki

- crate 是 Rust中最小的编译单元，package 是 单个或多个crate的集合。
- Cargo使用TOML作为标准配置格式，例如：Cargo.toml。

# Rust Command

## Normal

```bash
rustup update   // udpate rust version
rustup self uninstall   // delete rustup and rust toolchains
rustup doc     // 打开帮助文档 推荐 Rust by Example

rustc --version     // get rust version

```

## Cargo
```bash

cargo check    // 构建项目 跳过生成可执行文件的步骤
cargo build    // 构建项目 并生成文件
cargo run      // 构建并运行项目

// 构建产生的结果会被cargo存储再target/debug目录下

// 生成的可执行文件在 target/release目录下
cargo build --release  

cargo run --release

// 在本地构建一份有关项目所有依赖的文档 并自动地在浏览器中将文档打开来供用户查阅
cargo doc --open 
```

## Comments

- `/src_bk/`仅备份了一个简单端口扫描的代码

