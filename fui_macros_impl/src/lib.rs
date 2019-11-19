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

//
// let control: Rc<RefCell<dyn ControlObject>> = ...;
//
// ui!(
//     Horizontal {
//         Row: 1,
//         spacing: 4,
//
//         Button { Text { text: "Button".to_string() } },
//         Text { text: "Label".to_string() },
//         @control,
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
//
//         control,
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
//
// StaticChildrenSource and DynamicChildrenSource can be aggregated with AggregatedChildrenSource.
//
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
    let (properties, attached_values, children) = decouple_params(params);

    let properties_builder = get_properties_builder(control_name.clone(), properties);
    let attached_values_typemap = get_attached_values_typemap(attached_values);
    let children_source = get_children_source(children);

    quote! { #properties_builder.to_view(ViewContext {
        attached_values: #attached_values_typemap,
        children: #children_source,
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

fn get_children_source(children: Vec<CtrlParam>) -> proc_macro2::TokenStream {
    let mut sources = Vec::new();

    let mut static_children = Vec::new();
    for child in children {
        if let CtrlParam::Ctrl(static_child) = child {
            static_children.push(quote_control(static_child));
        } else if let CtrlParam::RawCtrl(raw_control) = child {
            let name = raw_control.name;
            static_children.push(quote!(#name));
        } else if let CtrlParam::Collection(dynamic_child) = child {
            if static_children.len() > 0 {
                sources.push(quote!(Box::new(StaticChildrenSource::new(
                    vec![#(#static_children,)*]
                ))));
                static_children = Vec::new();
            }

            let reference = dynamic_child.reference;
            sources.push(quote!(Box::new(DynamicChildrenSource::new(#reference))));
        }
    }

    if static_children.len() > 0 {
        sources.push(quote!(Box::new(StaticChildrenSource::new(
            vec![#(#static_children,)*]
        ))));
    }

    let len = sources.len();
    if len == 0 {
        quote!(Box::new(StaticChildrenSource::new(Vec::<
            Rc<RefCell<ControlObject>>,
        >::new())))
    } else if len == 1 {
        sources.into_iter().next().unwrap()
    } else {
        quote!(Box::new(AggregatedChildrenSource::new(vec![#(#sources,)*])))
    }
}

fn decouple_params(
    params: Punctuated<CtrlParam, Token![,]>,
) -> (Vec<CtrlProperty>, Vec<CtrlProperty>, Vec<CtrlParam>) {
    let mut properties = Vec::new();
    let mut attached_values = Vec::new();
    let mut children = Vec::new();

    for el in params.into_pairs() {
        let el = el.into_value();

        if let CtrlParam::Property(property) = el {
            if let Some(first_char) = property.name.to_string().chars().next() {
                if first_char.is_uppercase() {
                    attached_values.push(property)
                } else {
                    properties.push(property);
                }
            }
        } else if let CtrlParam::Ctrl(control) = el {
            children.push(CtrlParam::Ctrl(control))
        } else if let CtrlParam::RawCtrl(raw_control) = el {
            children.push(CtrlParam::RawCtrl(raw_control))
        } else if let CtrlParam::Collection(c) = el {
            children.push(CtrlParam::Collection(c))
        }
    }

    (properties, attached_values, children)
}
