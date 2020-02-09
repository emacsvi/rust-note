rust 的if let简单控制流。
<!--more-->

## 概念

`if let`语法让我们以一种不那么冗长的方式结合`if`和`let`，来处理只匹配一个模式的值而忽略其他模式的情况。考虑示例 6-6 中的程序，它匹配一个`Option<u8>`值并只希望当值为`3`时执行代码：
```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```
示例 6-6：`match`只关心当值为`Some(3)`时执行代码

我们想要对`Some(3)`匹配进行操作但是不想处理任何其他`Some<u8>`值或`None`值。为了满足`match`表达式（穷尽性）的要求，必须在处理完这唯一的成员后加上`_ => ()`，这样也要增加很多样板代码。

不过我们可以使用`if let`这种更短的方式编写。如下代码与示例 6-6 中的 match 行为一致：
```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```
`if let`获取通过等号分隔的一个模式和一个表达式。它的工作方式与 match 相同，这里的表达式对应 match 而模式则对应第一个分支。

使用`if let`意味着编写更少代码，更少的缩进和更少的样板代码。然而，这样会失去 match 强制要求的穷尽性检查。match 和 if let 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍。

换句话说，可以认为`if let`是`match`的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。

可以在 if let 中包含一个 else。else 块中的代码与 match 表达式中的 _ 分支块中的代码相同，这样的 match 表达式就等同于 if let 和 else。回忆一下示例 6-4 中 Coin 枚举的定义，其 Quarter 成员也包含一个 UsState 值。如果想要计数所有不是 25 美分的硬币的同时也报告 25 美分硬币所属的州，可以使用这样一个 match 表达式：

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```
或者可以使用这样的 if let 和 else 表达式：
```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```
如果你的程序遇到一个使用 match 表达起来过于啰嗦的逻辑，记住 if let 也在你的 Rust 工具箱中。

## if let 和 while let
If let
If let 允许你把 if 和 let 结合到一起，来减少某些类型的模式匹配所需的开销。

例如，有某种 Option<T>。如果它是 Some<T>，我们希望在它上面调用一个函数，如果不是，则什么也不做。就像下面这样：

match option {
Some(x) => { foo(x) },
None => {},
}
在这里我们不一定非要使用匹配,例如,我们可以使用 if

if option.is_some() {
let x = option.unwrap();
foo(x);
}
这些选项都不是特别有吸引力。我们可以用 if let 语句以更好的方式做同样的事情：

if let Some(x) = option {
foo(x);
}
如果一个模式匹配成功，它将给模式的标识符绑定任意合适的值，然后评估表达式。如果模式不匹配，则什么也不去做。

当模式不匹配时，如果你希望去做别的事情,您可以使用 else：

if let Some(x) = option {
foo(x);
} else {
bar();
}
while let
以类似的方式，当一个值匹配某种模式时，你可以用 while let 来进行条件循环。代码如下面所示：

loop {
match option {
Some(x) => println!("{}", x),
_ => break,
}
}
转换成下面这样的代码:

while let Some(x) = option {
println!("{}", x);
}


## 参考文献

- [if let](https://kaisery.github.io/trpl-zh-cn/ch06-03-if-let.html)
- [if let and while let](https://wiki.jikexueyuan.com/project/rust/if-let.html)
- []()


