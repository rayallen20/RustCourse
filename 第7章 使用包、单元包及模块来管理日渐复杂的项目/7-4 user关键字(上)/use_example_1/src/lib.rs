mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 这里使用use的效果类似文件系统中的软链接
// 引入之后 hosting就可以在当前作用域(crate root)内直接使用了
// 相当于模块hosting是在crate root下定义的
use crate::front_of_house::hosting;

// 其实等价于如下代码:

// mod front_of_house { pub fn add_to_waitlist() {} }

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}