fn main() {
    let x = 2;
    let y = x;

    println!("x: {} y: {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;
    // let s3 = s1; // use of moved value: `s1`

    // println!("{}", s1); // borrow of moved value: `s1`

    let mut str1 = String::from("hello");
    let str2 = &str1; // 不可变引用

    // str2.push_str(" world"); // 不可变引用不能修改可变引用，就是只读，没有写的权利
    let str3 = &mut str1;
    let str4 = &mut str1;
    println!("{}", str4);
    // 使用已经 borrow 的变量就会提示重复borrow
    println!("{}", str3);

    str4.push_str(" world");
    println!("{}", str4);
}
