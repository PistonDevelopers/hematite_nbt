//! This crate contains a compiler plugin to generate Named Binary Tag format
//! serialization code for custom structs. It can be used as follows:
//!
//! ```ignore
//! #![feature(plugin, custom_derive)]
//! #![plugin(nbt_macros)]
//! 
//! extern crate nbt;
//! 
//! use nbt::serialize::{NbtFmt, to_writer};
//! 
//! #[derive(NbtFmt)]
//! struct MyMob {
//!     name: String,
//!     health: i8
//! }
//!
//! fn main() {
//!     let mut bytes = Vec::new();
//!     let mob = MyMob { name: "Dr. Evil".to_string(), health: 240 };
//!
//!     to_writer(&mut bytes, mob).unwrap();
//! }
//! ```
//! 
//! The custom `derive(NbtFmt)` will generate code equivalent to the following:
//! 
//! ```ignore
//! impl NbtFmt for MyMob {
//!     fn to_bare_nbt<W>(&self, dst: &mut W) -> nbt::Result<()>
//!        where W: std::io::Write
//!     {
//!         try!(self.name.to_nbt(dst, "name"));
//!         try!(self.health.to_nbt(dst, "health"));
//! 
//!         close_nbt(dst)
//!     }
//! }
//! ```
//! 
//! Which will work so long as the fields of the struct have `NbtFmt`
//! implementations of their own.

#![feature(plugin_registrar, rustc_private, box_syntax)]

extern crate rustc;
extern crate syntax;

use syntax::ast;
use syntax::ast::{Expr, MetaItem, Mutability};
use syntax::attr::AttrMetaMethods;
use syntax::codemap::Span;
use syntax::ext::base::{Annotatable, ExtCtxt, MultiDecorator};
use syntax::ext::build::AstBuilder;
use syntax::ext::deriving::generic::*;
use syntax::ext::deriving::generic::ty::*;
use syntax::parse::token::{get_ident, intern, intern_and_get_ident, InternedString};
use syntax::ptr::P;


#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut rustc::plugin::Registry) {
    reg.register_syntax_extension(intern("derive_NbtFmt"),
                                  MultiDecorator(box expand_derive_nbtfmt));
}

macro_rules! pathvec {
    ($($x:ident)::+) => (
        vec![ $( stringify!($x) ),+ ]
    )
}

macro_rules! path {
    ($($x:tt)*) => (
        Path::new( pathvec!( $($x)* ) )
    )
}

macro_rules! pathexpr {
    ($cx:ident, $span:ident, $($x:ident)::+) => (
        $cx.expr_path($cx.path_global($span,
            vec![ $( $cx.ident_of( stringify!($x) ) ),+ ]))
    )
}

/// Expands uses of `#[derive(NbtFmt)]` into `NbtFmt` implementations.
pub fn expand_derive_nbtfmt(cx: &mut ExtCtxt, span: Span, meta_item: &MetaItem,
                            item: &Annotatable,
                            push: &mut FnMut(Annotatable))
{
    // First, check that we're deriving for a struct.
    if let &Annotatable::Item(ref it) = item {
        match it.node {
            ast::Item_::ItemStruct(_, _) => (), // Allow structs only.
            ast::Item_::ItemEnum(_, _) => {
                cx.span_err(span, "`NbtFmt` cannot yet be derived for enums.");
                return;
            },
            _ => {
                cx.span_err(span, "#[derive(NbtFmt)] only allowed on structs.");
                return;
            }
        }
    } else {
        cx.span_err(span, "#[derive(NbtFmt)] only allowed on structs.");
        return;
    }

    let trait_def = TraitDef {
        span: span,
        attributes: Vec::new(),
        path: path!(nbt::serialize::NbtFmt),
        additional_bounds: Vec::new(),
        generics: LifetimeBounds::empty(),
        methods: vec![
            MethodDef {
                name: "to_bare_nbt",
                generics: LifetimeBounds {
                    lifetimes: Vec::new(),
                    // This adds a <__W: std::io::Writer> generic to the method.
                    bounds: vec![("__W", vec![path!(std::io::Write)])],
                },
                // Pass the immutable borrowed self argument, `&self`.
                explicit_self: borrowed_explicit_self(),
                // Pass a single argument of type `&mut __W`.
                args: vec![Ptr(box Literal(Path::new_local("__W")),
                           Borrowed(None, Mutability::MutMutable))],
                // Return a `Result<(), nbt::Error>`.
                ret_ty: Literal(Path {
                    path: pathvec!(std::result::Result),
                    lifetime: None,
                    params: vec![box Tuple(Vec::new()), box Literal(path!(nbt::Error))],
                    global: true }),
                attributes: Vec::new(), // FIXME: Benchmark adding #[inline].
                is_unsafe: false,
                combine_substructure: combine_substructure(box cs_to_bare_nbt),
            },
            MethodDef {
                name: "read_bare_nbt",
                generics: LifetimeBounds {
                    lifetimes: Vec::new(),
                    // This adds a <__R: std::io::Read> generic to the method.
                    bounds: vec![("__R", vec![path!(std::io::Read)])],
                },
                // No self argument.
                explicit_self: None,
                // Pass a single argument of type `&mut __R`.
                args: vec![Ptr(box Literal(Path::new_local("__R")),
                               Borrowed(None, Mutability::MutMutable))],
                // Return a `Result<Self, nbt::Error>`.
                ret_ty: Literal(Path {
                    path: pathvec!(std::result::Result),
                    lifetime: None,
                    params: vec![box Self_, box Literal(path!(nbt::Error))],
                    global: true }),
                attributes: Vec::new(), // FIXME: Benchmark adding #[inline].
                is_unsafe: false,
                combine_substructure: combine_substructure(box cs_read_bare_nbt)
            },
        ],
        associated_types: vec![(cx.ident_of("Into"), Self_)],
    };

    trait_def.expand(cx, meta_item, item, push)
}

fn cs_to_bare_nbt(cx: &mut ExtCtxt, trait_span: Span, substr: &Substructure) -> P<Expr> {
    // Retrieve the argument passed to the to_bare_nbt function, i.e. the
    // `dst: &mut __W` bit. Since the method is already defined, there's no
    // reason for this to fail, so we call `cx.span_bug` indicating a compiler
    // error.
    let dst_expr = match (substr.nonself_args.len(), substr.nonself_args.get(0)) {
        (1, Some(dst)) => dst,
        _ => cx.span_bug(trait_span, "incorrect number of method arguments in `derive(NbtFmt)`")
    };

    // Define a closure that iterates over the fields in the struct and
    // generates a statement akin to `try!(NbtFmt::to_nbt(self, field));`.
    let call_nbt_fmt = |span, struct_self, name| {

        // Create expressions for the path to the `to_nbt` method and `&self`.
        let nbt_fmt_path = pathexpr!(cx, span, nbt::serialize::NbtFmt::to_nbt);
        let self_arg = cx.expr_addr_of(span, struct_self);

        // Create a string literal expression for the field's identifier.
        let name_expr = cx.expr_str(span, name);

        // Create a call expression, using the function path (nbt_fmt_path)
        // and `&self, dst, "<field>"` as arguments.
        let fmt_call = cx.expr_call(span, nbt_fmt_path,
                                    vec!(self_arg, dst_expr.clone(), name_expr));

        // Wrap the call in a try! macro.
        let try_fmt_call = cx.expr_try(span, fmt_call);

        // Add a semicolon.
        cx.stmt_expr(try_fmt_call)
    };

    match *substr.fields {
        Struct(ref fields) => {   
            // Unit structs are kind of irrelevant for NBT, so throw an error
            // if someone tries to derive(NbtFmt) over one.
            if fields.is_empty() {
                cx.span_err(trait_span, "`NbtFmt` has no meaning for unit structs.");
                return cx.expr_fail(trait_span, InternedString::new(""));
            }

            let mut stmts = Vec::with_capacity(fields.len());

            // Handle tuple structs, i.e. `struct Test(i8, i8, String);`
            // by using `"field0"`, `"field1"`, etc. as names.
            if fields[0].name.is_none() {
                for (i, field) in fields.iter().enumerate() {
                    // Just in case there is a compiler bug and we get an
                    // *named* field in the middle of a tuple struct, call `cx.bug`.
                    if let Some(_) = field.name {
                        cx.span_bug(trait_span, "named field in tuple struct")
                    }

                    // Make an interned "fieldX" string.
                    let name_ = intern_and_get_ident(&format!("field{}", i));

                    // Apply the closure on this named field.
                    stmts.push(call_nbt_fmt(field.span, field.self_.clone(), name_));
                }
            } else {
                for &FieldInfo { ref self_, span, name, attrs, .. } in fields {
                    // Just in case there is a compiler bug and we get an
                    // unnamed field in the middle of a struct, call `cx.bug`.
                    if let None = name {
                        cx.span_bug(trait_span, "unnamed field in named struct")
                    }

                    let mut name_ = get_ident(name.unwrap());

                    // Optionally change the name of the field when the
                    // #[nbt_field = "fieldX"] attribute is present on the item.
                    for ref attr in attrs {
                        if attr.check_name("nbt_field") {
                            if let Some(s) = attr.value_str() {
                                name_ = s;
                            } else {
                                cx.span_err(span, "`#[nbt_field]` requires a &str value.");
                                return cx.expr_fail(trait_span, InternedString::new(""));
                            }
                            break;
                        }
                    }

                    // Apply the closure on this named field.
                    stmts.push(call_nbt_fmt(span, self_.clone(), name_));
                }
            }

            // Creates a `close_nbt(dst)` expression to add to the end of
            // the block.
            let close_nbt_path = pathexpr!(cx, trait_span, nbt::serialize::close_nbt);
            let close = cx.expr_call(trait_span, close_nbt_path, vec![dst_expr.clone()]);

            cx.expr_block(cx.block(trait_span, stmts, Some(close)))
        },
        _ => cx.span_bug(trait_span, "impossible substructure in `#[derive(NbtFmt)]`")
    }
}

fn cs_read_bare_nbt(cx: &mut ExtCtxt, trait_span: Span, substr: &Substructure) -> P<Expr> {
    match *substr.fields {
        StaticStruct(_, _) => cx.expr_unreachable(trait_span),
        _ => cx.span_bug(trait_span, "impossible substructure in `#[derive(NbtFmt)]`")
    }
}
