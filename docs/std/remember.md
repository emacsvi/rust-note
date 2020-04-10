rust需要牢记的笔记
<!--more-->

## 概念
rust里面需要牢记的东西。

## ownship
**rust**没有`garbage collector`, 利用了**ownship**来管理**heap**上面的数据。所以有如下规则需要牢记：

> 突然想起我中学的杜老师，那时候强行要求我记很多公式，最后无形中发现它们产生了很大的作用。

- Rust中每一个值都有一个被称为其所有者owner的变量。
  > Each value in Rust has a variable that’s called its owner.
- 值有且只有一个所有者。
  > There can only be one owner at a time.
- 当所有者（变量）离开作用域，这个值将被丢弃。
  > When the owner goes out of scope, the value will be dropped.
- 在特定作用域中的特定数据有且只有一个可变引用。
  > you can have only one mutable reference to a particular piece of data in a particular scope. 
- 我们 也 不能在拥有不可变引用的同时拥有可变引用。不可变引用的用户可不希望在他们的眼皮底下值就被意外的改变了！然而，多个不可变引用是可以的，因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。
- 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
- 引用必须总是有效的。

**悬垂引用错误**
```rust
fn main() {
  let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle 返回一个字符串的引用
  let s = String::from("hello"); // s 是一个新字符串
  &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
 // 危险！
```

修改：将所有权转移出去，所以没有值被释放了：
```rust
fn dangle() -> String {
  let s = String::from("hello");
  s
}
```

## 参考文献

- [所有权相关](https://kaisery.gitbooks.io/trpl-zh-cn/content/ch04-01-what-is-ownership.html)


