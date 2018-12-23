use data_sep::*;
use ion::Ion;
use math::gcd;
use reaction::{ElemReaction, ReactionCompound, ReactionSide};
use trait_element::Element;
use trait_properties::Properties;
use trait_reaction::Reaction;
use types::*;

#[derive(Debug, Eq, PartialEq, Clone)]
/// A Redox reaction
pub struct RedoxReaction {
    /// The reductor
    pub reductor: ElemReaction<Ion>,

    /// The oxidator
    pub oxidator: ElemReaction<Ion>,
}

impl Reaction<Ion> for RedoxReaction {
    fn equalise(&self) -> bool {
        // NOTE: This edits a clone, so doesn't do much!
        self.elem_reaction().equalise()
    }

    fn is_valid(&self) -> bool {
        // oxidator > reductor
        get_sep(&self.oxidator) > get_sep(&self.reductor) && self.elem_reaction().is_valid()
    }

    fn energy_cost(&self) -> Energy {
        self.reductor.energy_cost() + self.oxidator.energy_cost()
    }

    fn elem_reaction(&self) -> ElemReaction<Ion> {
        // NOTE: Assuming .rhs and .lhs are equalised

        let red_charge;
        let oxi_charge;

        fn electrons_by_reactionside<T>(rs: &ReactionSide<T>) -> Option<usize>
        where
            T: Element,
        {
            rs.compounds.iter().position(|x| {
                x.element.clone().get_molecule().unwrap().compounds[0]
                    .atom
                    .number
                    == AtomNumber::from(0)
            })
        }

        // Get reductor charge by searching for the electron, then getting that amount
        if let Some(red_elec_pos) = electrons_by_reactionside(&self.reductor.rhs) {
            red_charge = self.reductor.rhs.compounds[red_elec_pos].amount;
        } else if let Some(red_elec_pos) = electrons_by_reactionside(&self.reductor.lhs) {
            red_charge = self.reductor.lhs.compounds[red_elec_pos].amount;
        } else {
            panic!("Reductor has no electrons!");
        }

        // Get oxidator charge by searching for the electron, then getting that amount
        if let Some(oxi_elec_pos) = electrons_by_reactionside(&self.oxidator.lhs) {
            oxi_charge = self.oxidator.lhs.compounds[oxi_elec_pos].amount;
        } else if let Some(oxi_elec_pos) = electrons_by_reactionside(&self.oxidator.rhs) {
            oxi_charge = self.oxidator.rhs.compounds[oxi_elec_pos].amount;
        } else {
            panic!("Oxidator has no electrons!");
        }

        // Make sure that 4/2 or 2/4 gets converted to 2/1 or 1/2 first
        let gcd = gcd(i32::from(red_charge), i32::from(oxi_charge)) as u16;
        let red_mult = oxi_charge / gcd;
        let oxi_mult = red_charge / gcd;

        let mut lhs = self.reductor.lhs.clone() * red_mult + self.oxidator.lhs.clone() * oxi_mult;
        let mut rhs = self.reductor.rhs.clone() * red_mult + self.oxidator.rhs.clone() * oxi_mult;

        // Remove electrons from both sides
        let remove_electron = |x: &ReactionCompound<Ion>| -> bool {
            x.element.molecule.compounds[0].atom.number != AtomNumber::from(0)
        };

        lhs.compounds.retain(&remove_electron);
        rhs.compounds.retain(&remove_electron);

        ElemReaction {
            lhs,
            rhs,

            is_equilibrium: true,
        }
    }
}

impl Properties for RedoxReaction {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        symbol += "oxidator: ";
        symbol += &self.oxidator.symbol();
        symbol += "\n";
        symbol += "reductor: ";
        symbol += &self.reductor.symbol();

        symbol
    }

    fn name(&self) -> String {
        format!(
            "oxidator: {}\nreductor: {}",
            self.oxidator.name(),
            self.reductor.name()
        )
    }

    fn mass(&self) -> AtomMass {
        // Law of Conservation of Mass
        AtomMass::from(0.0)
    }

    fn is_diatomic(&self) -> bool {
        // Reactions can't be diatomic
        false
    }
}
