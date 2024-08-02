fn main() {
    {
        // 变量r此时只是被声明了,没有被初始化
        // 这么做的目的是为了让r存在于x的作用域之外
        // Rust中,不允许空值存在.但是r此时还没有被初始化,因此r并不是一个空值
        let r;

        // 如果此时使用了r 则会报错
        // let b = r; // 这里会报错,因为r没有被初始化 (error[E0381]: borrow of possibly-uninitialized variable: `r`)

        {
            let x = 5;
            r = &x; // error[E0597]: `x` does not live long enough
        }

        // 在此处使用r时会报错 因为r指向的值(也就是x的引用)已经被释放了
        // 因为x已经离开了自己的作用域 所以x已经被释放了 进而x的引用也就失效了
        // 所以此时r指向的是一个已经被释放的内存 也就是悬垂引用
        println!("r: {}", r);
    }
}
