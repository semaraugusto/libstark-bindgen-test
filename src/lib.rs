#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("../bindings.rs");
mod utils;
use itertools::Itertools;

#[derive(Debug)]
struct RustWitness {
    assignments: Vec<Vec<u64>>,
    permutations: Vec<usize>
}
impl RustWitness {
    fn new(serialized_witness: &str, ncols: usize) -> Self {
        let (assignments_str, permutations_str) = serialized_witness.split_terminator("\n\n").next_tuple().unwrap();
        let assignments = utils::parse_assignment_str(assignments_str, ncols);
        let permutations = utils::parse_permutation_str(permutations_str);
        Self {
            assignments,
            permutations
        }

    }
}

#[cfg(test)]
mod tests {
    use super::RustWitness;
    use super::root::RustyLibstark::*;
    use std::ffi::CStr;
    
    #[test]
    fn test_hello_int() {
        let hello = unsafe { hello_from_c() };
        assert_eq!(hello, 42)
    }
    #[test]
    fn test_get_string() {
        let hello = unsafe { gen_hello() };
        let str = unsafe {CStr::from_ptr(hello)};
        assert_eq!("hello", str.to_str().unwrap());
    }
    #[test]
    fn test_gen_witness() {
        let serialized_witness_ptr = unsafe { gen_witness() };
        let serialized_witness_cstr = unsafe {CStr::from_ptr(serialized_witness_ptr)};
        let witness = RustWitness::new(serialized_witness_cstr.to_str().unwrap(), 3);
        assert_eq!(witness.assignments.len(), 8);
        assert_eq!(witness.assignments[0].len(), 3);
        assert_eq!(witness.permutations.len(), 8);
    }
}
