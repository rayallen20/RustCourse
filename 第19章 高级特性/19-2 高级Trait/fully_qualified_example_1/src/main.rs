/// 飞行员Trait
trait Pilot {
    fn fly(&self);
}

/// 巫师Trait
trait Wizard {
    fn fly(&self);
}

struct Human;

// Human实现Pilot
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

// Human实现Wizard
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// Human实现自己的fly方法
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;

    // 调用Human自己的fly方法
    person.fly();

    // 调用Human实现了的Pilot的fly方法
    // 这里传&person是因为Pilot的fly方法的接收者是&self
    Pilot::fly(&person);

    // 调用Human实现了的Wizard的fly方法
    Wizard::fly(&person);
}
