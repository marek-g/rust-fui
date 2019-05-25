extern crate proc_macro;


use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

use proc_macro2::Span;

use quote::quote;
use syn::punctuated::Punctuated;
use syn::{ parse_macro_input, Expr, FieldValue, Ident, Member, Token };

mod parser;
use crate::parser::Ctrl;
use crate::parser::CtrlParam;
use crate::parser::CtrlProperty;


#[proc_macro_hack]
pub fn ui(input: TokenStream) -> TokenStream {
    let ctrl = parse_macro_input!(input as Ctrl);
    let x = quote_control(ctrl);

    x.into()

    // for debug purposes - evaluate token stream to string
    //quote!(stringify!(#x)).into()
}

fn quote_control(ctrl: Ctrl) -> proc_macro2::TokenStream {
    let Ctrl { name: control_name, params } = ctrl;
    let (properties, controls) = decouple_params(params);

    let properties_name = Ident::new(&format!("{}Properties", control_name), control_name.span());
    let property_list = get_property_list(properties);
    let control_list = get_control_list(controls);

    let children = if control_list.len() > 0 {
        quote!(vec![#control_list])
    } else {
        quote!(Vec::<Rc<RefCell<ControlObject>>>::new())
    };

    quote!((#properties_name { #property_list }, #children))
}

fn get_property_list(properties: Vec<CtrlProperty>) -> Punctuated::<FieldValue, Token![,]> {
    let mut punctated_properties = Punctuated::<FieldValue, Token![,]>::new();

    for property in properties {
        let name = property.name;
        let expr = property.expr;

        let field_value = FieldValue {
            attrs: Vec::new(),
            member: Member::Named(name),
            colon_token: Some(syn::token::Colon {
                spans: [Span::call_site()],
            }),
            expr: expr,
        };

        punctated_properties.push(field_value);
    }

    punctated_properties
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
