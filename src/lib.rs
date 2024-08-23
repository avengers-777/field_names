extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, FieldsNamed};

/*
这是一个多行注释的例子。
可以包含多行文本。
适用于更长的注释，解释复杂的代码或算法。
9913 6296 2886 2097
4689 0477 6299 9985
3035 9854 2625 4321
5793 1500 6517 7275
4198 8839 3975 2214

*/

#[proc_macro_derive(FieldNames)]
pub fn field_names_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident; // 获取结构体名称

    let fields = match input.data {
        Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => named,
            _ => panic!("FieldNames only supports structs with named fields"),
        },
        _ => panic!("FieldNames only supports structs"),
    };

    let consts = fields.iter().map(|f| {
        f.ident
            .as_ref()
            .map(|ident| {
                let const_name = format_ident!("f_{}", ident.to_string());
                quote! {
                    pub const #const_name: &'static str = stringify!(#ident);
                }
            })
            .unwrap() // 使用 unwrap 确保 ident 存在
    });

    let expanded = quote! {
        impl #struct_name {
            #(#consts)*
        }
    };

    TokenStream::from(expanded)
}
