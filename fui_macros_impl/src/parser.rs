
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{braced, Expr, Ident, Token};


/// Syntax of ui! macro.
///
/// ```ignore
/// ui! {
///     $control:Ctrl
/// }
/// ```
///
/// Syntax of Ctrl:
///
/// ```ignore
/// $ctrlName {
///     $( $propertyName:Literal : $value:Expr, )*
///     $( $child:Ctrl  )*
/// }
/// ```
///
/// For example:
///
/// ```ignore
/// ui!(
///     Horizontal {
///         spacing: 5,
///
/// 		Button { Text { text: "Button".to_string() } },
/// 		Text { text: "Label".to_string() }
///     }
/// );
/// ```
pub struct Ctrl {
    pub name: Ident,
    pub params: Punctuated<CtrlParam, Token![,]>,
}

impl Parse for Ctrl {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let params = content.parse_terminated(CtrlParam::parse)?;
        Ok(Ctrl { name, params })
    }
}

pub enum CtrlParam {
    Property(CtrlProperty),
    Ctrl(Ctrl),
}

impl Parse for CtrlParam {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Ident) && input.peek2(Token![:]) {
            input.parse().map(CtrlParam::Property)
        } else {
            input.parse().map(CtrlParam::Ctrl)
        }
    }
}

pub struct CtrlProperty {
    pub name: Ident,
    pub expr: Expr,
}

impl Parse for CtrlProperty {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let expr: Expr = input.parse()?;
        Ok(CtrlProperty { name, expr })
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Ctrl, CtrlParam, CtrlProperty};
    use proc_macro2::Span;
    use syn::{parse_quote, Ident};

    #[test]
    fn test_property() {
        let prop: CtrlProperty = parse_quote!(property1: 4 + 5);
        assert_eq!(prop.name, Ident::new("property1", Span::call_site()));
    }

    #[test]
    fn test_empty_control() {
        let ctrl: Ctrl = parse_quote!(Button {});
        assert_eq!(ctrl.name, Ident::new("Button", Span::call_site()));
    }

    #[test]
    fn test_ctrl_param_property() {
        let ctrl_param: CtrlParam = parse_quote!(margin: 5);
        if let CtrlParam::Property(_) = ctrl_param {
        } else {
            panic!("Expected CtrlParam::Property")
        }
    }

    #[test]
    fn test_ctrl_param_ctrl() {
        let ctrl_param: CtrlParam = parse_quote!(Button {});
        if let CtrlParam::Ctrl(_) = ctrl_param {
        } else {
            panic!("Expected CtrlParam::Ctrl")
        }
    }

    #[test]
    fn test_control_with_properties1() {
        let ctrl: Ctrl = parse_quote!(Button {
            property1: 5,
            property2: Test {},
        });
        assert_eq!(ctrl.name, Ident::new("Button", Span::call_site()));
        assert_eq!(ctrl.params.len(), 2);
    }

    #[test]
    fn test_control_with_properties2() {
        let ctrl: Ctrl = parse_quote!(Horizontal {
            spacing: 5,
            Button { Text { text: "Button".to_string() } },
            Text { text: "Label".to_string() },
        });

        assert_eq!(ctrl.name, Ident::new("Horizontal", Span::call_site()));
        assert_eq!(ctrl.params.len(), 3);

        let mut params = ctrl.params.into_iter();

        let param = params.next().unwrap();
        if let CtrlParam::Property(param) = param {
            assert_eq!(param.name, Ident::new("spacing", Span::call_site()));
        } else {
            panic!("Expected spacing CtrlParam::Property");
        }

        let param = params.next().unwrap();
        if let CtrlParam::Ctrl(param) = param {
            assert_eq!(param.name, Ident::new("Button", Span::call_site()));
            let mut params = param.params.into_iter();
            let param = params.next().unwrap();
            if let CtrlParam::Ctrl(param) = param {
                assert_eq!(param.name, Ident::new("Text", Span::call_site()));
            } else {
                panic!("Expected Text CtrlParam::Ctrl");
            }
        } else {
            panic!("Expected Button CtrlParam::Ctrl");
        }

        let param = params.next().unwrap();
        if let CtrlParam::Ctrl(param) = param {
            assert_eq!(param.name, Ident::new("Text", Span::call_site()));
            let mut params = param.params.into_iter();
            let param = params.next().unwrap();
            if let CtrlParam::Property(param) = param {
                assert_eq!(param.name, Ident::new("text", Span::call_site()));
            } else {
                panic!("Expected text CtrlParam::Property");
            }
        } else {
            panic!("Expected Text CtrlParam::Ctrl");
        }
    }
}
