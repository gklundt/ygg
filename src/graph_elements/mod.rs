
pub mod graph;
pub mod node;
pub mod edge;
pub mod location;

#[cfg(test)]
mod tests {
    
    use test::Bencher;

    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4);
    }

    #[bench]
    fn name2(b: &mut Bencher) {
        b.iter(|| assert_eq!(2 + 2, 4));
     }
}
