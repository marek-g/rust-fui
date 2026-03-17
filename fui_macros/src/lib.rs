extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, Token};

mod parser;
use crate::parser::Ctrl;
use crate::parser::CtrlCallback;
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
//         control,
//     }
// )
//
// translates to:
//
// <Horizontal>::builder().spacing(4.into()).build().to_view(None, ViewContext {
//     attached_values: { let mut map = TypeMap::new(); map.insert::<Row>(1.into()); map },
//     children: Children::from(vec![
//
//         (<Button>::builder().build().to_view(None, ViewContext {
//             attached_values: TypeMap::new(),
//             children: Children::from(vec![
//
//                 (<Text>::builder()
//                     .text("Button".to_string().into())
//                     .build().to_view(None, ViewContext {
//                         attached_values: TypeMap::new(),
//                         children: Box::new(Vec::<Rc<RefCell<dyn ControlObject>>>::new()),
//                     }) as Rc<RefCell<dyn ControlObject>>).into(),
//
//             )]),
//         })).into(),
//
//         <Text>::builder()
//             .text("Label".to_string().into())
//             .build().to_view(None, ViewContext {
//                 attached_values: TypeMap::new(),
//                 children: Children::from(vec![]),
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
//     children: Children::from(vec![(&vm.items).into()]),
// })
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
// <Text>::builder().text("Text".into()).build().to_view(
//     Some(Box::new(<DefaultTextStyle>::new(<DefaultTextStyleParams>::builder()
//         color([1.0, 0.0, 0.0, 1.0].into()).build()))),
//     ViewContext {
//         attached_values: TypeMap::new(),
//         children: Children::from(vec![]),
//     }
// )
//
#[proc_macro]
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

    let (style, properties, callbacks, attached_values, children) = decouple_params(params);

    let properties_builder = get_properties_builder(control_name.clone(), properties, callbacks);
    let style_builder = get_style_builder(control_name, style);
    let attached_values_typemap = get_attached_values_typemap(attached_values);
    let children_source = get_children_source(children);

    quote! {
        {
            let __av = #attached_values_typemap;
            #properties_builder.to_view(#style_builder, ViewContext {
                attached_values: __av.0,
                inherited_values: __av.1,
                children: #children_source,
            })
        }
    }
}

fn get_properties_builder(
    struct_name: Ident,
    properties: Vec<CtrlProperty>,
    callbacks: Vec<CtrlCallback>,
) -> proc_macro2::TokenStream {
    use crate::parser::extract_self_properties_via_get;

    let mut method_calls = Vec::new();
    for property in properties {
        let name = property.name;
        let expr = property.expr;

        // Detect self.property.get() calls in the expression
        let prop_fields = extract_self_properties_via_get(&expr);

        if prop_fields.is_empty() {
            // No .get() calls - standard expression without tracking
            method_calls.push(quote!(.#name((#expr).into())));
        } else {
            // .get() calls detected - automatic dependency tracking
            // Replace 'self' with '_self_cloned' in the expression
            let mut _uses_self = false;
            let expr_tokens = quote!(#expr);
            let expr_replaced = replace_self(expr_tokens, &mut _uses_self);

            // Also replace 'self' with '_self_cloned' in field names and generate subscription code
            let subscription_code = prop_fields.iter().map(|ident| {
                let field_tokens = quote!(self.#ident);
                let mut _unused = false;
                let field_replaced = replace_self(field_tokens, &mut _unused);
                quote!({
                    let _self_for_sub = _self_cloned.clone();
                    let target = result.clone();
                    let handle = #field_replaced.on_changed(move |_| {
                        let _self_cloned = _self_for_sub.clone();
                        target.set(#expr_replaced);
                    });
                    result.add_bind_subscription(handle);
                })
            });

            method_calls.push(quote!(.#name({
                let _self_cloned = self.clone();
                let result = Property::new(#expr_replaced);
                #(#subscription_code)*
                result.into()
            })));
        }
    }

    // Generate callback code
    for callback in callbacks {
        let name = callback.name;
        let arg_pat = callback.arg_pat;
        let is_async = callback.is_async;
        let expr = callback.expr;

        // Replace 'self' with '_self_cloned' in the expression
        let mut uses_self = false;
        let expr_tokens = quote!(#expr);
        let expr_replaced = replace_self(expr_tokens, &mut uses_self);

        let callback_code = if is_async {
            // Async callback
            if let Some(pat) = arg_pat {
                // With argument: name(arg) async => expression
                quote!({
                    let _self_cloned = self.clone();
                    ::fui_core::Callback::new_async(move |#pat: _| {
                        let _self_cloned = _self_cloned.clone();
                        async move {
                            (#expr_replaced).await
                        }
                    })
                })
            } else {
                // Without argument: name async => expression
                quote!({
                    let _self_cloned = self.clone();
                    ::fui_core::Callback::new_async(move |_| {
                        let _self_cloned = _self_cloned.clone();
                        async move {
                            (#expr_replaced).await
                        }
                    })
                })
            }
        } else {
            // Sync callback
            if let Some(pat) = arg_pat {
                // With argument: name(arg) => expression
                quote!({
                    let _self_cloned = self.clone();
                    ::fui_core::Callback::new_sync(move |#pat: _| {
                        let _self_cloned = _self_cloned.clone();
                        #expr_replaced
                    })
                })
            } else {
                // Without argument: name => expression
                quote!({
                    let _self_cloned = self.clone();
                    ::fui_core::Callback::new_sync(move |_| {
                        let _self_cloned = _self_cloned.clone();
                        #expr_replaced
                    })
                })
            }
        };

        method_calls.push(quote!(.#name(#callback_code)));
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

            let (_style, properties, _callbacks, _attached_values, _children) = decouple_params(style.params);

            let properties_builder = get_properties_builder(params_name, properties, vec![]);

            quote!(Some(Box::new(<#style_name>::new(#properties_builder))))
        }
    }
}

fn get_attached_values_typemap(attached_values: Vec<CtrlProperty>) -> proc_macro2::TokenStream {
    let mut insert_statements = Vec::new();
    for attached_value in attached_values {
        let name = attached_value.name;
        let expr = attached_value.expr;
        // Use insert_attached_value which routes to the correct map based on registration
        insert_statements.push(quote! {
            ::fui_core::insert_attached_value::<#name>((#expr).into(), &mut attached_map, &mut inherited_map);
        })
    }
    quote! {
        {
            let mut attached_map = ::fui_core::TypeMap::new();
            let mut inherited_map = ::fui_core::InheritedTypeMap::new();
            #(#insert_statements)*
            (attached_map, inherited_map)
        }
    }
}

fn get_children_source(children: Vec<CtrlParam>) -> proc_macro2::TokenStream {
    let mut sources = Vec::new();

    for child in children {
        if let CtrlParam::ChildCtrl(static_child) = child {
            sources.push(quote_control(static_child));
        } else if let CtrlParam::ChildExpr(expression) = child {
            sources.push(quote!(#expression));
        } else if let CtrlParam::ForLoop(for_loop) = child {
            let pat = for_loop.pat;
            let expr = for_loop.expr;

            let mut loop_children = Vec::new();
            for c in for_loop.body {
                if let CtrlParam::ChildCtrl(ctrl) = c {
                    loop_children.push(quote_control(ctrl));
                } else if let CtrlParam::ChildExpr(e) = c {
                    loop_children.push(quote!(#e));
                }
            }

            let mut uses_self = false;

            // decide if to use map or flat_map depending on number of children
            let source = if loop_children.len() == 1 {
                let single_child = &loop_children[0];
                let single_child_replaced = replace_self(quote!(#single_child), &mut uses_self);

                if uses_self {
                    quote! {
                        {
                            let _self_cloned = self.clone();
                            (#expr).map({
                                move |#pat| {
                                    let #pat = #pat.clone();
                                    let _self_cloned = &_self_cloned;
                                    #single_child_replaced
                                }
                            })
                        }
                    }
                } else {
                    quote! {
                        (#expr).map({
                            move |#pat| {
                                let #pat = #pat.clone();
                                #single_child
                            }
                        })
                    }
                }
            } else {
                let loop_children_replaced: Vec<_> = loop_children
                    .iter()
                    .map(|c| replace_self(quote!(#c), &mut uses_self))
                    .collect();

                if uses_self {
                    quote! {
                        {
                            let _self_cloned = self.clone();
                            (#expr).flat_map({
                                move |#pat| {
                                    let #pat = #pat.clone();
                                    let _self_cloned = &_self_cloned;
                                    vec![ #(#loop_children_replaced),* ]
                                }
                            })
                        }
                    }
                } else {
                    quote! {
                        (#expr).flat_map({
                            move |#pat| {
                                let #pat = #pat.clone();
                                vec![ #(#loop_children),* ]
                            }
                        })
                    }
                }
            };

            sources.push(source);
        }
    }

    quote!({ Children::from(vec![#((#sources).into()),*]) })
}

fn decouple_params(
    params: Punctuated<CtrlParam, Token![,]>,
) -> (
    Option<Ctrl>,
    Vec<CtrlProperty>,
    Vec<CtrlCallback>,
    Vec<CtrlProperty>,
    Vec<CtrlParam>,
) {
    let mut style = None;
    let mut properties = Vec::new();
    let mut attached_values = Vec::new();
    let mut children = Vec::new();
    let mut callbacks = Vec::new();

    for el in params.into_pairs() {
        let el = el.into_value();

        if let CtrlParam::Style(s) = el {
            style = Some(s);
        } else if let CtrlParam::Callback(callback) = el {
            callbacks.push(callback);
        } else if let CtrlParam::Property(property) = el {
            if let Some(first_char) = property.name.to_string().chars().next() {
                if first_char.is_uppercase() {
                    attached_values.push(property);
                } else {
                    properties.push(property);
                }
            }
        } else if let CtrlParam::ForLoop(for_loop) = el {
            children.push(CtrlParam::ForLoop(for_loop));
        } else if let CtrlParam::ChildCtrl(control) = el {
            children.push(CtrlParam::ChildCtrl(control))
        } else if let CtrlParam::ChildExpr(expression) = el {
            children.push(CtrlParam::ChildExpr(expression))
        }
    }

    (style, properties, callbacks, attached_values, children)
}

/// A function that recursively passes through a TokenStream.
/// Changes each instance of 'self' to '_self_cloned' and sets the 'uses_self' flag.
fn replace_self(
    stream: proc_macro2::TokenStream,
    uses_self: &mut bool,
) -> proc_macro2::TokenStream {
    use proc_macro2::TokenTree;
    stream
        .into_iter()
        .map(|tt| match tt {
            TokenTree::Ident(ref ident) if ident == "self" => {
                *uses_self = true;
                TokenTree::Ident(proc_macro2::Ident::new("_self_cloned", ident.span()))
            }
            TokenTree::Group(group) => {
                let mut new_group = proc_macro2::Group::new(
                    group.delimiter(),
                    replace_self(group.stream(), uses_self),
                );
                new_group.set_span(group.span());
                TokenTree::Group(new_group)
            }
            _ => tt,
        })
        .collect()
}
