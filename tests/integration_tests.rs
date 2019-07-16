extern crate decomposition;

use decomposition::distribution::Distribution;
use decomposition::histogram::Histogram;

#[test]
pub fn create_distribution_ssh() {
    let test_file: String = String::from("/Users/justinfargnoli/IdeaProjects/decomposition/data/dhcp_10_4_4_71_wireless_rochester_edu_ssh_16563_2019_07_08T10_54_34_56_04_00_82238.hist");
    let histrogram: Histogram = Histogram::read_histogram_from_file(test_file);
    let distribution: Distribution = Distribution::build(histrogram);
    assert!(true);
}

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

    let recieved_answer: f32 = distribution.get_probability(0);
    let expected_answer: f32 = 1.0 / num_of_bins as f32;

    assert_eq!(recieved_answer, expected_answer);

    let recieved_answer: f32 = distribution.get_probability(3);
    let expected_answer: f32 = 4.0 / num_of_bins as f32;

    assert_eq!(recieved_answer, expected_answer);

    let recieved_answer: f32 = distribution.get_probability(10);
    let expected_answer: f32 = 0.0 / num_of_bins as f32;

    assert_eq!(recieved_answer, expected_answer);
}

#[test]
pub fn distribution_add_values_test() {
    let test_file: String = String::from("/Users/justinfargnoli/IdeaProjects/decomposition/data/test.hist");
    let histrogram: Histogram = Histogram::read_histogram_from_file(test_file);
    let mut distribution: Distribution = Distribution::build(histrogram);

    distribution.add_by_ri(0);
    distribution.add_by_ri(3);
    distribution.add_by_ri(10);
    let num_of_bins: usize = 14592;

    let recieved_answer: f32 = distribution.get_probability(0);
    let expected_answer: f32 = 2.0 / num_of_bins as f32;

    assert_eq!(recieved_answer, expected_answer);

    let recieved_answer: f32 = distribution.get_probability(3);
    let expected_answer: f32 = 5.0 / num_of_bins as f32;

    assert_eq!(recieved_answer, expected_answer);

    let recieved_answer: f32 = distribution.get_probability(10);
    let expected_answer: f32 = 1.0 / num_of_bins as f32;

    assert_eq!(recieved_answer, expected_answer);
}
