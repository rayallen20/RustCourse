#[route(GET, "/")]
// 在web项目中,通常为handleFunc添加route属性
fn index() {}

#[proc_macro_attribute]
// 而实际上route这个宏就是通过 #[proc_macro_attribute] 来定义的
// 其中 attr参数是属性的内容(在本例中是GET, "/")
// item参数是属性所修饰的函数(在本例中是fn index() {})
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
}
