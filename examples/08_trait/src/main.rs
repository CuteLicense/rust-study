use std::fmt::Debug;

// pub trait NAME
trait Info {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> i32;
    fn print_self(&self) where Self: Debug {
        println!("{:#?}", self);
    }
}

#[derive(Debug)]
struct Student {
    name: String,
    age: i32,
}

// impl TraitName for StrcutName
impl Info for Student {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> i32 {
        self.age
    }
}

#[derive(Debug)]
struct Teacher {
    name: String,
    age: i32,
    subject: String,
}

impl Info for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> i32 {
        self.age
    }
}

impl Teacher {
    fn get_subject(&self) -> &String {
        &self.subject
    }
}

// 方式一：trait 直接申明参数类型
fn trait_func(item: &impl Info){
    println!("item's name: {}", item.get_name());
}

// 方式二：使用泛型，trait bound 的写法
fn trait_func_without_ref<T: Info>(item: T){
    println!("item's name: {}", item.get_name());
}
// 方式三：使用 where 约束泛型
fn trait_func_without_ref_where<T>(item: T) where T: Info {
    println!("item's name: {}", item.get_name());
}

trait GetName {
    fn get_a_name(&self) -> &String;
}

trait GetAge {
    fn get_an_age(&self) -> i32;
}

impl GetName for Teacher {
    fn get_a_name(&self) -> &String {
        &self.name
    }
}

impl GetName for Student {
    fn get_a_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for Teacher {
    fn get_an_age(&self) -> i32 {
        self.age
    }
}

impl GetAge for Student {
    fn get_an_age(&self) -> i32 {
        self.age
    }
}

struct PeopleMatched<T, U> {
    master: T,
    student: U,
}

impl<T: GetName + GetAge, U: GetName + GetAge> PeopleMatched<T, U> {
    fn print_info(&self){
        println!("name of master is {}", self.master.get_a_name());
        println!("age of student is {}", self.student.get_an_age());
    }
}

fn main() {
    let s = Student {
        name: String::from("Andy"),
        age: 12
    };
    println!("name of student is {}, {} years old.", s.get_name(), s.get_age());

    let t = Teacher {
        name: "Echo".to_string(),
        age: 32,
        subject: String::from("programing"),
    };

    println!("name of teacher is {}, {} years old, teaching {}", t.get_name(), t.get_age(), t.get_subject());
    
    trait_func(&t);
    println!("{:?}", t);
    t.print_self();
    
    // s 所有权转移 Move 
    // trait_func_without_ref(s);
    // s 已被drop,不能继续使用
    // println!("{:?}", s);
    
    let p = PeopleMatched {
        master: t,
        student: s
    };
    p.print_info();
}
