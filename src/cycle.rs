pub struct Cycle {
    data_size: u32,
    periodicity: usize,
}

impl Cycle {
    pub fn build(data_size: u32, periodicity: usize) -> Cycle {
        Cycle {
            data_size,
            periodicity,
        }
    }
}


#[cfg(test)]
pub mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}