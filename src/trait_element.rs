use ion::Ion;
use molecule::Molecule;
use trait_properties::Properties;
use types::*;

use std::hash::Hash;

pub trait Element: Properties + Hash {
    /// Get the charge of the current Element
    fn get_charge(&self) -> Option<AtomCharge>;

    /// Get the molecule associated with the current Element
    fn get_molecule(self) -> Option<Molecule>;

    /// Get the ion associated with the current Element
    fn get_ion(self) -> Option<Ion>;
}
