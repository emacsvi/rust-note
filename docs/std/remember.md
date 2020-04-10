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

## 参考文献

- [所有权相关](https://kaisery.gitbooks.io/trpl-zh-cn/content/ch04-01-what-is-ownership.html)


