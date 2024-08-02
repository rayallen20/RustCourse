mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 此时外部的代码是无法访问hosting模块的,因为仅使用use关键字
// 导入的模块在本作用域内是私有的
// use front_of_house::front_of_house;

// 如果想要让外部的代码也能访问到hosting模块,就要使用pub use
// 相当于外部代码认为hosting模块是在当前作用域下定义的
// 这样的方式使得代码实际存在的位置和外部代码看到的位置不一致
// 进而导致代码的可读性下降,因为外部代码无法直观的看到hosting模块是在哪个作用域下定义的
pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
