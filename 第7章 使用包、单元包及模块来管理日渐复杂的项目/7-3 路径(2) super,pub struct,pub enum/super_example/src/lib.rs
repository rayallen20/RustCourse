fn server_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // server_order()位于根模块(crate root)中
        super::server_order();

        // 使用绝对路径的等价形式
        crate::server_order();
    }

    fn cook_order() {}
}