fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("some string");
    let mut s2 = s1.clone();
    s2.push_str(" - cloned");
    println!("{}, {}", s1, s2);
}
