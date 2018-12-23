use electron::ELECTRON;
use molecule::Molecule;
use namings::*;
use trait_element::Element;
use trait_properties::Properties;
use types::*;

#[derive(Debug, Eq, Clone)]
/// An `Ion`
pub struct Ion {
    /// The molecule of this ion
    pub molecule: Molecule,

    /// The charge of this ion
    pub charge: Option<AtomCharge>,
}

impl Ion {
    /// Convert a string representation of an `Ion` into one
    pub fn from_string(symbol: &str) -> Option<Ion> {
        let mut molecule = None;
        let mut charge = 0;
        let mut is_negative = false;
        let mut is_positive = false;

        let mut token = String::new();
        let mut set_charge = false;

        for c in symbol.chars() {
            if set_charge {
                if c == '-' {
                    is_negative = true;
                    continue;
                }

                if c == '+' {
                    is_positive = true;
                    continue;
                }

                if !is_number!(c) {
                    panic!("Expected charge, but got {}", c);
                }

                charge *= 10;
                charge += to_number!(c) as i8;
                continue;
            }

            if c == ';' {
                // Electron
                if token == "e" {
                    break;
                }

                molecule = Molecule::from_string(&token);
                token = String::new();
                set_charge = true;
                continue;
            }

            token.push(c);
        }

        // It's just a molecule
        if !token.is_empty() && !set_charge {
            // Electron
            if token == "e" {
                return Some(ELECTRON.clone());
            }

            molecule = Molecule::from_string(&token);
        }

        if is_negative {
            // assume - to mean -1
            if charge == 0 {
                charge = -1;
            } else {
                charge *= -1;
            }
        }

        if is_positive && charge == 0 {
            // assume + to mean +1
            charge = 1;
        }

        let charge_option = if set_charge {
            Some(AtomCharge::from(charge))
        } else {
            None
        };

        if let Some(molecule) = molecule {
            Some(Ion {
                molecule,
                charge: charge_option,
            })
        } else {
            None
        }
    }

    /// Convert a `Molecule` into an `Ion`
    pub fn from_molecule(molecule: Molecule) -> Ion {
        Ion {
            molecule,
            charge: None, // Will be calculated later
        }
    }

    /// Calculate the charge of this `Ion`
    pub fn calculate_charge(&self) -> Option<AtomCharge> {
        let mut charge = AtomCharge::from(0);

        // If a molecule is diatomic, charge is 0
        if self.molecule.is_diatomic() {
            return Some(charge);
        }

        for molecule_compound in &self.molecule.compounds {
            if let Some(atom_charge) = molecule_compound.atom.charge_by_group() {
                let mol_charge = AtomCharge::from(
                    // NOTE: Can't multiply u8 and i8, so they need to be casted first
                    (molecule_compound.amount as AtomCharge_type)
                        * (atom_charge.0 as AtomCharge_type),
                );

                charge += mol_charge;
            }
        }

        // HACK: This seems to be correct for now
        charge %= 8;

        Some(charge)
    }
}

impl Element for Ion {
    // Make sure that the charge is calculated when required
    fn get_charge(&self) -> Option<AtomCharge> {
        if let Some(ref charge) = self.charge {
            Some(charge.clone())
        } else {
            self.calculate_charge()
        }
    }

    fn get_molecule(self) -> Option<Molecule> {
        Some(self.molecule)
    }

    fn get_ion(self) -> Option<Ion> {
        Some(self)
    }
}

impl Properties for Ion {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        symbol += &self.molecule.symbol();

        if let Some(charge) = self.get_charge() {
            if charge != AtomCharge::from(0) {
                symbol += &ion_superscript(&charge);
            }
        }

        symbol
    }

    fn name(&self) -> String {
        let mut name = String::new();

        name += &self.molecule.name();

        if let Some(charge) = self.get_charge() {
            if charge != AtomCharge::from(0) {
                name += "(";
                name += &number_to_roman(charge.0);
                name += ")";
            }
        }

        name
    }

    fn mass(&self) -> AtomMass {
        self.molecule.mass()
    }

    fn is_diatomic(&self) -> bool {
        self.molecule.is_diatomic()
    }
}

impl PartialEq for Ion {
    fn eq(&self, rhs: &Self) -> bool {
        self.molecule == rhs.molecule && self.get_charge() == rhs.get_charge()
    }
}

use std::hash::*;

impl Hash for Ion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.molecule.hash(state);
        self.get_charge().hash(state);
    }
}
