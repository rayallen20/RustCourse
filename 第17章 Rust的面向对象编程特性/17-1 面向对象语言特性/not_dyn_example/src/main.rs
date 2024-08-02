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

fn make_animal_talk<T: Animal>(animals: Vec<T>) {
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

    let animals: Vec<Animal> = vec![dog, cat];  // error: doesn't have a size known at compile-time
    make_animal_talk(animals);
}
