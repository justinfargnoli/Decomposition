/*
Adapted from 'https://github.com/danielbusaba/histogram'.
*/

pub struct Histogram<T> {
    sublog_bits: u64,
    // Stores sublog bits
    values: Vec<T>,      // Stores frequency values for histogram
}

impl ConvertValueToIndex for Histogram<u64> {}

/*
 * Implements methods for a histogram with numeric frequencies
 */
impl Histogram<u64> {
    /*
     * Constructor that takes in the sublog bits and the maximum reuse time
     */
    pub fn new_single(sublog_bits: u64) -> Histogram<u64> {
        //Creates a histogram with a vector of an appropriate fixed length
        Histogram {
            sublog_bits,
            values: vec![0; Histogram::sublog_to_histogram_size(sublog_bits) as usize],
        }
    }

    /*
     * Formula taken from the loca projects 'dual_fp_all.cpp' file.
     */
    fn sublog_to_histogram_size(sublog_bits: u64) -> u64 {
        (65 - sublog_bits) * (1 << sublog_bits)
    }

    pub fn add(&mut self, reuse_time: u64) {
        //Retrieves old frequency
        self.values[Histogram::convert_value_to_index(self.sublog_bits, reuse_time) as usize] += 1;
    }

    /*
     * Inserts a value into the histogram at a given reuse time
     */
    pub fn insert(&mut self, reuse_time: u64, frequency: u64) {
        // Sets bucket value to frequency
        self.values[Histogram::convert_value_to_index(self.sublog_bits, reuse_time) as usize] = frequency;
    }

    /*
     * Retreives the frequency value at a given reuse time.
     */
    pub fn get_frequency(&self, reuse_time: u64) -> u64 {
        // Returns the frequency in the reuse time's bucket
        self.values[Histogram::convert_value_to_index(self.sublog_bits, reuse_time) as usize]
    }

    /*
     * Returns copy of internal vector
     */
    pub fn get_histgram_vec(&self) -> Vec<u64> {
        self.values.clone()
    }

    /*
     * Returns the value of sublog_bits
     */
    pub fn get_sublog_bits(&self) -> u64 {
        self.sublog_bits
    }
}

trait ConvertValueToIndex {
    /*
     * Taken from locas 'histo.h'.
     */
    fn convert_value_to_index(sublog_bits: u64, value: u64) -> usize {
        //Ignores values too small to be bucketized

        if value < (1 << sublog_bits) {
            return value as usize;
        }

        let most_significant_bit = (63 - value.leading_zeros()) as u64; //Find's value's most significant bit
        let shift = most_significant_bit - sublog_bits; //Defines shift as difference between most significant bit and sublog bits
        let mut index = value >> shift; //Sets index as value shifted by shift
        index = index & ((1 << sublog_bits) - 1); //Does a bitwise and with sublog bits number of 1's

        (index + ((shift + 1) << sublog_bits)) as usize //Adds the shift + 1 shifted by the number of sublog bits and to the index
    }
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
        h.insert(512, 2); //Inserts a value into the same bucket
        assert_eq!(h.get_frequency(513), 2); //Checks the bucket value
    }

    #[test]
    fn test_histogram_addition() //Tests frequency incrementation
    {
        let mut h = Histogram::new_single(8); //Creates a new histogram for given sublog bits and maximum reuse time
        h.add(512); //Adds to 512 bucket
        assert_eq!(h.get_frequency(512), 1); //Checks frequency at 512
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
                h1.insert(i, temp + 1);
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
                    values[Histogram::convert_value_to_index(sublog_bits, i) as usize],
                    u64::pow(2, exponent as u32)
                ); //Checks if bucket size matches theory
            }
    }

    mod bucket {
        use std::hash::{Hash, Hasher};

        // Used for custom bucket hashing.
        use super::ConvertValueToIndex;

        pub struct Bucket(pub u64, pub u64); //Custom bucket struct to index reuse time (takes in value, sublog bits)

        impl ConvertValueToIndex for Bucket {}

        impl Hash for Bucket //Turns raw indexes into bucket indexes
        {
            fn hash<H: Hasher>(&self, state: &mut H) {
                Bucket::convert_value_to_index(self.0, self.1).hash(state);
            }
        }

        impl PartialEq for Bucket //Defines equality for Bucket
        {
            fn eq(&self, other: &Bucket) -> bool //Checks for equality of bucket indexes
            {
                Bucket::convert_value_to_index(self.0, self.1) == Bucket::convert_value_to_index(other.0, other.1)
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
}
