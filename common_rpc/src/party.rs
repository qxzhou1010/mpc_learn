use std::cmp::{Eq, PartialEq};
use std::hash::Hash;
pub trait Party: PartialEq + Eq + Hash {
    fn get_party_id(&self) -> u32;
    fn get_party_name(&self) -> String;
    
    fn compare_to(&self, otherParty: &Self) -> bool {
        // Id_0 > Id_1 => false
        // Id_1 > Id_0 => true
        self.get_party_id() > otherParty.get_party_id()
    }
}