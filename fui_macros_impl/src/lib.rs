extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;


use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, Token};

mod parser;
use crate::parser::Ctrl;
use crate::parser::CtrlParam;
use crate::parser::CtrlProperty;

// ui!(
//     Horizontal {
//         spacing: 4,
//         Button { Text { text: "Button".to_string() } },
//         Text { text: "Label".to_string() }
//     }
// )
//
// translates to:
//
// <Horizontal>::builder().spacing(4).build().to_view(ViewContext {
//     children: vec![
//
//         <Button>::builder().build().to_view(ViewContext {
//             children: vec![
//
//                 <Text::builder()
//                     .text("Button".to_string())
//                     .build().to_view(ViewContext {
//                         children: Vec::<Rc<RefCell<ControlObject>>>::new()
//                     }),
//
//             )],
//         }),
//
//         <Text>::builder()
//             .text("Label".to_string())
//             .build().to_view(ViewContext {
//                 children: Vec::<Rc<RefCell<ControlObject>>>::new()
//             }),
//         ),
//     ],
// })
#[proc_macro_hack]
pub fn ui(input: TokenStream) -> TokenStream {
    let ctrl = parse_macro_input!(input as Ctrl);
    let x = quote_control(ctrl);

    x.into()

    // for debug purposes - evaluate token stream to string
    //quote!(stringify!(#x)).into()
}

fn quote_control(ctrl: Ctrl) -> proc_macro2::TokenStream {
    let Ctrl {
        name: control_name,
        params,
    } = ctrl;
    let (properties, controls) = decouple_params(params);

    let properties_builder = get_properties_builder(control_name.clone(), properties);

    let children = get_control_children(controls);

    quote! { #properties_builder.to_view(ViewContext {
        children: #children
    }) }
}

fn get_properties_builder(
    struct_name: Ident,
    properties: Vec<CtrlProperty>,
) -> proc_macro2::TokenStream {
    let mut method_calls = Vec::new();
    for property in properties {
        let name = property.name;
        let expr = property.expr;
        method_calls.push(quote!(.#name(#expr)))
    }
    quote!(<#struct_name>::builder()#(#method_calls)*.build())
}

fn get_control_children(controls: Vec<Ctrl>) -> proc_macro2::TokenStream {
    let mut children = Vec::new();
    for control in controls {
        children.push(quote_control(control));
    }

    if children.len() > 0 {
        quote!(vec![#(#children,)*])
    } else {
        quote!(Vec::<Rc<RefCell<ControlObject>>>::new())
    }
}

fn decouple_params(params: Punctuated<CtrlParam, Token![,]>) -> (Vec<CtrlProperty>, Vec<Ctrl>) {
    let mut properties = Vec::new();
    let mut controls = Vec::new();

    for el in params.into_pairs() {
        let el = el.into_value();

        if let CtrlParam::Property(property) = el {
            properties.push(property);
        } else if let CtrlParam::Ctrl(control) = el {
            controls.push(control)
        }
    }

    (properties, controls)
}
