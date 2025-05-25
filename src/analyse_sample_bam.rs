use hashbrown::HashMap;
use rust_htslib::bam;
use rust_htslib::bam::{Read, Record};
use std::path::PathBuf;

pub fn scan_bam(
    mut vect_files: Vec<PathBuf>,
    barcodes: HashMap<Vec<u8>, String>,
    k_size: &u8,
    genome_size: i64,
) -> (HashMap<String, i32>, i32, String) {
    // initialise kmer size
    let k = *k_size as usize;

    // sort vector of paths
    vect_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let mut result_barcodes: HashMap<String, i32> = HashMap::new();
    let mut kmer_counter: i64 = 0;
    let mut error_message: String = String::new();

    'label_loop: for filename in vect_files {
        // Try to open the BAM file and handle any errors
        let mut reader = match bam::Reader::from_path(&filename) {
            Ok(reader) => reader,
            Err(e) => {
                // capture the error message
                error_message = format!("Failed to open file {:?}: {}", filename, e);
                break 'label_loop; // Exit the loop if an error occurs
            }
        };

        let mut result = Record::new();

        while let Some(record) = reader.read(&mut result) {
            let sequence = match record {
                Ok(_record) => result.seq().as_bytes(),
                Err(err) => {
                    // re-initialise barcode results (-> no result)
                    result_barcodes = HashMap::new();
                    // save error message
                    error_message = format!("Error in file {:?}: {}", filename, err);
                    // stop reading file(s)
                    break 'label_loop;
                }
            };

            // only consider sequences long enough to have a kmer
            if sequence.len() >= k {
                // extract kmers (slices from Vect seq)
                for n in 0..(sequence.len() - k + 1) {
                    // get slice of Vect[u8]
                    let kmer = &sequence[n..n + k];

                    // check if kmer is known -> add to count if yes or create new count if no
                    if let Some(id) = barcodes.get(kmer) {
                        *result_barcodes.entry(id.clone()).or_insert(0) += 1;
                    }
                }

                // update kmer counter
                let nb_kmers = (sequence.len() - k) as i64;
                kmer_counter += nb_kmers;
            }
        }
    }

    // compute kmer coverage
    let coverage = (kmer_counter as f64 / genome_size as f64).round() as i32;
    (result_barcodes, coverage, error_message)
}
