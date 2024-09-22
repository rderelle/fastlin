pub mod analyse_sample_bam {

    use rust_htslib::bam::{Read, Record};
    use rust_htslib::bam;
    use std::path::PathBuf;
    use std::str;
    use hashbrown::HashMap;
    
    pub fn scan_bam(mut vect_files: Vec<PathBuf>, barcodes: HashMap<String, String>, k_size: &u8, genome_size: i64) -> (HashMap<String, i32>, i32, String) {

        // Initialise kmer size
        let k = *k_size as usize;

        // Sort vector of paths 
        vect_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

        let mut result_barcodes: HashMap<String, i32> = HashMap::new();
        let mut kmer_counter: i64 = 0;
        let mut error_message: String = String::new();

        'label_loop: for filename in vect_files {
            
            // Try to open the BAM file and handle any errors
            let mut reader = match bam::Reader::from_path(&filename) {
                Ok(reader) => reader,
                Err(e) => {
                    // Capture the error message
                    error_message = format!("Failed to open file {:?}: {}", filename, e);
                    break 'label_loop; // Exit the loop if an error occurs
                }
            };

            let mut result = Record::new();
            
            while let Some(record) = reader.read(&mut result) {
                let sequence = match record {
                    Ok(_record) => result.seq().as_bytes(),
                    Err(err) => {
                        // Re-initialise barcode results (-> no result)
                        result_barcodes = HashMap::new();
                        // Save error message
                        error_message = format!("Error in file {:?}: {}", filename, err);
                        // Stop reading file(s)
                        break 'label_loop;
                    }
                };

                // Only consider sequences long enough to have a kmer
                if sequence.len() >= k {
                    
                    // Extract kmers (slices from Vect seq)  
                    for n in 0..(sequence.len() - k + 1) {
                        
                        // Get slice of Vect[u8]
                        let kmer = &sequence[n..n + k];
                    
                        // Convert Vect[u8] into String
                        let seq_kmer = unsafe { str::from_utf8_unchecked(kmer) };     
                        
                        // Check if kmer is known -> add to count if yes or create new count if no
                        match barcodes.get(seq_kmer) {
                            Some(id) => {
                                match result_barcodes.get(id) {
                                    Some(count) => { result_barcodes.insert(id.to_string(), count + 1); }
                                    None => { result_barcodes.insert(id.to_string(), 1); }
                                }
                            }
                            None => {}
                        }
                    }
                
                    // Update kmer counter
                    let nb_kmers = (sequence.len() - k) as i64;
                    kmer_counter += nb_kmers;   
                }
            }
        }

        // Compute kmer coverage
        let coverage = (kmer_counter as f64 / genome_size as f64).round() as i32;
        (result_barcodes, coverage, error_message)
    }
}
