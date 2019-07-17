use crate::histogram::Histogram;

pub struct Distribution {
    distribution: Vec<f32>,
}

impl Distribution {

    pub fn build(histogram: Histogram) -> Distribution {
        let num_of_bins: usize = histogram.size();
        let mut distribution: Vec<f32> = vec![0.0; num_of_bins];

        for (i, bucket_value) in histogram.get_histogram_vec().iter().enumerate() {
            distribution[i] = *bucket_value as f32 / num_of_bins as f32;
        }

        Distribution {
            distribution,
        }
    }

    pub fn get_distribution(&self) -> &Vec<f32> {
        &self.distribution
    }
}


#[cfg(test)]
pub mod tests {
    use crate::histogram::Histogram;

    use super::*;

    #[allow(unused)]
    #[test]
    pub fn create_distribution_ssh() {
        let test_file: String = String::from("/Users/justinfargnoli/IdeaProjects/decomposition/data/dhcp_10_4_4_71_wireless_rochester_edu_ssh_16563_2019_07_08T10_54_34_56_04_00_82238.hist");
        let histrogram: Histogram = Histogram::read_histogram_from_file(test_file);
        let distribution: Distribution = Distribution::build(histrogram);
        assert!(true);
    }

    #[allow(unused)]
    #[test]
    pub fn create_distribution_test() {
        let test_file: String = String::from("/Users/justinfargnoli/IdeaProjects/decomposition/data/test.hist");
        let histrogram: Histogram = Histogram::read_histogram_from_file(test_file);
        let distribution: Distribution = Distribution::build(histrogram);
        assert!(true);
    }

    #[test]
    pub fn distribution_values_test() {
        let test_file: String = String::from("/Users/justinfargnoli/IdeaProjects/decomposition/data/test.hist");
        let histrogram: Histogram = Histogram::read_histogram_from_file(test_file);
        let distribution: Distribution = Distribution::build(histrogram);

        let num_of_bins: usize = 14592;

        let recieved_answer: f32 = distribution.get_distribution()[0];
        let expected_answer: f32 = 1.0 / num_of_bins as f32;

        assert_eq!(recieved_answer, expected_answer);

        let recieved_answer: f32 = distribution.get_distribution()[3];
        let expected_answer: f32 = 4.0 / num_of_bins as f32;

        assert_eq!(recieved_answer, expected_answer);

        let recieved_answer: f32 = distribution.get_distribution()[10];
        let expected_answer: f32 = 0.0 / num_of_bins as f32;

        assert_eq!(recieved_answer, expected_answer);
    }
}