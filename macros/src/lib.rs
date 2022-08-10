use proc_macro::{TokenStream, };
use quote::{quote, ToTokens, format_ident};

#[proc_macro_attribute]
pub fn permission(attr: TokenStream, content : TokenStream) -> TokenStream {
    let mut item: syn::Item = syn::parse(content).unwrap();
    
    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("expected fn !")
    };

    let mut scope_arg : Option<String> = None;
    
    for arg in fn_item.sig.inputs.clone() {
        let tokens : Vec<String> = arg.into_token_stream().into_iter().map(|t| {
            t.to_string().clone()
        })
        .collect();

        if tokens.contains(&"mut".to_string()) && tokens.contains(&"HandleScope".to_string()) {
            scope_arg = Some(tokens.get(0).unwrap().to_string());
        }
    }

    let scope = match scope_arg {
        Some(a) => a,
        None    => panic!("fn must have &mut v8::HandleScope!")
    };

    let perm = attr.to_string();

    // Add these lines to the start of the function
    let scope_var = format_ident!("{}", scope);

    let q = quote! {
        match Avdan::security::Constraints::from_scope(#scope_var)
              .throw_permission_exception(#scope_var, #perm) {
          true  => {},
          false => return
        };
    };

    let to_add = syn::parse (
        q.into()
    );

    match to_add {
        Err(e) => {
            panic!("{}",
                format! (
                    "{:?}", e.into_compile_error()
                )
            );
        },
        
        Ok(body) => {
            fn_item.block.stmts.insert(0, body);
            fn_item.to_token_stream().into()
        }
    }
}
