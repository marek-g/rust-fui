use proc_macro2::TokenTree;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{braced, Expr, Ident, Pat, Token};

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
///         Button { Text { text: "Button".to_string() } },
///         Text { text: "Label".to_string() }
///     }
/// );
/// ```

mod keyword {
    syn::custom_keyword!(Style);
}

pub struct Ctrl {
    pub name: Ident,
    pub params: Punctuated<CtrlParam, Token![,]>,
}

impl Parse for Ctrl {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let params = content.parse_terminated(CtrlParam::parse, Token![,])?;
        Ok(Ctrl { name, params })
    }
}

pub enum CtrlParam {
    // Style: Default { property: "Value", },
    Style(Ctrl),

    // property: "value",
    Property(CtrlProperty),

    // Button { ... }
    ChildCtrl(Ctrl),

    // control, &vm.items
    ChildExpr(Expr),

    // for item in self.items { Button { ... } }
    ForLoop(CtrlForLoop),
}

impl Parse for CtrlParam {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(keyword::Style) && input.peek2(Token![:]) {
            input.parse::<keyword::Style>()?;
            input.parse::<Token![:]>()?;
            input.parse().map(CtrlParam::Style)
        } else if input.peek(Token![for]) {
            input.parse().map(CtrlParam::ForLoop)
        } else if input.peek(Ident) && input.peek2(Token![:]) {
            input.parse().map(CtrlParam::Property)
        } else if input.peek(Ident) && input.peek2(syn::token::Brace) {
            input.parse().map(CtrlParam::ChildCtrl)
        } else {
            input.parse().map(CtrlParam::ChildExpr)
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

pub struct CtrlForLoop {
    pub pat: Pat,
    pub expr: Expr,
    pub body: Punctuated<CtrlParam, Token![,]>,
}

impl Parse for CtrlForLoop {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![for]>()?;
        let pat: Pat = syn::Pat::parse_single(input)?;
        input.parse::<Token![in]>()?;
        let expr: Expr = input.parse()?;

        let content;
        braced!(content in input);

        let body = content.parse_terminated(CtrlParam::parse, Token![,])?;

        Ok(CtrlForLoop { pat, expr, body })
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
        if let CtrlParam::ChildCtrl(_) = ctrl_param {
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
            Row: 1,
            spacing: 5,
            Button { Text { text: "Button".to_string() } },
            Text { text: "Label".to_string() },
        });

        assert_eq!(ctrl.name, Ident::new("Horizontal", Span::call_site()));
        assert_eq!(ctrl.params.len(), 4);

        let mut params = ctrl.params.into_iter();

        let param = params.next().unwrap();
        if let CtrlParam::Property(param) = param {
            assert_eq!(param.name, Ident::new("Row", Span::call_site()));
        } else {
            panic!("Expected Row CtrlParam::Property");
        }

        let param = params.next().unwrap();
        if let CtrlParam::Property(param) = param {
            assert_eq!(param.name, Ident::new("spacing", Span::call_site()));
        } else {
            panic!("Expected spacing CtrlParam::Property");
        }

        let param = params.next().unwrap();
        if let CtrlParam::ChildCtrl(param) = param {
            assert_eq!(param.name, Ident::new("Button", Span::call_site()));
            let mut params = param.params.into_iter();
            let param = params.next().unwrap();
            if let CtrlParam::ChildCtrl(param) = param {
                assert_eq!(param.name, Ident::new("Text", Span::call_site()));
            } else {
                panic!("Expected Text CtrlParam::Ctrl");
            }
        } else {
            panic!("Expected Button CtrlParam::Ctrl");
        }

        let param = params.next().unwrap();
        if let CtrlParam::ChildCtrl(param) = param {
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

    #[test]
    fn test_control_with_style() {
        let ctrl: Ctrl = parse_quote!(Text {
            Style: Default {
                color: [1.0f32, 0.0f32, 0.0f32, 1.0f32],
            },
            property1: 5,
        });
        assert_eq!(ctrl.name, Ident::new("Text", Span::call_site()));
        assert_eq!(ctrl.params.len(), 2);

        let mut params = ctrl.params.into_iter();

        let param = params.next().unwrap();
        if let CtrlParam::Style(style) = param {
            assert_eq!(style.name, Ident::new("Default", Span::call_site()));
            assert_eq!(style.params.len(), 1);

            let mut params = style.params.into_iter();
            let param = params.next().unwrap();
            if let CtrlParam::Property(param) = param {
                assert_eq!(param.name, Ident::new("color", Span::call_site()));
            } else {
                panic!("Expected color CtrlParam::Property");
            }
        } else {
            panic!("Expected CtrlParam::Style");
        }
    }
}

/// Extracts all `self.property_name.get()` method calls from an expression.
///
/// This is used for automatic dependency tracking in the ui! macro.
/// When a property is accessed via `.get()`, it indicates that the value
/// should be tracked for changes.
///
/// # Example
///
/// For expression `format!("{}: {}", self.name.get(), self.counter.get())`,
/// returns `["name", "counter"]`.
pub fn extract_self_properties_via_get(expr: &Expr) -> Vec<Ident> {
    use syn::visit::Visit;

    struct GetVisitor {
        fields: Vec<Ident>,
    }

    impl<'ast> Visit<'ast> for GetVisitor {
        fn visit_expr(&mut self, node: &'ast Expr) {
            // Looking for pattern: self.field.get()
            // Expr::MethodCall with method == "get" and receiver == self.field
            if let Expr::MethodCall(method_call) = node {
                if method_call.method == "get" {
                    if let Expr::Field(field) = &*method_call.receiver {
                        if let Expr::Path(path) = &*field.base {
                            if path.path.is_ident("self") {
                                if let syn::Member::Named(ident) = &field.member {
                                    self.fields.push(ident.clone());
                                }
                            }
                        }
                    }
                }
            }

            // Also search inside macros (e.g., format!(...))
            if let Expr::Macro(expr_macro) = node {
                let tokens = expr_macro.mac.tokens.clone();
                extract_fields_from_tokens(tokens, &mut self.fields);
            }

            syn::visit::visit_expr(self, node);
        }
    }

    let mut visitor = GetVisitor { fields: Vec::new() };
    visitor.visit_expr(expr);
    visitor.fields
}

// Helper function to extract self.field.get() from macro token streams
fn extract_fields_from_tokens(tokens: proc_macro2::TokenStream, fields: &mut Vec<Ident>) {
    let mut token_iter = tokens.into_iter().peekable();

    while let Some(token) = token_iter.next() {
        match token {
            TokenTree::Ident(ident) if ident == "self" => {
                // Check if next is '.'
                if let Some(TokenTree::Punct(punct)) = token_iter.peek() {
                    if punct.as_char() == '.' {
                        token_iter.next(); // consume '.'

                        // Get field name
                        if let Some(TokenTree::Ident(field_name)) = token_iter.peek() {
                            let field_name = field_name.clone();
                            token_iter.next(); // consume field name

                            // Check for another '.'
                            if let Some(TokenTree::Punct(punct)) = token_iter.peek() {
                                if punct.as_char() == '.' {
                                    token_iter.next(); // consume '.'

                                    // Check for 'get'
                                    if let Some(TokenTree::Ident(method)) = token_iter.peek() {
                                        if method == "get" {
                                            fields.push(field_name);
                                            token_iter.next(); // consume 'get'
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            TokenTree::Group(group) => {
                // Recursively search inside groups (parentheses, brackets, etc.)
                extract_fields_from_tokens(group.stream(), fields);
            }
            _ => {}
        }
    }
}
