// Taken from Rust source code: src/libsyntax/ext/concat.rs
use std::io::Write;
use std::rc::Rc;
use syntax::ast;
use syntax::codemap;
use syntax::ext::base;
use syntax::ext::build::AstBuilder;

pub fn expand_syntax_ext(cx: &mut base::ExtCtxt,
                         sp: codemap::Span,
                         tts: &[ast::TokenTree])
                         -> Box<base::MacResult+'static> {
    let es = match base::get_exprs_from_tts(cx, sp, tts) {
        Some(e) => e,
        None => return base::DummyResult::expr(sp)
    };
    let mut accumulator = Vec::new();
    for e in es {
        match e.node {
            ast::ExprLit(ref lit) => {
                match lit.node {
                    ast::LitStr(ref s, _) |
                    ast::LitFloat(ref s, _) |
                    ast::LitFloatUnsuffixed(ref s) => {
                        write!(accumulator, "{}", s).unwrap();
                    }
                    ast::LitChar(c) => {
                        write!(accumulator, "{}", c).unwrap();
                    }
                    ast::LitInt(i, ast::UnsignedIntLit(_)) |
                    ast::LitInt(i, ast::SignedIntLit(_, ast::Plus)) |
                    ast::LitInt(i, ast::UnsuffixedIntLit(ast::Plus)) => {
                        write!(accumulator, "{}", i).unwrap();
                    }
                    ast::LitInt(i, ast::SignedIntLit(_, ast::Minus)) |
                    ast::LitInt(i, ast::UnsuffixedIntLit(ast::Minus)) => {
                        write!(accumulator, "-{}", i).unwrap();
                    }
                    ast::LitBool(b) => {
                        write!(accumulator, "{}", b).unwrap();
                    }
                    ast::LitByte(b) => {
                        accumulator.push(b);
                    }
                    ast::LitBinary(ref bytes) => {
                        accumulator.push_all(bytes);
                    }
                }
            }
            _ => {
                cx.span_err(e.span, "expected a literal");
            }
        }
    }
    base::MacEager::expr(cx.expr_lit(
        sp,
        ast::LitBinary(Rc::new(accumulator))
    ))
}
