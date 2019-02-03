extern crate petgraph;
extern crate primal;

#[allow(dead_code)]
mod systematic_constants;
#[allow(dead_code)]
mod rng;
#[allow(dead_code)]
mod octet;
#[allow(dead_code)]
mod symbol;
#[allow(dead_code)]
mod matrix;
#[allow(dead_code)]
mod constraint_matrix;
#[allow(dead_code)]
mod base;

use constraint_matrix::generate_constraint_matrix;
use octet::Octet;
use systematic_constants::extended_source_block_symbols;

fn main() {
    for elements in [10, 100, 1000, 10000].iter() {
        let num_symbols = extended_source_block_symbols(*elements);
        let a = generate_constraint_matrix(num_symbols, 0..num_symbols);
        let mut density = 0;
        for i in 0..a.height() {
            for j in 0..a.width() {
                if a.get(i, j) != Octet::zero() {
                    density += 1;
                }
            }
        }
        println!("Original density for {}x{}: {} of {}", a.height(), a.width(), density, a.height() * a.width());

        let inverse = a.inverse().unwrap();
        let mut density = 0;
        for i in 0..inverse.height() {
            for j in 0..inverse.width() {
                if inverse.get(i, j) != Octet::zero() {
                    density += 1;
                }
            }
        }
        println!("Inverse density for {}x{}: {} of {}", inverse.height(), inverse.width(), density, inverse.height() * inverse.width());
    }
}