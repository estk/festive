use quote::quote;
use syn::{
    parse_macro_input, parse_quote, AttributeArgs, ItemFn, Lit, Meta, MetaNameValue, NestedMeta,
    Path,
};

#[proc_macro_attribute]
pub fn festive(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let timeout: syn::Expr = syn::parse_str(&format!("{:?}", parse_args(&args))).unwrap();

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(input as ItemFn);
    let fun_name = sig.ident.to_string();

    (quote! {
        #(
            #attrs
        )*
        #[test]
        #vis #sig {
            fn inner() #block

            struct _Anon;
            let fork_id = ::festive::ForkId::of(::std::any::TypeId::of::<_Anon>());

            let path = format!("{}::{}", ::std::module_path!(), #fun_name);

            let res = ::festive::fork(
                &path,
                fork_id,
                #timeout,
                inner,
            ).expect("forking test failed");

            let stringout = format!(
                "Child stdout:: {}\nChild stderr: {}",
                String::from_utf8_lossy(&res.stdout),
                String::from_utf8_lossy(&res.stderr)
            );

            assert!(res.status.success(), stringout);
        }
    })
    .into()
}

fn parse_args(args: &[NestedMeta]) -> Option<u64> {
    let timeout_path: Path = parse_quote!(timeout_ms);
    for a in args {
        match a {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. }))
                if path == &timeout_path =>
            {
                if let Lit::Int(li) = lit {
                    return li.base10_parse().ok();
                } else {
                    panic!("timeout_ms should have an int value")
                }
            }
            _ => {}
        }
    }
    None
}
