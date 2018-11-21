extern crate meval;
use crate::uuid::guid_64::Guid;
use std::rc::Rc;
use std::str;
use std::str::FromStr;
use std::error::Error;
use std::collections::VecDeque;
use std::fmt;
use meval::Expr;

const NO_VALUE: &str = "No value provided";

pub trait ConstraintOperations {
    fn evaluate(&mut self) -> Result<bool, ConstraintViolation>;
    fn push_var(&mut self, fact_name: String, fact_value: f64);
    fn mut_var(&mut self, fact_name: String, fact_value: f64);
    fn push_const(&mut self, fact_name: String, fact_value: f64);
    fn push_constraint(&mut self, constraint_name: String, constraint_value: bool);
    fn push_formula(&mut self, formula_name: String, formula_string: String);
    fn push_compare_op(&mut self, operator: ComparisonOperatorKind);
    fn push_bool_op(&mut self, operator: BoolOperatorKind);
}


#[derive(Debug)]
pub enum BoolOperatorKind { And, Not, Or, Xor }

#[derive(Debug)]
pub enum ComparisonOperatorKind { Eq, Ne, Gt, Lt, GtEq, LtEq }

#[derive(Debug)]
pub enum ConstraintRecordKind { Fact, Const, Formula, Constraint, CompareOperator, BoolOperator }

#[derive(Debug)]
pub struct Constraint {
    name: String,
    guid: Rc<Guid>,
    records: Vec<ConstraintRecord>,
    violations: Vec<ConstraintViolation>,
    errors: Vec<ConstraintViolation>,
}

#[derive(Debug)]
pub struct ConstraintRecord {
    name: String,
    record_type: ConstraintRecordKind,
    fact_value: Option<f64>,
    const_value: Option<f64>,
    constraint_value: Option<bool>,
    formula: Option<String>,
    compare_operator: Option<ComparisonOperatorKind>,
    bool_operator: Option<BoolOperatorKind>,
}

#[derive(Debug)]
pub struct ConstraintViolation {
    stack_index: usize,
    record_name: String,
    reason: String,
}

impl ConstraintRecord {
    pub fn update_fact(&mut self, new_value: f64) {
        self.fact_value = Some(new_value);
    }
}

impl ConstraintViolation {
    pub fn new(stack_index: usize, record_name: String, reason: String) -> Self {
        ConstraintViolation { stack_index, record_name, reason }
    }
}

impl fmt::Display for ConstraintViolation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Error for ConstraintViolation {
    fn description(&self) -> &str {
        self.reason.as_str()
    }
}

impl Constraint {
    pub fn new(name: String) -> Self {
        Constraint { name, guid: Guid::new(), records: Vec::new(), violations: Vec::new(), errors: Vec::new() }
    }
}

impl ConstraintOperations for Constraint {
    fn evaluate(&mut self) -> Result<bool, ConstraintViolation> {
        let mut operands = VecDeque::new();
        let mut implicants = VecDeque::new();


        let mut i_broke = ConstraintViolation::new(0, "nothing".to_string(), "no reason".to_string());

        for i in 0..self.records.len() {
            unsafe {
                let rec: &ConstraintRecord = self.records.get_unchecked(i);
                match rec.record_type {
                    ConstraintRecordKind::Fact => {
                        if let Some(x) = rec.fact_value {
                            operands.push_back(x);
                        } else {
                            let err = ConstraintViolation::new(i, rec.name.clone(), NO_VALUE.to_string());
                            self.errors.push(err);
                        }
                    }
                    ConstraintRecordKind::Const => {
                        if let Some(x) = rec.const_value {
                            operands.push_back(x);
                        } else {
                            let err = ConstraintViolation::new(i, rec.name.clone(), NO_VALUE.to_string());
                            self.errors.push(err);
                        }
                    }
                    ConstraintRecordKind::Formula => {
                        if let Some(x) = rec.formula.clone() {
                            let m = Expr::from_str(x.as_str());
                            match m {
                                Ok(e) => {
                                    match e.eval() {
                                        Ok(x) => {
                                            operands.push_back(x)
                                        }
                                        Err(r) => {
                                            let err = ConstraintViolation::new(i, rec.name.clone(), r.to_string());
                                            self.errors.push(err);
                                        }
                                    }
                                }
                                Err(r) => {
                                    let err = ConstraintViolation::new(i, rec.name.clone(), r.to_string());
                                    self.errors.push(err);
                                }
                            }
                        }
                    }
                    ConstraintRecordKind::Constraint => {
                        if let Some(x) = rec.constraint_value {
                            implicants.push_back(x);
                        } else {
                            let err = ConstraintViolation::new(i, rec.name.clone(), NO_VALUE.to_string());
                            self.errors.push(err);
                        }
                    }
                    ConstraintRecordKind::CompareOperator => {
                        match &rec.compare_operator {
                            None => {
                                let err = ConstraintViolation::new(i, rec.name.clone(), NO_VALUE.to_string());
                                self.errors.push(err);
                            }
                            Some(op) => {
                                let first = operands.pop_front();
                                let second = operands.pop_front();

                                match op {
                                    ComparisonOperatorKind::Eq => { implicants.push_back(first == second) }
                                    ComparisonOperatorKind::Ne => { implicants.push_back(first != second) }
                                    ComparisonOperatorKind::Gt => { implicants.push_back(first > second) }
                                    ComparisonOperatorKind::Lt => { implicants.push_back(first < second) }
                                    ComparisonOperatorKind::GtEq => { implicants.push_back(first >= second) }
                                    ComparisonOperatorKind::LtEq => { implicants.push_back(first <= second) }
                                }
                            }
                        }
                    }
                    ConstraintRecordKind::BoolOperator => {
                        match &rec.bool_operator {
                            None => {
                                let err = ConstraintViolation::new(i, rec.name.clone(), NO_VALUE.to_string());
                                self.errors.push(err);
                            }
                            Some(op) => {
                                let first = implicants.pop_front().unwrap();
                                let second = implicants.pop_front().unwrap();

                                match op {
                                    BoolOperatorKind::And => { implicants.push_back(first && second) }
                                    BoolOperatorKind::Not => { implicants.push_back(!(first && second)) }
                                    BoolOperatorKind::Or => { implicants.push_back(first || second) }
                                    BoolOperatorKind::Xor => {
                                        let a = first && second;
                                        let b = !first && !second;
                                        implicants.push_back(a && b)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if let true = implicants.len() == 1 { Ok(implicants.pop_front().unwrap()) } else { Err(i_broke) }
    }

    fn push_var(&mut self, fact_name: String, fact_value: f64) {
        self.records.push(ConstraintRecord {
            name: fact_name,
            record_type: ConstraintRecordKind::Fact,
            fact_value: Some(fact_value),
            const_value: None,
            formula: None,
            compare_operator: None,
            bool_operator: None,
            constraint_value: None,
        });
    }

    fn mut_var(&mut self, fact_name: String, fact_value: f64) {
        for record in self.records.iter_mut() {
            if let true = record.name.eq(&fact_name) { record.update_fact(fact_value); }
        }
    }


    fn push_const(&mut self, fact_name: String, fact_value: f64) {
        self.records.push(ConstraintRecord {
            name: fact_name,
            record_type: ConstraintRecordKind::Const,
            fact_value: None,
            const_value: Some(fact_value),
            formula: None,
            compare_operator: None,
            bool_operator: None,
            constraint_value: None,
        });
    }

    fn push_constraint(&mut self, constraint_name: String, constraint_value: bool) {
        self.records.push(ConstraintRecord {
            name: constraint_name,
            record_type: ConstraintRecordKind::Constraint,
            fact_value: None,
            const_value: None,
            formula: None,
            compare_operator: None,
            bool_operator: None,
            constraint_value: Some(constraint_value),
        });
    }

    fn push_formula(&mut self, formula_name: String, formula_string: String) {
        self.records.push(ConstraintRecord {
            name: formula_name,
            record_type: ConstraintRecordKind::Formula,
            fact_value: None,
            const_value: None,
            formula: Some(formula_string),
            compare_operator: None,
            bool_operator: None,
            constraint_value: None,
        });
    }

    fn push_compare_op(&mut self, operator: ComparisonOperatorKind) {
        self.records.push(ConstraintRecord {
            name: "Compare Operator".to_string(),
            record_type: ConstraintRecordKind::CompareOperator,
            fact_value: None,
            const_value: None,
            formula: None,
            compare_operator: Some(operator),
            bool_operator: None,
            constraint_value: None,
        });
    }

    fn push_bool_op(&mut self, operator: BoolOperatorKind) {
        self.records.push(ConstraintRecord {
            name: "Bool Operator".to_string(),
            record_type: ConstraintRecordKind::BoolOperator,
            fact_value: None,
            const_value: None,
            formula: None,
            compare_operator: None,
            bool_operator: Some(operator),
            constraint_value: None,
        });
    }
}