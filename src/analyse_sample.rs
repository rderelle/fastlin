
pub mod analyse_sample {

    use std::fs::File;
    use seq_io::fastq::{Reader,Record};
    use std::io::{BufRead, BufReader};
    use flate2::read::GzDecoder;
    use std::path::PathBuf;
    use std::str;
    use hashbrown::HashMap;
        

    pub fn scan_reads(mut vect_files: Vec<PathBuf>, barcodes: HashMap<String, String>, k_size: &u8, limit_kmer: bool, max_kmers: i64, genome_size: i64) -> (HashMap<String, i32>, i32) {
        
        // initialise kmer size
        let k = *k_size as usize;
        
        // sort vector of paths 
        vect_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        
        let mut result_barcodes: HashMap<String, i32> = HashMap::new();
        let mut kmer_counter: i64 = 0;

        'label_loop: for filename in vect_files {
            
            // set the reader
            let buf = get_reader(&filename);
            let mut reader = Reader::new(buf);
    
            // lookup records 1 by 1
            while let Some(record) = reader.next() {
            
                // unwrap record (contains name, sequence and quality)
                let record_ready = record.unwrap();
            
                // get sequences and sequence length
                let seq = record_ready.seq();
                //let len_seq = seq.len();
                
                // only consider sequences long enough to have a kmer
                if seq.len() >= k {
                    
                    // extract kmers (slices from Vect seq)  
                    for n in 0 .. (seq.len() - k + 1) {
                        
                        // get slice of Vect[u8]
                        let kmer = &seq[n .. n + k];
                    
                        // convert Vect[u8] into String
                        let seq_kmer = unsafe { str::from_utf8_unchecked(kmer)};     
                        
                        // check if kmer is known -> add to count if yes or create new count if no
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
                    
                    // update kmer counter
                    let nb_kmers = (seq.len() - k) as i64;
                    kmer_counter += nb_kmers;
                    
                    if limit_kmer {
                        // stop process if number of maximum kmer coverage reached 
                        if kmer_counter > max_kmers {
                            break 'label_loop;
                        }
                    }

                } 
            
            }
        }
        // compute kmer coverage
        let coverage = (kmer_counter as f64 / genome_size as f64).round() as i32;
        
        (result_barcodes, coverage)
    }


    fn get_reader(path: &PathBuf) -> Box<dyn BufRead + Send> {
        let mut filetype = "unzip";
        let filename_str = path.to_str().unwrap();
        let file = match File::open(path) {
                Ok(file) => file,
                Err(error) => panic!("Error opening compressed file: {:?}.", error),
            };
        if filename_str.ends_with(".gz")  {filetype = "zip";}
        let reader :Box<dyn BufRead + Send> = match filetype { 
            "zip" => Box::new(BufReader::new(GzDecoder::new(file))), 
            _ =>     Box::new(BufReader::new(file)), 
        }; 
        reader
    }
    


}