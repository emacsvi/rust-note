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
`If let`允许你把`if`和`let`结合到一起，来减少某些类型的模式匹配所需的开销。

例如，有某种`Option<T>`。如果它是`Some<T>`，我们希望在它上面调用一个函数，如果不是，则什么也不做。就像下面这样：
```rust
match option {
  Some(x) => { foo(x) },
  None => {},
}
```
在这里我们不一定非要使用匹配,例如,我们可以使用`if`
```rust
if option.is_some() {
  let x = option.unwrap();
  foo(x);
}
```
这些选项都不是特别有吸引力。我们可以用`if let`语句以更好的方式做同样的事情：
```rust
if let Some(x) = option {
  foo(x);
}
```
如果一个模式匹配成功，它将给模式的标识符绑定任意合适的值，然后评估表达式。如果模式不匹配，则什么也不去做。

当模式不匹配时，如果你希望去做别的事情,您可以使用`else`：
```rust
if let Some(x) = option {
  foo(x);
} else {
  bar();
}
```

**while let**:

以类似的方式，当一个值匹配某种模式时，你可以用`while let`来进行条件循环。代码如下面所示：
```rust
loop {
  match option {
    Some(x) => println!("{}", x),
    _ => break,
  }
}
```
转换成下面这样的代码:
```rust
while let Some(x) = option {
  println!("{}", x);
}
```

## 更多的例子

在一些场合下，用`match`匹配枚举类型并不优雅。比如：
```rust
// 将 `optional` 定为 `Option<i32>` 类型
let optional = Some(7);

match optional {
    Some(i) => {
        println!("This is a really long string and `{:?}`", i);
        // ^ 行首需要 2 层缩进。这里从 optional 中解构出 `i`。
        // 译注：正确的缩进是好的，但并不是 “不缩进就不能运行” 这个意思。
    },
    _ => {},
    // ^ 必须有，因为 `match` 需要覆盖全部情况。不觉得这行很多余吗？
};
```

`if let`在这样的场合要简洁得多，并且允许指明数种失败情形下的选项：
```rust
fn main() {
    // 全部都是 `Option<i32>` 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构读作：若 `let` 将 `number` 解构成 `Some(i)`，则执行
    // 语句块（`{}`）
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // 如果要指明失败情形，就使用 else：
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // 解构失败。切换到失败情形。
        println!("Didn't match a number. Let's go with a letter!");
    };

    // 提供另一种失败情况下的条件。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // 解构失败。使用 `else if` 来判断是否满足上面提供的条件。
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 条件的值为 false。于是以下是默认的分支：
        println!("I don't like letters. Let's go with an emoticon :)!");
    };
}
```

同样，可以用`if let`匹配任何枚举值：
```rust
// 以这个 enum 类型为例
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    // 创建变量
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    
    // 变量 a 匹配到了 Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    
    // 变量 b 没有匹配到 Foo::Bar，因此什么也不会打印。
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    
    // 变量 c 匹配到了 Foo::Qux，它带有一个值，就和上面例子中的 Some() 类似。
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}
```


另一个好处是：`if let`允许匹配枚举非参数化的变量，即枚举未注明`#[derive(PartialEq)]`，我们也没有为其实现`PartialEq`。在这种情况下，通常`if Foo::Bar==a`会出错，因为此类枚举的实例不具有可比性。但是，`if let`是可行的。

你想挑战一下吗？使用 if let修复以下示例：
```rust
// 该枚举故意未注明 `#[derive(PartialEq)]`，
// 并且也没为其实现 `PartialEq`。这就是为什么下面比较 `Foo::Bar==a` 会失败的原因。
enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    // 变量匹配 Foo::Bar
    if Foo::Bar == a {
    // ^-- 这就是编译时发现的错误。使用 `if let` 来替换它。
        println!("a is foobar");
    }

    // 修复代码如下：
    if let Foo::Bar == a {
      println!("a is foobar");
    }
}
```

## 参考文献

- [if let](https://kaisery.github.io/trpl-zh-cn/ch06-03-if-let.html)
- [if let and while let](https://wiki.jikexueyuan.com/project/rust/if-let.html)
- [更多的例子](https://rustwiki.org/zh-CN/rust-by-example/flow_control/if_let.html)


