let sql = sql!("SELECT * FROM `user`");

#[proc_macro]
// 本宏用于解析 SQL 语句
// 这个宏的实现要比macro_rules!复杂得多 因为其中不仅要匹配到Rust代码的语法结构 还要匹配到SQL语句的语法结构
pub fn sql(input: TokenStream) -> TokenStream {

}
