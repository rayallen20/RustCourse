extern crate proc_macro;

// proc_macro包提供了编译器接口 其中的TokenStream类型用于表示Rust代码
use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

// #[proc_macro_derive(HelloMacro)] 宏用于定义一个自定义的派生宏
// 简单理解就是 当你在一个类型上标注 #[derive(XXX)] 属性时
// Rust编译器会自动查找并调用 #[proc_macro_derive(XXX)] 指定的函数
#[proc_macro_derive(HelloMacro)]
/// 本函数将在类型添加了 #[derive(HelloMacro)] 属性时,被自动调用
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 将输入的 TokenStream(可以理解为就是输入的Rust代码) 转换成AST
    let ast = syn::parse(input).unwrap();

    // 为输入的代码提供HelloMacro trait的实现
    impl_hello_macro(&ast)
}

/// 本函数用于为输入的代码提供HelloMacro trait的实现
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // 获取添加了 #[derive(HelloMacro)] 属性的类型名称
    let name = &ast.ident;

    // 为该类型实现 HelloMacro trait并提供实现
    // quote! 宏: 用于将宏内部的Rust代码转换成TokenStream
    let gen = quote! {
        // 此处的 #name 是一个插值表达式,用于将name变量的值插入到代码中
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify! 宏: 用于将传入的表达式转换成字符串
                // 例如: stringify!(1 + 2) 将返回 "1 + 2"
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };

    // quote! 宏返回的TokenStream是一种编译器无法直接理解的数据结构,
    // 因此需要将其转换成编译器可以理解的TokenStream
    gen.into()
}