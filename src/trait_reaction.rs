use reaction::ElemReaction;
use trait_element::Element;
use trait_properties::Properties;
use types::*;

pub trait Reaction<E: Element>: Properties {
    /// Check if the reaction is valid by comparing the amount of total atoms on both sides,
    /// and by checking if the total charge on both sides is equal
    fn is_valid(&self) -> bool;

    /// Balance both sides of the equation to make sure it's valid
    /// Returns true if it managed to equalise it, false otherwise
    fn equalise(&self) -> bool;

    /// Calculate the cost of this reaction
    /// This can be negative in case of an exothermic reaction
    fn energy_cost(&self) -> Energy;

    /// Get the ElemReaction version
    fn elem_reaction(&self) -> ElemReaction<E>;
}
