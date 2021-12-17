#[derive(Debug)]
struct Dog {
    name: String,
    weight: f32,
}

impl Dog {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn bark(&self) {
        println!("{} barks", self.name);
    }

    // 不传 &self 类似静态方法
    fn turn_around() {
        println!("take around");
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Dog"),
        weight: 70.0,
    };

    println!("{:#?}", dog);
    println!("{}", dog.get_name());
    println!("{}", dog.get_weight());

    dog.bark();
    // dog.turn_around(); // this is an associated function, not a method
    Dog::turn_around();

    let mut v: Vec<String> = Vec::new();
    v.push(String::from("1"));
    v.push(String::from("2"));
    v.push(String::from("3"));

    for i in &v {
        println!("{}", i);
    }

    println!("{:?}", v);
}
