use std::collections::HashMap;

pub enum ComparisonOperatorKind { Eq, Ne, Gt, Lt, GtEq, LtEq }
pub enum BoolOperatorKind { And, Not, Or, Xor }

pub struct Constraint {}

pub trait Constrainer {
    fn evaluate(&self) -> bool;
    fn push_fact(&self, HashMap<String, f64>);
    fn push_formula(&self, HashMap<String, String>);
    fn push_compare_op(&self);
    fn push_bool_op();
}

/*
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


*/