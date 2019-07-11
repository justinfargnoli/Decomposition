use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::histogram::Histogram;

pub fn file_to_struct(file_name: String) -> Histogram<u64> {
    let file: File = match File::open(file_name) {
        Ok(T) => T,
        Err(E) => panic!("An error when creating the File struct: \n\t {}", E),
    };
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();

    let sublog_bits: String = match lines.next() {
        Some(sublog_bits_result) => {
            match sublog_bits_result {
                Ok(T) => T,
                Err(E) => panic!("An error occurred when reading the the value of sublog_bits: \n\t {}", E)
            }
        }
        None => panic!("None was returned when reading the the value of sublog_bits."),
    };
    let sublog_bits: u64 = sublog_bits.parse().unwrap();
    let histogram_struct: Histogram<u64> = Histogram::new_single(sublog_bits);

    lines.next(); // Passes over the number of buckets in the sublog histogram
    // because Histogram does not need that information

    histogram_struct
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1)
    }

    #[test]
    fn read_test_hist_for_sublog_bits() {
        let test_file: String = String::from("/Users/justinfargnoli/IdeaProjects/decomposition/data/test.hist");
        let histrogram: Histogram<u64> = file_to_struct(test_file);

        assert_eq!(histrogram.get_sublog_bits(), 8);
    }

    #[test]
    fn read_test_hist_for_histogram_length() {
        let test_file: String = String::from("/Users/justinfargnoli/IdeaProjects/decomposition/data/test.hist");
        let histrogram: Histogram<u64> = file_to_struct(test_file);

        assert_eq!(histrogram.get_histgram_vec().len(), 14592);
    }
}