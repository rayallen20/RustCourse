struct ImportantExcerpt<'a> {
    part: &'a str,
}

// impl<'a> 结构体字段的生命周期参数,需要声明在impl关键字后边
// ImportantExcerpt<'a> 表示这个生命周期应用于ImportantExcerpt结构体 注意: 这里的生命周期参数是ImportantExcerpt类型声明的一部分
impl<'a> ImportantExcerpt<'a> {
    // &self可以不标注生命周期 因为它的生命周期与ImportantExcerpt结构体的生命周期一样
    fn level(&self) -> i32 {
        3
    }

    // 根据规则1: fn announce_and_return_part<'a, 'b>(&'a self, announcement: &'b str) -> &str
    // 该函数不适用规则2
    // 根据规则3: fn announce_and_return_part<'a, 'b>(&'a self, announcement: &'b str) -> &'a str
    // 至此,所有引用的生命周期参数都确定了
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
    println!("ImportantExcerpt: {}", i.part);
}
