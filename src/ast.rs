// AST types for parsing and generation.
// Represents syntax:
//   <MNE> => [ (<OpCode>, <AddrMode>, <Bytes>, <Cycles>), ...]
// EBNF:
// Ops     ::= Op[, Op]*
// Op      ::= Mnemonic '=>' '[' OpTuple[, OpTuple]* ']'
// OpTuple ::= '(' <OpCode>, <AddrMode>, <Bytes, <Cycles> ')'
//
use quote::*;
use syn::{parse::*, punctuated::*, *};

#[derive(Clone)]
pub(crate) struct Ops {
    pub(crate) ops: Punctuated<Op, Token![,]>,
}

#[derive(Clone)]
pub(crate) struct Op {
    pub(crate) mnemonic: Expr,
    pub(crate) ops: Punctuated<OpTuple, Token![,]>,
}

#[derive(Clone)]
pub(crate) struct OpTuple {
    pub(crate) opcode: Expr,
    pub(crate) addr_mode: Ident,
    pub(crate) num_bytes: Expr,
    pub(crate) num_cycles: Expr,
}

impl Parse for Ops {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            ops: input.parse_terminated(Op::parse, Token![,])?,
        })
    }
}

impl Parse for Op {
    fn parse(input: ParseStream) -> Result<Self> {
        let mnemonic: Expr = input.parse()?;
        let _assign_token: Token![=>] = input.parse()?;
        let content;
        let _bracket_token: token::Bracket = bracketed!(content in input);
        let ops: Punctuated<OpTuple, Token![,]> =
            content.parse_terminated(OpTuple::parse, Token![,])?;

        Ok(Self { mnemonic, ops })
    }
}

impl Parse for OpTuple {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let opcode: Expr = content.parse()?;
        let _: token::Comma = content.parse()?;
        let addr_mode: Ident = content.parse()?;
        let _: token::Comma = content.parse()?;
        let num_bytes: Expr = content.parse()?;
        let _: token::Comma = content.parse()?;
        let num_cycles: Expr = content.parse()?;
        /*
        let t: ExprTuple = input.parse()?;
        if t.elems.len() != 4 {
            return Err(Error::new(
                input.span(),
                "expected (<OpCode>, <AddrMode>, <NumBytes>, <NumCycles>)",
            ));
        }
        t.elems[1].
        let amode = match t.elems[1] {
            ExprLit { lit, .. } =>  lit,
            _ => return Err(Error::new(input.span(), "expected identifier")),
        }
        Ok(Self {
            opcode: t.elems[0].clone(),
            addr_mode: t.elems[1].clone(),
            num_bytes: t.elems[2].clone(),
            num_cycles: t.elems[3].clone(),
        })
        */
        Ok(Self {
            opcode,
            addr_mode,
            num_bytes,
            num_cycles,
        })
    }
}

#[derive(Clone)]
pub(crate) struct OpGen {
    pub(crate) mnemonic: Expr,
    pub(crate) opcode: Expr,
    pub(crate) addr_mode: Ident,
    pub(crate) num_bytes: Expr,
    pub(crate) num_cycles: Expr,
}

impl OpGen {
    pub fn from(op: &Op) -> Vec<OpGen> {
        op.ops
            .iter()
            .map(|o| Self {
                mnemonic: op.mnemonic.clone(),
                opcode: o.opcode.clone(),
                addr_mode: o.addr_mode.clone(),
                num_bytes: o.num_bytes.clone(),
                num_cycles: o.num_cycles.clone(),
            })
            .collect::<Vec<_>>()
    }
}

impl ToTokens for OpGen {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let OpGen {
            mnemonic,
            opcode,
            addr_mode,
            num_bytes,
            num_cycles,
        } = self;
        let gen = quote! {
            Op {
                mnemonic: Mnemonic::#mnemonic,
                opcode: #opcode,
                addr_mode: AddrMode::#addr_mode,
                num_bytes: #num_bytes,
                num_cycles: #num_cycles,
            }
        };
        tokens.extend(gen);
    }
}
