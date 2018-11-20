use std::collections::HashMap;
use crate::heuristics::resources::ResourceKind;
use crate::heuristics::resource_modifiers::ResourceModifierKind;
use crate::heuristics::constraints::Constraint;

pub struct HeuristicsModel {
    // facts
    variables: HashMap<String, f64>,
    // immutable facts
    constants: HashMap<String, f64>,
    // an expression that evaluates to a fact
    // need an expression parser that constructs these
    // Git examples
    // https://github.com/rodolf0/tox/tree/master/shunting
    // meval ... from crates.io
    // https://github.com/rodolf0/tox
    formulae: HashMap<String, String>,
    constraints: HashMap<String, Constraint>,
    resources: HashMap<String, ResourceKind>,
    modifiers: HashMap<String, ResourceModifierKind>,
}

