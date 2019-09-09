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
use crate::parser::Collection;

// ui!(
//     Horizontal {
//         Row: 1,
//         spacing: 4,
//         Button { Text { text: "Button".to_string() } },
//         Text { text: "Label".to_string() }
//     }
// )
//
// translates to:
//
// <Horizontal>::builder().spacing(4).build().to_view(ViewContext {
//     attached_values: { let mut map = TypeMap::new(); map.insert::<Row>(1); map },
//     children: Box::new(StaticChildrenSource::new(vec![
//
//         <Button>::builder().build().to_view(ViewContext {
//             attached_values: TypeMap::new(),
//             children: Box::new(StaticChildrenSource::new(vec![
//
//                 <Text::builder()
//                     .text("Button".to_string())
//                     .build().to_view(ViewContext {
//                         attached_values: TypeMap::new(),
//                         children: Box::new(StaticChildrenSource::new(Vec::<Rc<RefCell<ControlObject>>>::new()))
//                     }),
//
//             )])),
//         }),
//
//         <Text>::builder()
//             .text("Label".to_string())
//             .build().to_view(ViewContext {
//                 attached_values: TypeMap::new(),
//                 children: Box::new(StaticChildrenSource::new(Vec::<Rc<RefCell<ControlObject>>>::new()))
//             }),
//         ),
//     ])),
// })
//
// ui!(
//     Vertical {
//         &vm.items,
//     }
// )
//
// translates to:
//
// <Vertical>::builder().build().to_view(ViewContext {
//     attached_values: TypeMap::new(),
//     children: Box::new(DynamicChildrenSource::new(&vm.items)),
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
    let (properties, attached_values, controls, collection) = decouple_params(params);

    let properties_builder = get_properties_builder(control_name.clone(), properties);
    let attached_values_typemap = get_attached_values_typemap(attached_values);

    let children = if let Some(collection) = collection {
        let reference = collection.reference;
        quote!(Box::new(DynamicChildrenSource::new(#reference)))
    } else {
        get_control_children(controls)
    };

    quote! { #properties_builder.to_view(ViewContext {
        attached_values: #attached_values_typemap,
        children: #children,
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

fn get_attached_values_typemap(attached_values: Vec<CtrlProperty>) -> proc_macro2::TokenStream {
    let mut insert_statements = Vec::new();
    for attached_value in attached_values {
        let name = attached_value.name;
        let expr = attached_value.expr;
        insert_statements.push(quote!(map.insert::<#name>(#expr);))
    }
    quote!({ let mut map = TypeMap::new(); #(#insert_statements)* map })
}

fn get_control_children(controls: Vec<Ctrl>) -> proc_macro2::TokenStream {
    let mut children = Vec::new();
    for control in controls {
        children.push(quote_control(control));
    }

    if children.len() > 0 {
        quote!(Box::new(StaticChildrenSource::new(vec![#(#children,)*])))
    } else {
        quote!(Box::new(StaticChildrenSource::new(Vec::<Rc<RefCell<ControlObject>>>::new())))
    }
}

fn decouple_params(params: Punctuated<CtrlParam, Token![,]>) -> (Vec<CtrlProperty>, Vec<CtrlProperty>, Vec<Ctrl>, Option<Collection>) {
    let mut properties = Vec::new();
    let mut attached_values = Vec::new();
    let mut controls = Vec::new();
    let mut collection = None;

    for el in params.into_pairs() {
        let el = el.into_value();

        if let CtrlParam::Property(property) = el {
            if let Some(first_char) = property.name.to_string().chars().next() {
                if first_char.is_uppercase() {
                    attached_values.push(property)
                }
                else {
                    properties.push(property);
                }
            }
        } else if let CtrlParam::Ctrl(control) = el {
            controls.push(control)
        } else if let CtrlParam::Collection(c) = el {
            collection = Some(c)
        }
    }

    (properties, attached_values, controls, collection)
}
