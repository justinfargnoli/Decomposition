use crate::histogram::Histogram;

pub struct Distribution {
    distribution: Vec<f32>,
    num_of_bins: usize,
}

impl Distribution {
    pub fn new(num_of_access: usize) -> Distribution {
        Distribution {
            distribution: Vec::with_capacity(num_of_access),
            num_of_bins: num_of_access,
        }
    }

    pub fn build(histogram: Histogram) -> Distribution {
        let num_of_bins: usize = histogram.size();
        let mut distribution: Vec<f32> = vec![0.0; num_of_bins];

        for (i, bucket_value) in histogram.get_histogram_vec().iter().enumerate() {
            distribution[i] = *bucket_value as f32 / num_of_bins as f32;
        }

        Distribution {
            distribution,
            num_of_bins,
        }
    }

    pub fn add_by_ri(&mut self, reuse_interval: usize) {
        self.distribution[reuse_interval] += 1.0 / self.num_of_bins as f32
    }

    pub fn get_probability(&self, reuse_interval: usize) -> f32 {
        self.distribution[reuse_interval]
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn new_w_correct_capcaity() {
        let num_of_accesses: usize = 15;
        let distribution: Distribution = Distribution::new(num_of_accesses);

        assert_eq!(distribution.distribution.capacity(), num_of_accesses)
    }

    #[test]
    pub fn new_w_correct_length() {
        let num_of_accesses: usize = 15;
        let distribution: Distribution = Distribution::new(num_of_accesses);

        assert_eq!(distribution.distribution.len(), 0)
    }
}