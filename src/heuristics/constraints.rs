use std::collections::HashMap;
use std::rc::Rc;
use crate::uuid::guid_64::Guid;

pub enum ComparisonOperatorKind { Eq, Ne, Gt, Lt, GtEq, LtEq }

pub enum BoolOperatorKind { And, Not, Or, Xor }

enum ConstraintRecordKind { Fact, Const, Formula, CompareOperator, BoolOperator }

struct ConstraintRecord{
    name: String,
    record_type: ConstraintRecordKind,
    fact_value: Option<f64>,
    const_value: Option<f64>,
    formula: Option<String>,
    compare_operator: Option<ComparisonOperatorKind>,
    bool_operator: Option<BoolOperatorKind>,
}

pub struct Constraint {
    name: String,
    guid: Rc<Guid>,
    records: Vec<ConstraintRecord>,
}

pub trait ConstraintOperations {
    fn evaluate(&self) -> bool;
    fn push_fact(&self, fact_name: String, fact_value: f64);
    fn push_const(&self, fact_name: String, fact_value: f64);
    fn push_formula(&self, formula_name: String, formula_string: String);
    fn push_compare_op(&self, operator: ComparisonOperatorKind);
    fn push_bool_op(&self, operator: BoolOperatorKind);
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