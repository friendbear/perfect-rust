mod assosiation;
mod generics;
use crate::assosiation::try_out as ass;
use crate::generics::try_out as gen;

fn main() {
    gen::use_generics_method();
    ass::use_association_method();
    gen::use_service_metthod();
}
