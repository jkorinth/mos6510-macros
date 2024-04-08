use proc_macro2::TokenStream;
use quote::*; pub fn gen_types() -> TokenStream { quote! {
        #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
        pub struct Cycles(u32);

        impl core::ops::Deref for Cycles {
            type Target = u32;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
        pub struct Bytes(usize);

        impl core::ops::Deref for Bytes {
            type Target = usize;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
        pub struct OpCode(pub u8);

        impl core::ops::Deref for OpCode {
            type Target = u8;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Eq, PartialEq, Debug, Hash, Clone)]
        pub struct Op {
            pub(crate) mnemonic: Mnemonic,
            pub(crate) addr_mode: AddrMode,
            pub(crate) opcode: OpCode,
            pub(crate) num_bytes: Bytes,
            pub(crate) num_cycles: Cycles,
        }

        impl Op {
            #[inline]
            pub fn mnemonic(&self) -> Mnemonic {
                self.mnemonic
            }

            #[inline]
            pub fn addr_mode(&self) -> AddrMode {
                self.addr_mode
            }

            #[inline]
            pub fn opcode(&self) -> &OpCode {
                &self.opcode
            }

            #[inline]
            pub fn num_bytes(&self) -> Bytes {
                self.num_bytes
            }

            #[inline]
            pub fn num_cycles(&self) -> Cycles {
                self.num_cycles
            }
        }

        #[derive(Clone, Debug)]
        pub struct Instruction {
            pub mnemonic: Mnemonic,
            pub ops: Vec<Op>,
        }

        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub enum Group {
            Jump,
            Branch,
            Compare,
            Bitwise,
            Flag,
            Arithmetic,
            Memory,
            Register,
            Stack,
            Other,
        }
    }
}
