use proc_macro2::{Delimiter, Group, TokenStream};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, Result, Token, Type};

#[proc_macro]
pub fn impl_with_ptr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ImplTemplate { name, args, result, body } = parse_macro_input!(input as ImplTemplate);

    let fn_type_offset = format_ident!("{}_type_offset", name);
    let fn_byte_offset = format_ident!("{}_byte_offset", name);
    let fn_offset = format_ident!("{}_offset", name);

    let ptr_type_offset = format_ident!("get_ptr_type_offset");
    let ptr_byte_offset = format_ident!("get_ptr_byte_offset");
    let ptr_offset = format_ident!("get_ptr_offset");

    let args_tpl: Vec<_> = args.iter()
        .map(|arg| {
            let ArgPair { name, ty } = arg;
            quote!( #name: #ty )
        })
        .collect();

    let output = quote! {
        impl MemBuffer {
            pub fn #name<T>(&self #(, #args_tpl )*) -> Result<#result, MemErr> {
                match self.get_ptr::<T>() {
                    Ok(ptr) => Ok(#body),
                    Err(e) => Err(e),
                }
            }
            pub fn #fn_offset<T>(&self, offset: MemOffset #(, #args_tpl )*) -> Result<#result, MemErr> {
                match self.#ptr_offset::<T>(offset) {
                    Ok(ptr) => Ok(#body),
                    Err(e) => Err(e),
                }
            }
            pub fn #fn_type_offset<T>(&self, type_offset: usize #(, #args_tpl )*) -> Result<#result, MemErr> {
                match self.#ptr_type_offset::<T>(type_offset) {
                    Ok(ptr) => Ok(#body),
                    Err(e) => Err(e),
                }
            }
            pub fn #fn_byte_offset<T>(&self, byte_offset: usize #(, #args_tpl )*) -> Result<#result, MemErr> {
                match self.#ptr_byte_offset::<T>(byte_offset) {
                    Ok(ptr) => Ok(#body),
                    Err(e) => Err(e),
                }
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

struct ArgPair {
    name: Ident,
    ty: Type,
}

struct ImplTemplate {
    name: Ident,
    args: Vec<ArgPair>,
    result: Type,
    body: Group,
}

impl Parse for ImplTemplate {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;

        let mut args = Vec::<ArgPair>::new();

        // 形参列表：开始
        input.parse::<Token![|]>()?;
        while let Some(name) = input.parse::<Option<Ident>>()? {
            input.parse::<Token![:]>()?;

            let ty = input.parse::<Type>()?;
            input.parse::<Option<Token![,]>>()?;

            args.push(ArgPair { name, ty });
        }
        input.parse::<Token![|]>()?;
        // 形参列表：结束

        input.parse::<Token![->]>()?;

        let result = input.parse::<Type>()?;
        input.parse::<Token![,]>()?;

        let body = Group::new(Delimiter::Brace, input.parse::<TokenStream>()?);

        Ok(Self { name, args, result, body })
    }
}