static mut UNIQUE_SIZE_CNT: usize = 0;

pub fn unique_usize() -> usize {
    unsafe {
        UNIQUE_SIZE_CNT += 1;
    }
    unsafe { UNIQUE_SIZE_CNT }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_unique_usize() {
        assert_eq!(unique_usize(), 1);
        assert_eq!(unique_usize(), 2);
        assert_eq!(unique_usize(), 3);
    }
}
