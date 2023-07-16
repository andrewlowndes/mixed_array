use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

const ENUM_NAME_PREFIX: &str = "Mixed";

struct Context<'a> {
    name: &'a Ident,
    generics: &'a TokenStream,
    chars: &'a [Ident],
}

fn gen_borrow(char: &Ident) -> TokenStream {
    let func_name = format_ident!("borrow_{}", char.to_string().to_lowercase());

    quote!(
        pub fn #func_name(&self) -> Option<&#char> {
            match self {
                Self::#char(item) => Some(item),
                _ => None,
            }
        }
    )
}

fn gen_borrow_mut(char: &Ident) -> TokenStream {
    let func_name = format_ident!("borrow_{}_mut", char.to_string().to_lowercase());

    quote!(
        pub fn #func_name(&mut self) -> Option<&mut #char> {
            match self {
                Self::#char(item) => Some(item),
                _ => None,
            }
        }
    )
}

fn gen_take(char: &Ident) -> TokenStream {
    let func_name = format_ident!("take_{}", char.to_string().to_lowercase());

    quote!(
        pub fn #func_name(self) -> Option<#char> {
            match self {
                Self::#char(item) => Some(item),
                _ => None,
            }
        }
    )
}

fn gen_copy(
    Context {
        chars,
        generics,
        name,
        ..
    }: &Context,
) -> TokenStream {
    let where_copy = chars.iter().map(|char| quote!(#char: Copy));

    quote!(
        impl #generics Copy for #name #generics where #(#where_copy),* {}
    )
}

fn gen_clone(
    Context {
        chars,
        generics,
        name,
        ..
    }: &Context,
) -> TokenStream {
    let where_clone = chars.iter().map(|char| quote!(#char: Clone));

    let clone_variants = chars
        .iter()
        .map(|char| quote!(Self::#char(val) => Self::#char(val.clone())));

    quote!(
        impl #generics Clone for #name #generics where #(#where_clone),* {
            fn clone(&self) -> Self {
                match self {
                    #(#clone_variants),*
                }
            }
        }
    )
}

fn gen_debug(
    Context {
        chars,
        generics,
        name,
        ..
    }: &Context,
) -> TokenStream {
    let where_debug = chars.iter().map(|char| quote!(#char: std::fmt::Debug));

    let debug_variants = chars.iter().map(|char| {
        let quoted = format!("{char}");
        quote!(Self::#char(val) => f.debug_tuple(#quoted).field(val).finish())
    });

    quote!(
        impl #generics std::fmt::Debug for #name #generics where #(#where_debug),* {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                match self {
                    #(#debug_variants),*
                }
            }
        }
    )
}

fn gen_eq(
    Context {
        chars,
        generics,
        name,
        ..
    }: &Context,
) -> TokenStream {
    let partialeq_all = chars
        .iter()
        .map(|char| quote!(PartialEq<#char>))
        .collect::<Vec<_>>();

    let where_partialeq = chars.iter().map(|char| quote!(#char: #(#partialeq_all)+*));

    let where_eq = chars
        .iter()
        .map(|char| quote!(#char: Eq + #(#partialeq_all)+*));

    let partialeq_other_variant = chars
        .iter()
        .map(|char| quote!(Self::#char(right) => left.eq(right)))
        .collect::<Vec<_>>();

    let partialeq_variants = chars.iter().map(|char| {
        quote!(
            Self::#char(left) => {
                match other {
                    #(#partialeq_other_variant),*
                }
            }
        )
    });

    quote!(
        impl #generics PartialEq<Self> for #name #generics where #(#where_partialeq),* {
            fn eq(&self, other: &Self) -> bool {
                match self {
                    #(#partialeq_variants),*
                }
            }
        }

        impl #generics Eq for #name #generics where #(#where_eq),* {}
    )
}

fn gen_ord(
    Context {
        chars,
        generics,
        name,
        ..
    }: &Context,
) -> TokenStream {
    let partialord_all = chars
        .iter()
        .map(|char| quote!(PartialOrd<#char>))
        .collect::<Vec<_>>();

    let where_partialord = chars.iter().map(|char| quote!(#char: #(#partialord_all)+*));

    let where_ord = chars
        .iter()
        .map(|char| quote!(#char: Ord + #(#partialord_all)+*));

    let partialord_other_variant = chars
        .iter()
        .map(|char| quote!(Self::#char(right) => left.partial_cmp(right)))
        .collect::<Vec<_>>();

    let partialord_variants = chars.iter().map(|char| {
        quote!(
            Self::#char(left) => {
                match other {
                    #(#partialord_other_variant),*
                }
            }
        )
    });

    quote!(
        impl #generics PartialOrd<Self> for #name #generics where #(#where_partialord),* {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                match self {
                    #(#partialord_variants),*
                }
            }
        }

        impl #generics Ord for #name #generics where #(#where_ord),* {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
            }
        }
    )
}

fn gen_hash(
    Context {
        chars,
        generics,
        name,
        ..
    }: &Context,
) -> TokenStream {
    let where_hash = chars
        .iter()
        .map(|generic| quote!(#generic: std::hash::Hash));

    let hash_variants = chars
        .iter()
        .map(|generic| quote!(Self::#generic(val) => val.hash(state)));

    quote!(
        impl #generics std::hash::Hash for #name #generics where #(#where_hash),* {
            fn hash<Z: std::hash::Hasher>(&self, state: &mut Z) {
                match self {
                    #(#hash_variants),*
                }
            }
        }
    )
}

fn gen_mixed_enum(index: usize) -> TokenStream {
    let chars = ('A'..='Z')
        .take(index)
        .map(|char| format_ident!("{char}"))
        .collect::<Vec<_>>();

    let name = format_ident!("{ENUM_NAME_PREFIX}{index}");

    let generics = quote!(
        <#(#chars),*>
    );

    let context = Context {
        name: &name,
        generics: &generics,
        chars: &chars,
    };

    let borrow_methods = chars.iter().map(gen_borrow);
    let borrow_mut_methods = chars.iter().map(gen_borrow_mut);
    let take_methods = chars.iter().map(gen_take);
    let copy_impl = gen_copy(&context);
    let clone_impl = gen_clone(&context);
    let debug_impl = gen_debug(&context);
    let eq_impl = gen_eq(&context);
    let ord_impl = gen_ord(&context);
    let hash_impl = gen_hash(&context);

    quote!(
        pub enum #name #generics {
            #(#chars(#chars)),*
        }

        #[allow(dead_code)]
        impl #generics #name #generics {
            #(#borrow_methods)*
            #(#borrow_mut_methods)*
            #(#take_methods)*
        }

        #copy_impl
        #clone_impl
        #debug_impl
        #eq_impl
        #ord_impl
        #hash_impl
    )
}

#[proc_macro]
pub fn mixed_type_proc(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let enums = (2..=12).map(gen_mixed_enum);

    quote!(
        #(#enums)*
    )
    .into()
}
