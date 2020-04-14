## generics
**Rust**实现了泛型，使得使用泛型类型参数的代码相比使用具体类型并没有任何速度上的损失。**Rust**通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

- 当需要在函数体中使用一个参数时，必须在函数签名中声明这个参数以便编译器能知道函数体中这个名称的意义。同理，当在函数签名中使用一个类型参数时，必须在使用它之前就声明它。
- **类型参数声明位于函数名称与参数列表**中间的尖括号`<>`中。`fn largest<T>(list: &[T]) -> T {}`
- 同样也可以使用`<>`语法来定义拥有一个或多个泛型参数类型字段的结构体。必须在结构体名称后面的尖括号中声明泛型参数的名称。接着在结构体定义中可以指定具体数据类型的位置使用泛型类型。
```rust
struct Point<T, U> {
  x: T,
  y: U,
}

fn main() {
// 使用两个泛型的 Point，这样 x 和 y 可能是不同类型,也可以是相同类型
   let integer = Point {x:5, y:1.0};
   let float = Point {x:5.0, y:4.0};
}
```
- 枚举中定义泛型也类型。
```rust
enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}
```
- 方法定义中使用泛型。 注意必须在`impl`后面声明`T`，这样就可以在`Point<T>`上实现的方法中使用它了。在`impl`之后声明泛型`T`，这样`Rust`就知道`Point`的尖括号中的类型是泛型而不是具体类型。
```rust
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
// 在 Point<T> 结构体上实现方法 x，它返回 T 类型的字段 x 的引用
  fn x(&self) -> &T {
    &self.x
  }
}

fn main() {
  let p = Point{x:5, y:1}; println!("p.x={}", p.x());
}
```

- 结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型。<br>
这个例子的目的是展示一些泛型通过`impl`声明而另一些通过方法定义声明的情况。这里泛型参数`T`和`U`声明于`impl`之后，因为他们与结构体定义相对应。而泛型参数`V`和`W`声明于`fn mixup`之后，因为他们只是相对于方法本身的。
```rust
struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

fn main() {
  let p1 = Point {x:5, y: 10.4};
  let p2 = Point {x: "Hello", y: 'c'};
  let p3 = p1.mixup(p2);
  println!("p3.x={}, p3.y={}", p3.x, p3.y);
}
```
