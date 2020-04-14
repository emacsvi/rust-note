## trait
`trait`告诉**Rust**编译器某个特定类型拥有可能与其他类型共享的功能。可以通过`trait`以一种抽象的方式定义共享的行为。可以使用`trait bounds`指定泛型是任何拥有特定行为的类型。

```rust
pub trait Summary {
  // 在方法签名后跟分号
  fn summarize(&self) -> String;
}

// 为NewsArticle实现Summary trait
impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

// 为Tweet实现Summary trait
impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}
```

- 在类型上实现 trait 类似于实现与 trait 无关的方法。区别在于 impl 关键字之后，我们提供需要实现 trait 的名称，接着是 for 和需要实现 trait 的类型的名称。
- 在 impl 块中，使用 trait 定义中的方法签名，不过不再后跟分号，而是需要在大括号中编写函数体来为特定类型实现 trait 方法所拥有的行为。
- 实现 trait 时需要注意的一个限制是，**只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait**。
  > 例如，可以为 aggregator crate 的自定义类型 Tweet 实现如标准库中的 Display trait，这是因为 Tweet 类型位于 aggregator crate 本地的作用域中。类似地，也可以在 aggregator crate 中为 Vec<T> 实现 Summary，这是因为 Summary trait 位于 aggregator crate 本地作用域中。
  > 但是不能为外部类型实现外部 trait。例如，不能在 aggregator crate 中为 Vec<T> 实现 Display trait。这是因为 Display 和 Vec<T> 都定义于标准库中，它们并不位于 aggregator crate 本地作用域中。
- 这个限制是被称为 相干性（coherence） 的程序属性的一部分，或者更具体的说是 孤儿规则（orphan rule），其得名于不存在父类型。这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现。


## trait默认实现
一旦定义了 summarize_author，我们就可以对 Tweet 结构体的实例调用 summarize 了，而 summary 的默认实现会调用我们提供的 summarize_author 定义。因为实现了 summarize_author，Summary trait 就提供了 summarize 方法的功能，且无需编写更多的代码。

默认实现可以用`impl Summary for NewsArticle {}` 指定一个空的impl块。
```rust
pub trait Summary {
  fn summarize_author(&self) -> String;
  // 默认实现
  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}

impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}
```

## trait作为参数
```rust
pub fn notify(item: impl Summary) {
  println!("Breaking news! {}", item.summarize());
}
```

## trait bound
```rust
pub fn notify(item: impl Summary) {}
// 像这样：
pub fn notify<T: Summary>(item: T) {}

pub fn notify(item1: impl Summary, item2: impl Summary){}
pub fn notify<T: Summary>(item1: T, item2: T){}

// + 号指定多个trait bound
pub fn notify(item: impl Summary + Display) {}
pub fn notify<T: Summary + Display>(item: T) {}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

// 也可以用where 从句：
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
          {}
```

## 返回实现了trait的类型
