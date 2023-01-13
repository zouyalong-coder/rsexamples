//! 过程宏有三种：
//! - 属性宏
//! - 函数宏(function-like)
//! - 语法扩展宏（derive）
//! 实现过程：
//! - Cargo.toml 中添加 proc-macro = true，这样该库不可以包含其它代码

extern crate proc_macro;
use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    fold::{self, Fold},
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    DeriveInput, Expr, Ident, Local, Pat, Result, Stmt, Token,
};

struct Args {
    vars: HashSet<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        // parses a,b,c, or a,b,c where a,b and c are Indent
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

impl Fold for Args {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
            // for changing assignment like a=5
            Expr::Assign(e) => {
                // check should print
                if self.should_print_expr(&e.left) {
                    self.assign_and_print(*e.left, &e.eq_token, *e.right)
                } else {
                    // continue with default travesal using default methods
                    Expr::Assign(fold::fold_expr_assign(self, e))
                }
            }
            // for changing assigment and operation like a+=1
            Expr::AssignOp(e) => {
                // check should print
                if self.should_print_expr(&e.left) {
                    self.assign_and_print(*e.left, &e.op, *e.right)
                } else {
                    // continue with default behaviour
                    Expr::AssignOp(fold::fold_expr_assign_op(self, e))
                }
            }
            // continue with default behaviour for rest of expressions
            _ => fold::fold_expr(self, e),
        }
    }

    // for let statements like let d=9
    fn fold_stmt(&mut self, s: Stmt) -> Stmt {
        match s {
            Stmt::Local(s) => {
                if s.init.is_some() && self.should_print_pat(&s.pat) {
                    self.let_and_print(s)
                } else {
                    Stmt::Local(fold::fold_local(self, s))
                }
            }
            _ => fold::fold_stmt(self, s),
        }
    }
}

impl Args {
    fn should_print_expr(&self, e: &Expr) -> bool {
        match *e {
            Expr::Path(ref e) => {
                // variable shouldn't start wiht ::
                if e.path.leading_colon.is_some() {
                    false
                // should be a single variable like `x=8` not n::x=0
                } else if e.path.segments.len() != 1 {
                    false
                } else {
                    // get the first part
                    let first = e.path.segments.first().unwrap();
                    // check if the variable name is in the Args.vars hashset
                    self.vars.contains(&first.ident) && first.arguments.is_empty()
                }
            }
            _ => false,
        }
    }

    // used for checking if to print let i=0 etc or not
    fn should_print_pat(&self, p: &Pat) -> bool {
        match p {
            // check if variable name is present in set
            Pat::Ident(ref p) => self.vars.contains(&p.ident),
            _ => false,
        }
    }

    // manipulate tree to insert print statement
    fn assign_and_print(&mut self, left: Expr, op: &dyn ToTokens, right: Expr) -> Expr {
        // recurive call on right of the assigment statement
        let right = fold::fold_expr(self, right);
        // returning manipulated sub-tree
        parse_quote!({
            #left #op #right;
            println!(concat!(stringify!(#left), " = {:?}"), #left);
        })
    }

    // manipulating let statement
    fn let_and_print(&mut self, local: Local) -> Stmt {
        let Local { pat, init, .. } = local;
        let init = self.fold_expr(*init.unwrap().1);
        // get the variable name of assigned variable
        let ident = match pat {
            Pat::Ident(ref p) => &p.ident,
            _ => unreachable!(),
        };
        // new sub tree
        parse_quote! {
            let #pat = {
                #[allow(unused_mut)]
                let #pat = #init;
                println!(concat!(stringify!(#ident), " = {:?}"), #ident);
                #ident
            };
        }
    }
}

/// 属性宏。
/// using proc_macro_attribute to declare an attribute like procedural macro
#[proc_macro_attribute]
pub fn trace_vars(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as syn::ItemFn);
    // parse the arguments. 调用 Parse trait 的 parse 方法
    let mut args = parse_macro_input!(metadata as Args);
    let output = args.fold_item_fn(input_fn);
    TokenStream::from(quote! {#output})
}

#[proc_macro_attribute]
// _metadata is argument provided to macro call and _input is code to which attribute like macro attaches
pub fn my_macro(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    TokenStream::from(quote! {struct H{}})
}
