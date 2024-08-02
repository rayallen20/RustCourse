struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    /// `drop()`方法通常用于释放资源
    /// 此处出于演示的目的,我们只是打印了一句话
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    // 创建变量时是先创建的c再创建的d
    // 而在main函数结束时,变量的释放顺序是先释放d再释放c
}
