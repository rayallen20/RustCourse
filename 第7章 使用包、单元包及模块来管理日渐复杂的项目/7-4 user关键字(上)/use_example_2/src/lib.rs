mod front_of_house {
    pub mod hosting {
        // 此处处于hosting模块中
        // hosting的父模块是front_of_house
        // front_of_house的父模块是crate root
        use super::super::eat_at_restaurant;

        pub fn add_to_waitlist() {}

        fn use_super() {
            eat_at_restaurant();
        }
    }
}

// 此处处于root crate 所以想要调用root crate的子模块中的内容
// 直接指定路径即可
use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
