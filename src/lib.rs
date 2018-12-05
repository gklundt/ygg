#![feature(test)]
#![feature(extern_crate_item_prelude)]
#![feature(vec_remove_item)]
#![feature(toowned_clone_into)]
extern crate test;

pub mod graph_elements;
pub mod uuid;
pub mod metrics;
pub mod solutions;
pub mod heuristics;
#[cfg(test)]
mod tests;


