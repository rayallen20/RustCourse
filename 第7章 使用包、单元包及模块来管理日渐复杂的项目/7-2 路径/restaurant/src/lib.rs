mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径(使用crate关键字)
    // 函数eat_at_restaurant()和模块front_of_house都在lib.rs中
    // lib.rs是library crate的根文件 所以函数eat_at_restaurant()和模块front_of_house隐式地组成了根模块(crate root)
    // 所以这里的crate关键字 指的就是crate root 也就是函数eat_at_restaurant()和模块front_of_house的位置
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    // 函数eat_at_restaurant()和模块front_of_house在crate中处于同级
    // 因此直接从模块名(front_of_house)开始就可以了
    front_of_house::hosting::add_to_waitlist();
}
