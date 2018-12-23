use atom::Atom;
use ion::Ion;
use namings::*;
use trait_element::Element;
use trait_properties::Properties;
use types::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
/// A molecule
pub struct Molecule {
    /// The compounds it contains
    pub compounds: Vec<MoleculeCompound>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
/// A compound of a molecule
pub struct MoleculeCompound {
    /// The atom it uses
    pub atom: Atom,

    /// The amount
    pub amount: u8,
}

impl Molecule {
    /// Convert a string representation of a molecule into one
    /// TODO: Parse parentheses, e.g.  Ca3(PO4)2
    pub fn from_string(string: &str) -> Option<Molecule> {
        let mut compounds = vec![];

        let mut token = String::new();

        for c in string.chars() {
            // Ignore whitespace
            if is_whitespace!(c) || is_separator!(c) {
                continue;
            }

            if is_upper!(c) && !token.is_empty() {
                let compound = MoleculeCompound::from_string(&token).unwrap();

                compounds.push(compound);
                token = String::new();
            }

            token.push(c);
        }

        // If some tokens remain, convert it into a compound
        if !token.is_empty() {
            if let Some(compound) = MoleculeCompound::from_string(&token) {
                compounds.push(compound);
            }
        }

        if !compounds.is_empty() {
            Some(Molecule { compounds })
        } else {
            None
        }
    }
}

impl MoleculeCompound {
    /// Takes a symbol string representing a MoleculeCompound, and turns it into one
    pub fn from_string(string: &str) -> Option<MoleculeCompound> {
        let mut amount = 0;

        let mut token = String::new();

        for c in string.chars() {
            if is_letter!(c) {
                token.push(c);
            } else if is_number!(c) {
                amount *= 10;
                amount += to_number!(c);
            } else {
                panic!(
                    "Invalid character '{}' in string \"{}\" for MoleculeCompound",
                    c, string
                );
            }
        }

        // If no amount given, assume 1
        if amount == 0 {
            amount = 1;
        }

        if let Some(atom) = Atom::from_string(&token) {
            Some(MoleculeCompound { atom, amount })
        } else {
            panic!("Failed to find Atom for {}", &token);
        }
    }

    /// Converts an Atom into a MoleculeCompound, taking care of diatomic ones
    pub fn from_atom(atom: Atom) -> MoleculeCompound {
        let amount = if atom.diatomic { 2 } else { 1 };

        MoleculeCompound { atom, amount }
    }
}

impl Properties for Molecule {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        for compound in &self.compounds {
            symbol += &compound.symbol();
        }

        symbol
    }

    fn name(&self) -> String {
        let mut name = String::new();

        // TODO: Add special cases
        // NOTE: https://www.youtube.com/watch?v=mlRhLicNo8Q
        for compound in &self.compounds {
            name += &compound.name();
        }

        name
    }

    fn mass(&self) -> AtomMass {
        let mut mass = AtomMass::from(0.0);

        for compound in &self.compounds {
            mass += compound.mass();
        }

        mass
    }

    fn is_diatomic(&self) -> bool {
        self.compounds.len() == 1
            && self.compounds[0].amount == 2
            && self.compounds[0].atom.diatomic
    }
}

impl Properties for MoleculeCompound {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        symbol += &self.atom.symbol();

        if self.amount > 1 {
            symbol += &subscript(self.amount);
        }

        symbol
    }

    fn name(&self) -> String {
        let mut name = String::new();

        if self.amount > 1 {
            name += &number_to_greek(self.amount);
        }

        name += &self.atom.name();

        name
    }

    fn mass(&self) -> AtomMass {
        self.atom.mass.clone() * (AtomMass_type::from(self.amount))
    }

    fn is_diatomic(&self) -> bool {
        false
    }
}

impl Element for Molecule {
    fn get_charge(&self) -> Option<AtomCharge> {
        Some(AtomCharge::from(0))
    }

    fn get_molecule(self) -> Option<Molecule> {
        Some(self)
    }

    fn get_ion(self) -> Option<Ion> {
        Some(Ion::from_molecule(self.clone()))
    }
}
