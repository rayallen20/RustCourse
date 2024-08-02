trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    // 调用Dog自身的baby_name关联函数
    let dog_baby_name = Dog::baby_name();
    println!("A baby dog is called a {}", dog_baby_name);

    // 调用Dog实现的Animal trait的baby_name方法
    // 上一个例子中,Trait的方法可以通过传入的具型来判断调用该泛型的哪个实现的方法
    // 但是这里由于方法没有接收者,和关联函数一样,编译器无法判断调用哪个具型上的方法
    // 因此需要指明具型
    let animal_baby_name = <Dog as Animal>::baby_name();
    println!("A baby dog is called a {}", animal_baby_name);
}
