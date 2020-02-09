Rust程序设计语言笔记
<!--more-->

## 介绍
Rust 也为系统编程世界带来了现代化的开发工具：
- **Cargo**，内置的依赖管理器和构建工具，它能轻松增加、编译和管理依赖，并使其在 Rust 生态系统中保持一致。
- **Rustfmt** 确保开发者遵循一致的代码风格。
- **Rust Language Server** 为集成开发环境（IDE）提供了强大的代码补全和内联错误信息功能。
通过使用 Rust 生态系统中的这些和其他工具，开发者可以在编写系统层面代码时保持高生产力。

## 安装

[rust安装](https://www.rust-lang.org/zh-CN/tools/install)
您似乎正在运行 macOS、Linux 或其它类 Unix 系统。要下载 Rustup 并安装 Rust，请在终端中运行以下命令，然后遵循屏幕上的指示。
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
如果你曾经安装过**rustup**，则可以升级**rust**:
```bash
rustup update
```
详细的文档请看：[rustup](https://github.com/rust-lang/rustup/blob/master/README.md)

**常用的一些命令**:
```bash
# path的路径： ~/.cargo/bin
rustc --version
# 升级rust以及rustup自身
rustup update
# working with nightly Rust
# 安装nightly
rustup toolchain install nightly
# 安装之后，但是没有激活，如果查看是否激活
rustup run nigitly rustc --version
# 设置nightly为默认的：
rustup default nightly

rustup install stable
rustup install beta
rustup install nightly

# 版本管理
rustup toolchain default nightly
# 使用一个toolchain而不是默认的 rustup run
rustup run  nightly cargo build
cargo +nightly build # 和上面的一样

# 安装组件
rustup component list
rustup component add rust-src
rustup component rustfmt-preview
```

**cargo使用总结**：

[cargo.toml](https://learnku.com/docs/cargo-book/2018/cargo-toml-vs-cargo-lock/4768)
**Cargo**是**Rust**的构建系统和包管理工具。
主要负责三个工作：
- 构建`build`代码
- 下载你代码 依赖`dependencies` 的 包装箱`crate`
- 编译 你的源码 和 包装箱`crate`。

```bash
# 生成一个项目，--bin表示项目将生成一个可执行文件
cargo new hello --bin

#  cargo check 命令。该命令快速检查代码确保其可以编译，但并不产生可执行文件
cargo check

# cargo build 默认是以 debug 方式编译，在项目 release 时，使用 --release 参数编译最终版本。
cargo build
cargo build --release

# 可以不用build,直接run,它会自动为我们编译并运行程序
cargo run 

cargo test
cargo doc
cargo publish
cargo --version
```

## 猜数字游戏
猜数字游戏注意的地方：
- new是类型String的关联函数(associted function)
- Result是枚举类型enums,有Ok和Err两个值
- io::Result实例拥有一个except方法来处理上面提到的两个值。
- println!()里面的`{}`是点位符
- `use rand::Rng`是一个trait,他定义了随机数的实现方法，想使用这些方法，此trait必须在作用哉之中才可以。
- loop, break, continue, Ordering的使用。
- parse()是如何推导出int是u32还是u64等值的。


```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("we will play game with guest number");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess.");

        // 调用String的类型函数来创建一个可变的guess变量
        let mut guess = String::new();

        // 接收输入，如果报错则panic失败的内容
        io::stdio().read_line(&mut guess)
            .except("Failed to read line.");

        // 这里不用except()来panic,
        // 而是如果报错了，则continue继续下一次
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num, // 成功则返回Ok返回的num
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // 成功则退出游戏
            }
        }

    }
}
```

## 变量和可变性
变量默认是不可改变的(immutable)

**变量与常量的区别**：
- 常量是绑定到一个名称的不允许改变的值。
- 不允许对常量使用`mut`，常量不光默认不能变，它总是不能变。
- 声明常量使用`const`关键字，并且必须注明值的类型。
- 常量可以在任何作用域中声明。
- 常量只能被设置成常量表达式，而不能是函数调用的结果，或任何其他只能在运行时计算出的值。

```rust
let x = 5;
x = 6; // 报错

let mut x = 5;
x = 6; // 正确

// 声明一个常量
const MAX_POINTS: u32 = 100_000;
```

**隐藏(shadowing)**:

- 定义一个同名新变量，隐藏之前的变量
- 重复使用`let`进行多次隐藏

```rust
let x = 5;
let x = x + 1;
let x = x * 2;

let spaces = "     ";
let spaces = spaces.len();


let mut spaces = "    ";
spaces = spaces.len; // 则会报编译错误，类型不匹配，必须加let重新声明一个变量
```

## 数据类型
常用类型

**Booleans**类型: `bool`类型的值有`true`或者`false`

**char类型**代表一个`Unicode`标量值，所以是**4个字节**

**数值类型**有：i8,i16,i32,i64,u8,u16,u32,u64,isize,usize,f32,f64

**数组**是相同类型且固定大小的顺序对象。 [参考数组](https://www.twle.cn/c/yufei/rust/rust-basic-array.html)

**切片** 指的是对另一个数据结构的索引(或“视图”)。他们是用于允许安全，高效的访问数组的一部分而不需复制数组的内容。例如，您可能只是想索引文件中某一行并将其读入内存中。从本质上说，切片不是直接创建的，而是来自于现有的变量。切片拥有长度，是可变的也可以设置不可变，并且在许多方面像数组是相似的

```rust
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

    // 默认是float64
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x:{}, y:{}", x, y);

    // bool 类型
    let t = true;
    let f: bool = false;
    if t != f {
        println!("not equal.");
    }

    // char 字符类型
    // 单引号指定，并且大小为四个字节(four bytes)
    // 四个字节代码Unicode标量值，比ASCII表示更多内容
    let c = 'z';
    let z = 'Z';
    println!("c:{}, z:{}", c, z);

    // 元组长度固定，并且一旦声明，长度不能改变
    // 元组类型可以不相同，数组类型必须一样。
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x:{}, y:{}, z:{}", x, y, z);
    // 也可以这样访问
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("x:{}, y:{}, z:{}", x, y, z);

    // 数组是固定长度，分配在栈上，每个元素类型必须相同
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
    let a = ["January", "Feb", "March", "April", "May"];
    println!("{}", a[0]);
    // i32 是每个元素的类型。分号之后，数字 5 表明该数组包含五个元素
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
    let a = [3; 5];
    println!("{}", a[0]);
    // let index = 5;
    // println!("{}", a[index]); // 会报错，运行时错误
    println!("{}", a.len()) //获取数据个数

    // 切处是对另外一个数据结构的索引或者视图
    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4]; // A slice of a: just the elements 1, 2, and 3
    let complete = &a[..]; // A slice containing all of the elements in a
    println!("a={:?}", a);
    println!("m={:?}", middle);
    println!("complete={:?}", complete);
    // 默认值是-1的方式定义数组
    let a: [i32; 4] = [-1; 4];
    println!("a={:?}", a);

    // 遍历数组
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    println!("a={:?}", a);
    println!("a.len={}", a.len());

    for index in 0..a.len() {
        println!("index is: {} & value is: {}", index, a[index]);
    }

    for val in a.iter() {
        println!("value is:{}", val);
    }

    // 如果想修改数组的元素，需要定义可变数组
    let mut a: [i32; 4] = [10, 10, 30, 40];
    a[1] = 20;
    println!("a={:?}", a);

    // 数组做为函数参数
    // 数组可以作为函数的参数。而传递方式有 传值传递 和 引用传递 两种方式。
    // 传值传递 就是传递数组的一个副本给函数做参数，函数对副本的任何修改都不会影响到原来的数组。
    // 引用传递 就是传递数组在内存上的位置给函数做参数，因此函数对数组的任何修改都会影响到原来的数组。
    let a = [10, 20, 30];
    update(a);
    println!("Inside main {:?}", a);

    let mut a = [10, 20, 30];
    delete(&mut a);
    println!("delete main {:?}", a);
}

fn update(mut a: [i32; 3]) {
    for i in 0..a.len() {
        a[i] = 0;
    }
    println!("Inside update {:?}", a);
}

fn delete(a: &mut [i32; 3]) {
    for i in 0..a.len() {
        a[i] = 0
    }

    println!("delete {:?}", a);
}
```

## 所有权
首先，让我们看一下所有权的规则。当我们通过举例说明时，请谨记这些规则：

1. Rust 中的每一个值都有一个被称为其**所有者**(owner）的变量。
2. 值有且只有一个所有者。
3. 当所有者（变量）离开作用域，这个值将被丢弃。

**引用**：`&s1` 则创建了一个引用.同时函数的定义中，我们获取`&String`

`&`符号就是引用。它们允许你使用值但不获取其所有权。

**借用**: 我们将获取引用作为函数参数称为借用.o

**可变引用**: 上面代码改为`mut s`。然后必须创建一个可变引用`&mut s`和接受一个可变引用`some_string: &mut String`。

不过可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用。

### 引用的规则:
让我们概括一下之前对引用的讨论：

- 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
- 引用必须总是有效的。

```rust
fn main() {
    copy_trait();
    takes();
    mut_apply();
    mut_reference();
}

fn mut_apply() {
    let s = String::from("hello");
    mut_change(s);
}

fn mut_change(mut x: String) {
    x.push('o');
    println!("mut_change result: {}", x);
}

fn copy_trait() {
    let s = "hello world!";
    println!("s = {}", s);
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("s = {}", s);

    // 都放入到栈中
    let x = 5;
    let y = x;
    println!("x = {}", x);
    println!("y = {}", y);

    let s1 = String::from("hello");
    let s2 = s1;
    // s1离开作用域就已经失效了
    // println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone(); // 复制堆上所有数据
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn mut_reference() {
    // 这样会报错, 因为m必须是可变的才可以
    // let m = String::from("hello");
    let mut m = String::from("hello");
    m.push_str("dd");
    println!("m={}", m);

    let mut s = String::from("hello");
    mut_reference_change(&mut s);
    println!("mut_reference mut s = {}", s);
}

fn mut_reference_change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

















## 参考链接

- [Rust程序设计语言](https://kaisery.github.io/trpl-zh-cn/ch06-02-match.html)
- [rust doc](https://doc.rust-lang.org/std/index.html)

