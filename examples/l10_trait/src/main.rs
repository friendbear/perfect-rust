mod generics;
mod assosiation;
use crate::generics::try_out as gen;
use crate::assosiation::try_out as ass;

fn main() {
    gen::use_generics_method();
    ass::use_association_method();
}
