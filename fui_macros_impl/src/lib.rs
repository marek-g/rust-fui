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
// <Horizontal>::builder().spacing(4.into()).build().to_view(None, ViewContext {
//     attached_values: { let mut map = TypeMap::new(); map.insert::<Row>(1.into()); map },
//     children: Box::new(vec![
//
//         <Button>::builder().build().to_view(None, ViewContext {
//             attached_values: TypeMap::new(),
//             children: Box::new(vec![
//
//                 <Text>::builder()
//                     .text("Button".to_string().into())
//                     .build().to_view(None, ViewContext {
//                         attached_values: TypeMap::new(),
//                         children: Box::new(Vec::<Rc<RefCell<dyn ControlObject>>>::new()),
//                     }) as Rc<RefCell<dyn ControlObject>>,
//
//             )]),
//         }),
//
//         <Text>::builder()
//             .text("Label".to_string().into())
//             .build().to_view(None, ViewContext {
//                 attached_values: TypeMap::new(),
//                 children: Box::new(Vec::<Rc<RefCell<dyn ControlObject>>>::new()),
//             }),
//         ),
//
//         control,
//     ]),
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
// <Vertical>::builder().build().to_view(None, ViewContext {
//     attached_values: TypeMap::new(),
//     children: Box::<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>::from(&vm.items),
// })
//
// Vecs and ObservableCollections can be aggregated with AggregatedChildrenSource.
//
// ui!(
//     Text {
//         Style: Default {
//             color: [1.0, 0.0, 0.0, 1.0],
//         },
//         text: "Text",
//     }
// )
//
// translates to:
//
// <Text>::builder().build().to_view(
//     Some(Box::new(<DefaultTextStyle>::new(<DefaultTextStyleParams>::builder()
//         color([1.0, 0.0, 0.0, 1.0].into()).build()))),
//     ViewContext {
//         attached_values: TypeMap::new(),
//         children: Box::<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>::from(&vm.items),
//     }
// )
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

    let (style, properties, attached_values, children) = decouple_params(params);

    let properties_builder = get_properties_builder(control_name.clone(), properties);
    let style_builder = get_style_builder(control_name.clone(), style);
    let attached_values_typemap = get_attached_values_typemap(attached_values);
    let children_source = get_children_source(children);

    quote! { #properties_builder.to_view(#style_builder, ViewContext {
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
        method_calls.push(quote!(.#name((#expr).into())))
    }
    quote!(<#struct_name>::builder()#(#method_calls)*.build())
}

fn get_style_builder(control_name: Ident, style: Option<Ctrl>) -> proc_macro2::TokenStream {
    match style {
        None => quote!(None),
        Some(style) => {
            let name = style.name;

            let style_name = Ident::new(&format!("{}{}Style", name, control_name), name.span());
            let params_name =
                Ident::new(&format!("{}{}StyleParams", name, control_name), name.span());

            let (_style, properties, _attached_values, _children) = decouple_params(style.params);

            let properties_builder = get_properties_builder(params_name, properties);

            quote!(Some(Box::new(<#style_name>::new(#properties_builder))))
        }
    }
}

fn get_attached_values_typemap(attached_values: Vec<CtrlProperty>) -> proc_macro2::TokenStream {
    let mut insert_statements = Vec::new();
    for attached_value in attached_values {
        let name = attached_value.name;
        let expr = attached_value.expr;
        insert_statements.push(quote!(map.insert::<#name>(#expr.into());))
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
                sources.push(quote!(Box::new(
                    vec![#(#static_children as Rc<RefCell<dyn ControlObject>>,)*]
                )));
                static_children = Vec::new();
            }

            let reference = dynamic_child.reference;
            sources.push(quote!(Box::<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>::from(#reference)));
        }
    }

    if static_children.len() > 0 {
        sources.push(quote!(Box::new(
            vec![#(#static_children as Rc<RefCell<dyn ControlObject>>,)*]
        )));
    }

    let len = sources.len();
    if len == 0 {
        quote!(Box::new(Vec::<Rc<RefCell<ControlObject>>>::new()))
    } else if len == 1 {
        sources.into_iter().next().unwrap()
    } else {
        quote!(Box::new(AggregatedChildrenSource::new(vec![#(#sources,)*])))
    }
}

fn decouple_params(
    params: Punctuated<CtrlParam, Token![,]>,
) -> (
    Option<Ctrl>,
    Vec<CtrlProperty>,
    Vec<CtrlProperty>,
    Vec<CtrlParam>,
) {
    let mut style = None;
    let mut properties = Vec::new();
    let mut attached_values = Vec::new();
    let mut children = Vec::new();

    for el in params.into_pairs() {
        let el = el.into_value();

        if let CtrlParam::Style(s) = el {
            style = Some(s);
        } else if let CtrlParam::Property(property) = el {
            if let Some(first_char) = property.name.to_string().chars().next() {
                if first_char.is_uppercase() {
                    attached_values.push(property);
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

    (style, properties, attached_values, children)
}
