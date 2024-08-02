// 本trait用于定义摘要行为
// 注意: 本trait是一个公共trait 这意味着在lib.rs(library crate的根模块)中定义的该Trait可以被其他crate使用
pub trait Summary {
    // 本方法用于生成一个摘要内容
    fn summarize(&self) -> String;
}