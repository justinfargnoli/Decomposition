use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Adapted from 'https://github.com/danielbusaba/histogram'.
*/


pub struct Histogram<T> {
    sublog_bits: u64,
    // Stores sublog bits
    values: Vec<T>,      // Stores frequency values for histogram
}

/*
 * Implements methods for a histogram with numeric frequencies
 */
impl Histogram<u128> {
    pub fn read_histogram_from_file(file_name: String) -> Histogram<u128> {
        let file: File = match File::open(file_name) {
            Ok(t) => t,
            Err(e) => panic!("An error when creating the File struct: \n\t {}", e),
        };
        let buf_reader = BufReader::new(file);
        let mut lines = buf_reader.lines();

        let sublog_bits: String = match lines.next() {
            Some(sublog_bits_result) => {
                match sublog_bits_result {
                    Ok(t) => t,
                    Err(e) => panic!("An error occurred when reading the the value of sublog_bits: \n\t {}", e)
                }
            }
            None => panic!("None was returned when reading the the value of sublog_bits."),
        };
        let sublog_bits: u64 = sublog_bits.parse().unwrap();
        let mut histogram_struct: Histogram<u128> = Histogram::new_single(sublog_bits);

        lines.next(); // Passes over the number of buckets in the sublog histogram
        // because Histogram does not need that information

        let mut index: usize = 0;
        for line in lines {
            let frequency: String = match line {
                Ok(t) => t,
                Err(e) => panic!("An error occurred when reading a frequency from the data file: \n\t {}", e),
            };
            let frequency: u128 = frequency.parse().unwrap();
            histogram_struct.set_frequency_via_index(index, frequency);
            index += 1;
        }

        histogram_struct
    }

    /*
     * Constructor that takes in the sublog bits and the maximum reuse time
     */
    fn new_single(sublog_bits: u64) -> Histogram<u128> {
        //Creates a histogram with a vector of an appropriate fixed length
        Histogram {
            sublog_bits,
            values: vec![0; Histogram::sublog_to_histogram_size(sublog_bits) as usize],
        }
    }

    /*
     * Formula taken from the loca projects 'dual_fp_all.cpp' file.
     */
    fn sublog_to_histogram_size(sublog_bits: u64) -> u128 {
        (65 - sublog_bits) as u128 * (1 << (sublog_bits as u128))
    }

    /*
     * Inserts a value into the histogram at a given reuse time
     */
    pub fn set_frequency_via_index(&mut self, index: usize, frequency: u128) {
        // Sets bucket value to frequency
        self.values[index] = frequency;
    }

    /*
     * Retreives the frequency value at a given reuse time.
     */
    pub fn get_frequency(&self, reuse_interval: u64) -> u128 {
        // Returns the frequency in the reuse time's bucket
        self.values[convert_reuse_interval_to_index(self.sublog_bits, reuse_interval) as usize]
    }

    /*
     * Returns copy of internal vector
     */
    pub fn get_histgram_vec(&self) -> Vec<u128> {
        self.values.clone()
    }

    /*
     * Returns the value of sublog_bits
     */
    pub fn get_sublog_bits(&self) -> u64 {
        self.sublog_bits
    }

    pub fn add(&mut self, reuse_interval: u64) {
        //Retrieves old frequency
        self.values[convert_reuse_interval_to_index(self.sublog_bits, reuse_interval) as usize] += 1;
    }

    /*
     * Inserts a value into the histogram at a given reuse time
     */
    pub fn set_frequency_via_reuse_interval(&mut self, reuse_interval: u64, frequency: u128) {
        self.values[convert_reuse_interval_to_index(self.sublog_bits, reuse_interval) as usize] = frequency;
    }
}

/*
 * Taken from locas 'histo.h'.
 */
fn convert_reuse_interval_to_index(sublog_bits: u64, reuse_interval: u64) -> usize {
    //Ignores values too small to be bucketized

    if reuse_interval < (1 << sublog_bits) {
        return reuse_interval as usize;
    }

    let most_significant_bit = (63 - reuse_interval.leading_zeros()) as u64; //Find's value's most significant bit
    let shift = most_significant_bit - sublog_bits; //Defines shift as difference between most significant bit and sublog bits
    let mut index = reuse_interval >> shift; //Sets index as value shifted by shift
    index = index & ((1 << sublog_bits) - 1); //Does a bitwise and with sublog bits number of 1's

    (index + ((shift + 1) << sublog_bits)) as usize //Adds the shift + 1 shifted by the number of sublog bits and to the index
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bucket::Bucket;

    use super::*;

//Can be used with bucket to create sublog histograms

    // #[test]  //Fails
    // fn test_index_to_value()
    // {
    //     assert_eq!(convert_index_to_value(514, 8), 514);
    // }

    #[test]
    fn test_hash() //Demonstrates bucket usage in HashMap
    {
        let b1 = Bucket(8, 512);
        let b2 = Bucket(8, 513);

        let mut histogram: HashMap<Bucket, usize> = HashMap::new();
        histogram.insert(b1.clone(), 1);

        if histogram.contains_key(&b2)
        //Finds ame bucket as b1
        {
            let temp = histogram.get(&b2).unwrap().clone();
            histogram.insert(b2, temp + 1); //Overwrites b1
        }

        assert_eq!(*histogram.get(&b1).unwrap(), 2);
    }

    #[test]
    fn test_histogram_insertion() //Tests histogram buckets
    {
        let mut h = Histogram::new_single(8); //Creates a new histogram for given sublog bits and maximum reuse time
        h.set_frequency_via_reuse_interval(512, 2); //Inserts a value into the same bucket
        assert_eq!(h.get_frequency(513), 2); //Checks the bucket value
    }

    #[test]
    fn test_histogram_addition() //Tests frequency incrementation
    {
        let mut h = Histogram::new_single(8); //Creates a new histogram for given sublog bits and maximum reuse time
        h.add(512); //Adds to 512 bucket
        assert_eq!(h.get_frequency(512), 1); //Checks frequency at 512
    }

    mod bucket {
        use std::hash::{Hash, Hasher};

        // Used for custom bucket hashing.
        use super::convert_reuse_interval_to_index;

        pub struct Bucket(pub u64, pub u64); //Custom bucket struct to index reuse time (takes in value, sublog bits)

        impl Hash for Bucket //Turns raw indexes into bucket indexes
        {
            fn hash<H: Hasher>(&self, state: &mut H) {
                convert_reuse_interval_to_index(self.0, self.1).hash(state);
            }
        }

        impl PartialEq for Bucket //Defines equality for Bucket
        {
            fn eq(&self, other: &Bucket) -> bool //Checks for equality of bucket indexes
            {
                convert_reuse_interval_to_index(self.0, self.1) == convert_reuse_interval_to_index(other.0, other.1)
            }
        }

        impl Eq for Bucket {}

        impl Clone for Bucket //Defines copying for custom bucket
        {
            fn clone(&self) -> Bucket //Returns a copy of the custom bucket
            {
                Bucket(self.0, self.1)
            }
        }
    }

    #[test]
    fn test_histogram_values() //Tests iterating through histogram
    {
        let sublog_bits = 1;
        let mut h1 = Histogram::new_single(sublog_bits); //Creates a new histogram for given sublog bits and maximum reuse time
        for i in 1..=7
            //Fills each bucket with its bucket size
            {
                let temp = h1.get_frequency(i);
                h1.set_frequency_via_reuse_interval(i, temp + 1);
            }

        let values = h1.get_histgram_vec(); //Retrieves histogram values
        for i in 1..=7
            //Iterates through histogram
            {
                let mut exponent = (63 - (i as u64).leading_zeros()) as i64 - sublog_bits as i64; //Defines exponent in 2^(bits - sublog_bits) formula
                if exponent < 0
                //Sets exponent to 0 if less than 0
                {
                    exponent = 0;
                }
                assert_eq!(
                    values[convert_reuse_interval_to_index(sublog_bits, i) as usize],
                    u128::pow(2, exponent as u32)
                ); //Checks if bucket size matches theory
            }
    }

    /*
        Tests the read_histogram method.
    */

    #[test]
    fn read_test_hist_for_sublog_bits() {
        let test_file: String = String::from("/Users/justinfargnoli/IdeaProjects/decomposition/data/test.hist");
        let histrogram: Histogram<u128> = Histogram::read_histogram_from_file(test_file);

        assert_eq!(histrogram.get_sublog_bits(), 8);
    }

    #[test]
    fn read_test_hist_for_histogram_length() {
        let test_file: String = String::from("/Users/justinfargnoli/IdeaProjects/decomposition/data/test.hist");
        let histrogram: Histogram<u128> = Histogram::read_histogram_from_file(test_file);

        assert_eq!(histrogram.get_histgram_vec().len(), 14592);
    }

    #[test]
    fn read_test_hist_for_first_values() {
        let test_file: String = String::from("/Users/justinfargnoli/IdeaProjects/decomposition/data/test.hist");
        let histrogram: Histogram<u128> = Histogram::read_histogram_from_file(test_file);
        let first_values: Vec<u128> = vec![1, 2, 3, 4, 3, 2, 1, 0];

        assert_eq!(histrogram.get_histgram_vec()[0..8], first_values[..]);
    }

    #[test]
    fn read_ssh_hist_for_sublog_bits() {
        let test_file: String = String::from("/Users/justinfargnoli/IdeaProjects/decomposition/data/dhcp_10_4_4_71_wireless_rochester_edu_ssh_16563_2019_07_08T10_54_34_56_04_00_82238.hist");
        let histrogram: Histogram<u128> = Histogram::read_histogram_from_file(test_file);

        assert_eq!(histrogram.get_sublog_bits(), 8);
    }

    #[test]
    fn read_ssh_hist_for_histogram_length() {
        let test_file: String = String::from("/Users/justinfargnoli/IdeaProjects/decomposition/data/dhcp_10_4_4_71_wireless_rochester_edu_ssh_16563_2019_07_08T10_54_34_56_04_00_82238.hist");
        let histrogram: Histogram<u128> = Histogram::read_histogram_from_file(test_file);

        assert_eq!(histrogram.get_histgram_vec().len(), 14592);
    }

    #[test]
    fn read_ssh_hist_for_first_values() {
        let test_file: String = String::from("/Users/justinfargnoli/IdeaProjects/decomposition/data/dhcp_10_4_4_71_wireless_rochester_edu_ssh_16563_2019_07_08T10_54_34_56_04_00_82238.hist");
        let histrogram: Histogram<u128> = Histogram::read_histogram_from_file(test_file);
        let first_values: Vec<u128> = vec![00, 6750181113855227, 156526102, 3922575107523, 180368130756, 3098870710510, 4650485551176, 874884546544540];

        assert_eq!(histrogram.get_histgram_vec()[0..8], first_values[..]);
    }
}
