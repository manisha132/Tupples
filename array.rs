fn main() {
    let s = "/test1/test2/test3";
    let pos = s.rfind('/');

    println!("{:?}", pos); // prints "Some(12)"
}
