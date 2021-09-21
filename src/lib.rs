use proc_macro::TokenStream;
use quote::quote;
use syn::{
    fold::Fold,
    parse::{Parse, ParseStream, Result},
    parse_macro_input, FnArg, Ident, ItemFn, ItemImpl, ReturnType, Token, Type,
};

struct Args {
    wrapper: Ident,
    ty: Type,
}

impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let wrapper: Ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        let ty: Type = input.parse()?;

        Ok(Self { wrapper, ty })
    }
}

struct Rewriter {
    wrapper: Ident,
    ty: Type,
    async_wrapper: bool,
}

impl Fold for Rewriter {
    // TODO
    // fn fold_item_impl(&mut self, ast: ItemImpl) -> ItemImpl {
    //     ast
    // }

    fn fold_item_fn(&mut self, mut ast: ItemFn) -> ItemFn {
        if ast.sig.abi.is_some() || ast.sig.variadic.is_some() {
            panic!("#[wrap] can not be used on FFI functions");
        }

        let outer_await = if ast.sig.asyncness.is_some() {
            quote! { .await }
        } else {
            quote! {}
        };

        let wrapper_await = if self.async_wrapper {
            quote! { .await }
        } else {
            quote! {}
        };

        let mut new = ast.clone();
        let wrapper = &self.wrapper;

        ast.sig.ident = proc_macro2::Ident::new("__inner", proc_macro2::Span::call_site());
        ast.sig.inputs.clear();
        let args = ast.sig.inputs.iter().map(|i| match i {
            FnArg::Receiver(_) => quote! { self },
            FnArg::Typed(t) => {
                let p = &t.pat;
                quote! { #p }
            }
        });

        let ts = proc_macro::TokenStream::from(quote! {
            {
                #ast

                #wrapper(__inner(#(#args),*)#outer_await)#wrapper_await
            }
        });

        new.block = Box::new(syn::parse(ts).unwrap());
        new.sig.output = match new.sig.output {
            ReturnType::Default => ReturnType::Default,
            ReturnType::Type(arr, _) => ReturnType::Type(arr, Box::new(self.ty.clone())),
        };

        new
    }
}

fn wrap_fn_inner(args: TokenStream, input: TokenStream, async_wrapper: bool) -> TokenStream {
    let ast = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as Args);

    let rewritten = Rewriter {
        wrapper: args.wrapper.clone(),
        ty: args.ty.clone(),
        async_wrapper,
    }
    .fold_item_fn(ast);

    TokenStream::from(quote! {
        #rewritten
    })
}

/// Wrap one function with a normal function
#[proc_macro_attribute]
pub fn wrap_fn(args: TokenStream, input: TokenStream) -> TokenStream {
    wrap_fn_inner(args, input, false)
}

/// Wrap one function with an async function
#[proc_macro_attribute]
pub fn wrap_fn_async(args: TokenStream, input: TokenStream) -> TokenStream {
    wrap_fn_inner(args, input, true)
}

/// Wrap all functions in an impl block with a given function
#[proc_macro_attribute]
pub fn wrap_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemImpl);
    let args = parse_macro_input!(args as Args);

    let rewritten = Rewriter {
        wrapper: args.wrapper.clone(),
        ty: args.ty.clone(),
        async_wrapper: false,
    }
    .fold_item_impl(ast);

    TokenStream::from(quote! {
        #rewritten
    })
}
