trait Animal {
    fn name(&self) -> String;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

// Sized特性表示类型的大小在编译时是已知的
// ?Sized表示类型的大小在编译时是未知的
// 这里的T是指Animal的实现类型,所以T是大小未知的,因此要使用?Sized
fn make_animal_talk<T: Animal + ?Sized>(animals: Vec<Box<T>>) {
    for animal in animals {
        animal.talk();
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Dog"),
    };
    let cat = Cat {
        name: String::from("Cat"),
    };

    // 使用dyn关键字进行动态分发
    // 注意: 这里直接写Vec<dyn Animal>会报错,因为dyn Trait是一个动态大小类型,需要使用Box来包装
    // 因为Rust要求所有的类型在编译时都需要知道其大小,而dyn Trait是一个动态大小类型,所以需要使用Box来包装
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];
    make_animal_talk(animals);
}
