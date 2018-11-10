#![feature(test)]
#![feature(extern_crate_item_prelude)]
extern crate test;

pub mod graph_elements;
pub mod uuid;
pub mod metrics;
pub mod solutions;

#[cfg(test)]
mod tests;
