use cycle::Cycle;
use distribution::Distribution;
use histogram::Histogram;

mod histogram;
mod distribution;
mod cycle;

pub struct Decomposer {
    cycles: Vec<Cycle>,
}

impl Decomposer {
    pub fn initialize(histogram_file_path: String) -> Decomposer {
        let histrogram: Histogram = Histogram::read_histogram_from_file(histogram_file_path);
        let distribution: Distribution = Distribution::build(histrogram);

        let mut cycles: Vec<Cycle> = Vec::new();
        for (reuse_interval, probability) in distribution.get_distribution().iter().enumerate() {
            let data_size: u32 = (reuse_interval as f32 * *probability) as u32;
            let cycle: Cycle = Cycle::build(data_size, reuse_interval);
            cycles.push(cycle);
        }

        Decomposer {
            cycles,
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
