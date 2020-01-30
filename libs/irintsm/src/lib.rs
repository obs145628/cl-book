pub mod ir;
pub mod irbuilder;
pub mod irprinter;
pub mod irvalidation;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
