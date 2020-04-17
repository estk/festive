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
    let input = parse_macro_input!(input as ItemFn);

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;
    let fun_name = sig.ident.to_string();

    let timeout = parse_args(&args);

    (quote! {
        #(
            #attrs
        )*
        #[test]
        #vis #sig {
            fn inner() #block

            fn supervise(child: &mut rusty_fork::ChildWrapper,
                            _file: &mut ::std::fs::File) {
                rusty_fork::fork_test::supervise_child(child, #timeout)
            }

            struct _Anon;
            let fork_id = rusty_fork::RustyForkId::of(::std::any::TypeId::of::<_Anon>());

            // Convert the path for fork
            let path = format!("{}::{}", ::std::module_path!(), #fun_name);
            let path = path.find("::").map(|ix| &path[ix+2..]).unwrap_or(&path);

            rusty_fork::fork(
                path,
                fork_id,
                rusty_fork::fork_test::no_configure_child,
                supervise,
                inner
            ).expect("forking test failed")
        }
    })
    .into()
}

fn parse_args(args: &[NestedMeta]) -> u64 {
    let timeout_path: Path = parse_quote!(timeout_ms);
    for a in args {
        match a {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. }))
                if path == &timeout_path =>
            {
                if let Lit::Int(li) = lit {
                    return li.base10_parse().unwrap();
                } else {
                    panic!("timeout_ms should have an int value")
                }
            }
            _ => {}
        }
    }
    0
}
