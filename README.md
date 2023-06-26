# fastlin

### Overview

Fastlin is'\n

__Reference:__ [TBA](https://www)

### Installation
To install fastlin, you must have the [rust toolchain](https://www.rust-lang.org/tools/install) installed.
```
cargo install fastlin
```
Or alternatively you can copy the code from this repository and install it using this command:
```
cargo install --path .
```

### Running fastlin
<p>The default command line is:
```
fastlin -d /path/directory_fastq_files -b barcodes_file.txt
```
I you are are sure the data are not BAM-derived fastq files, then we would recommend to apply a maximum kmer coverage threshold to reduce runtimes: 
```
fastlin -d /path/directory_fastq_files -b barcode_file.txt -x 80
```
<p>

### Input files
<p>Fastlin takes as input the path of the directory containing the fastq files. The fastq files should be compressed, with extensions being either '.fastq.gz' or 'fq.gz'. Paired-end files should be in the form 'name_1.fq.gz' and 'name_2.fq.gz'. The directory can contain both paired-end and single-end fastq files.</p>
<p>The MTC barcode file can be downloaded from [TBA](https://www). Alternatively, you can build and test your own kmer barcodes using the Python scripts available in that directory.</p> 


### Output file
<p>Fastlin output consists on a tab-delimited file with the following fields:
+ sample: sample name
+ nb_files: 'single' or 'paired'-end files
+ k_cov: kmer coverage of the sample calculated from the number of kmers extracted
+ mixture: pure ('no') or mixed ('yes') sample
+ lineages: all lineages detected in the sample with their kmer occurences within paratheses
+ log_barcodes: all SNP barcodes (indicated by their kmer occurences) passing the minimum occurence threshold


### TO DO LIST
+ add multi-threading
+ add possiblity to analyse FASTA files (genome assemblies)



