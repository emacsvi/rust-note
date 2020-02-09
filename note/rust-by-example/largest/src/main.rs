use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T:Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("Hello Largest!");

    // 计算数组中最大值
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // 比较最长字符串是哪一个
    let s1 = String::from("hello jimny.");
    let s2 = String::from("hello pajiro.");
    let result = longest_with_an_announcement(s1.as_str(), s2.as_str(), "start begin.");
    println!("longest_with_an_announcement result:{}", result);
}
