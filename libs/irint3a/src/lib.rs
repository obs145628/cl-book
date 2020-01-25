pub mod ir;
pub mod irbuilder;
pub mod irparser;
pub mod irprinter;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
