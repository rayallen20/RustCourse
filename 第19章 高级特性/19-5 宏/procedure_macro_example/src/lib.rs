use proc_macro::TokenStream;

// #[some_attribute]: 用于指定过程宏类型的占位符
// TokenStream: 用于表示输入和输出的 token 序列 可以简单理解为该类型表示过程宏输入和输出的Rust代码
#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
    input
}