extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;


use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, Ident, Token};

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
// Control::new(
//     <Horizontal>::new(<HorizontalProperties>::builder().spacing(4).build()),
//     <HorizontalDefaultStyle>::new(),
//     vec![
//         Control::new(
//             <Button>::new(<ButtonProperties>::builder().build()),
//             <ButtonDefaultStyle>::new(),
//             vec![Control::new(
//                 <Text>::new(
//                     <TextProperties>::builder()
//                         .text("Button".to_string())
//                         .build(),
//                 ),
//                 <TextDefaultStyle>::new(),
//                 Vec::<Rc<RefCell<ControlObject>>>::new(),
//             )],
//         ),
//         Control::new(
//             <Text>::new(
//                 <TextProperties>::builder()
//                     .text("Label".to_string())
//                     .build(),
//             ),
//             <TextDefaultStyle>::new(),
//             Vec::<Rc<RefCell<ControlObject>>>::new(),
//         ),
//     ],
// )
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

    let properties_name = Ident::new(&format!("{}Properties", control_name), control_name.span());
    let properties_builder = get_properties_builder(properties_name, properties);

    let control_style = Ident::new(
        &format!("{}DefaultStyle", control_name),
        control_name.span(),
    );

    let children = get_control_children(controls);

    quote! {
        Control::new(<#control_name>::new(#properties_builder),
            <#control_style>::new(),
            #children)
    }
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

fn get_control_list(controls: Vec<Ctrl>) -> Punctuated<Expr, Token![,]> {
    let mut punctated_controls = Punctuated::<Expr, Token![,]>::new();

    for control in controls {
        let control_tokens = quote_control(control);
        let ctrl_expr: Expr = syn::parse2(control_tokens).unwrap();
        punctated_controls.push(ctrl_expr);
    }

    punctated_controls
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
