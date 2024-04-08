use proc_macro::TokenStream;
use quote::*;
use syn::*;

mod ast;
mod types;
use ast::*;

use std::collections::HashSet;

#[proc_macro]
pub fn instructions(_input: TokenStream) -> TokenStream {
    let Ops { ops } = parse_macro_input!(_input as Ops);
    let mnemonics = ops.iter().map(|op| &op.mnemonic);
    let amodes = ops
        .iter()
        .map(|op| op.ops.iter().map(|o| &o.addr_mode))
        .flatten()
        .collect::<HashSet<_>>();
    let addrmodes = amodes.iter();
    let ops = ops
        .iter()
        .map(|op| OpGen::from(op))
        .flatten()
        .map(|o| (o.opcode.clone(), o.clone()));

    let keys = ops.clone().map(|p| p.0);
    let vals = ops.map(|p| p.1);

    let gen_types = types::gen_types();

    let gen = quote! {
     #gen_types

     #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
     pub enum Mnemonic {
         #( #mnemonics ),*
     }

     #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
     pub enum AddrMode {
        #( #addrmodes ),*
     }

     pub static OPS: Lazy<HashMap<OpCode, Op>> = Lazy::new(|| {
         let mut isa: HashMap<OpCode, Op> = HashMap::new();
         #( isa.insert(#keys, #vals); )*
         isa
     });

    };
    gen.into()
}
