struct ImportantExcerpt<'a> {
    // 此处的'a是一个生命周期标注
    // 表示ImportantExcerpt的实例不能比其part字段的引用存在的更久
    // 或者换言之 part字段的引用要比ImportantExcerpt的实例存在的更久 且 要求part字段的引用的生命周期能够完全覆盖ImportantExcerpt的实例的生命周期
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,       // // 此处part字段的生命周期与first_sentence的生命周期相同 也就是第10行到第14行的生命周期 而ImportantExcerpt实例的生命周期是第11行到第14行
    };
    println!("{}", i.part);
}
