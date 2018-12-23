use types::*;

use std::fmt;

pub trait Properties: Eq + PartialEq + Clone + fmt::Debug {
    /// Get the symbol of the current item
    fn symbol(&self) -> String;

    /// Get the name of the current item
    fn name(&self) -> String;

    /// Get the mass of the current item
    fn mass(&self) -> AtomMass;

    /// Check if the item is a valid diatomic molecule
    fn is_diatomic(&self) -> bool;

    /// Stringify the current item (default: self.symbol())
    fn stringify(&self) -> String {
        self.symbol()
    }
}
