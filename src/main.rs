use std::fs::File;
use std::io::Write;
use std::str;
use clap::Parser;
use std::process;
use indicatif::{ProgressBar, ProgressStyle};


mod get_barcodes;
use get_barcodes::get_barcodes::get_barcodes;

mod input_files;
use input_files::input_files::get_input_files;

mod analyse_sample;
use analyse_sample::analyse_sample::scan_reads;

mod process_barcodes;
use process_barcodes::process_barcodes::process_barcodes;


#[derive(Parser, Debug)]
#[command(author = None, version, about = None, long_about = None)]
struct Args {
    /// directory containing the fastq.gz files
    #[arg(short, long)]
    dir: String,

    /// file containing the reference barcodes
    #[arg(short = 'b', long)]
    barcodes: String,

    /// output file [out_fastlin.txt]
    #[arg(short = 'o', long, default_value_t = String::from("output_fastlin.txt"))]
    output: String,

    /// kmer size
    #[arg(short, long, default_value_t = 25)]
    kmer_size: u8,

    /// minimum number of kmer occurences
    #[arg(short = 'c', long, default_value_t = 4)]
    min_count: i32,
    
    /// minimum number of barcodes
    #[arg(short = 'n', long, default_value_t = 3)]
    n_barcodes: usize,

    /// maximum kmer coverage
    #[arg(short = 'x', long)]
    max_cov: Option<i64>,
}


#[allow(unused_assignments)]
fn sample_nb_files(name_sample: String, nb_files:usize) -> String {
    // depending on the number of files, returns 'single', 'paired' or exit with error message
    let mut result = "";
    if nb_files == 1  {result = "single";}
    else if nb_files == 2  {result = "paired";}
    else {
        eprintln!("error: the sample {} has {} files", name_sample, nb_files);
        process::abort();
    }
    result.to_string()
}


fn main() {
    println!("\n      fastlin     \n");
    
    // get command line arguments
    let args = Args::parse();
    
    // check chosen kmer size
    if args.kmer_size < 11 || args.kmer_size > 99 || args.kmer_size % 2 == 0 {
        // warning message
        eprintln!(" Error: the kmer size should be an odd number between 11 and 99.\n");
        // exit fastlin
        std::process::exit(0);
    }
    
    // get reference barcodes
    let (barcodes, genome_size) = get_barcodes((&args.barcodes).into(), &args.kmer_size);
    
    // calculate maximum number of kmers to extract (and limit_kmer = true if such limit exists)
    let mut max_kmers = 0;
    let mut limit_kmer = false;
    match args.max_cov {
        Some(value) => {
            max_kmers = genome_size * value;
            limit_kmer = true; 
        }
        None => {}
    }
        
    // get samples and input files
    let all_samples = get_input_files(&args.dir);

    // sort samples
    let mut sorted_samples: Vec<_> = all_samples.iter().collect();
    sorted_samples.sort_by_key(|k| k.0);
    
    // create output file
    let mut output_file = File::create(args.output).expect("\n   Warning: couldn't not create output file.\n");
    output_file.write("#sample	nb_files	k_cov	mixture	lineages	log_barcodes	log_errors\n".as_bytes()).expect("write failed!");

    // initialise progress bar
    let pb = ProgressBar::new(sorted_samples.len().try_into().unwrap());
    let sty = ProgressStyle::with_template("   {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}",).unwrap().progress_chars("##-");
    pb.set_style(sty);
    
    // process samples 1 by 1
    println!(" . analyse all samples");
    for (sample, list_files) in &sorted_samples {
        
        // progress bar
        pb.inc(1);

        // get sequencing type ('single' or 'paired' reads)
        let type_reads = sample_nb_files(sample.to_string(), list_files.len());
        
        // analyse reads
        let (barcode_found, coverage, error_message) = scan_reads(list_files.to_vec(), barcodes.to_owned(), &args.kmer_size, limit_kmer, max_kmers, genome_size);
        
        // process barcodes
        let (lineages, mixture, string_occurences) = process_barcodes(barcode_found, args.min_count, args.n_barcodes);
        
        // write sample info into output file
        write!(output_file, "{}\t{}\t{}\t{}\t{}\t{}\t{}\n", sample, type_reads, coverage, mixture, lineages, string_occurences, error_message).expect("Failed to write to file");
        
    }
    
    println!("   done.");
}
