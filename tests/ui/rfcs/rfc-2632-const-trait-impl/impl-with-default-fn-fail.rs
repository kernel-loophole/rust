#![allow(incomplete_features)]
#![feature(const_trait_impl, effects)]

#[const_trait]
trait Tr {
    fn req(&self);

    fn default() {}
}

struct S;

impl const Tr for u16 {
    fn default() {}
} //~^^ ERROR not all trait items implemented


fn main() {}
