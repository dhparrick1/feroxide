use data_atoms::ALL_ATOMS;
use trait_properties::Properties;
use types::*;

use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
/// An atom
pub struct Atom {
    /// Atom Number (Hydrogen: 1)
    pub number: AtomNumber,

    /// Atom Group (Hydrogen: 1)
    pub group: AtomGroup,

    /// Atom symbol (Hydrogen: H)
    pub symbol: &'static str,

    /// Atom name (Hydrogen: hydrogen)
    pub name: &'static str,

    /// Atom mass (Hydrogen: 1.008)
    pub mass: AtomMass,

    // Diatomic? (Hydrogen: true)
    pub diatomic: bool,
}

impl Atom {
    /// Convert a string representation to an `Atom`
    pub fn from_string(symbol: &str) -> Option<Atom> {
        // TODO: Make this more efficient
        for atom in ALL_ATOMS {
            if atom.symbol == symbol {
                return Some(atom.clone());
            }
        }

        None
    }

    /// Get the charge an atom has based on its group
    pub fn charge_by_group(&self) -> Option<AtomCharge> {
        let group = self.group.0;
        let number = self.number.0;

        if group == 1 {
            Some(AtomCharge::from(1))
        } else if group == 2 {
            Some(AtomCharge::from(2))
        } else if group == 15 && number <= 15 {
            Some(AtomCharge::from(-3))
        } else if group == 16 && number <= 34 {
            Some(AtomCharge::from(-2))
        } else if group == 17 && number <= 53 {
            Some(AtomCharge::from(-1))
        } else if group == 18 {
            Some(AtomCharge::from(0))
        } else {
            None
        }
    }
}

impl Eq for Atom {}

impl PartialEq for Atom {
    /// Two `Atom`s are equal when their atom numbers are equal
    fn eq(&self, rhs: &Atom) -> bool {
        self.number == rhs.number
    }
}

impl Hash for Atom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Only the `AtomNumber` determines the hash
        self.number.0.hash(state)
    }
}

impl Properties for Atom {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }

    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn mass(&self) -> AtomMass {
        self.mass.clone()
    }

    fn is_diatomic(&self) -> bool {
        // NOTE: Atom's can never have two atoms, so is never a valid diatomic molecule
        false
    }
}
