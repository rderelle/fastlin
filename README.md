<h1 align="center">fastlin</h1>


### Overview

Fastlin is an ultra-fast program to perform lineage typing of Mycobacterium tuberculosis complex (MTBC) samples. 

Reference:[TBA](https://www)


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
The default command line is:
```
fastlin -d /path/directory_fastq_files -b barcodes_file.txt
```
I you are are sure the data are not BAM-derived fastq files, then we would recommend to apply a maximum kmer coverage threshold to reduce runtimes: 
```
fastlin -d /path/directory_fastq_files -b barcode_file.txt -x 80
```

### Input files
<p>Fastlin takes as input the path of the directory containing the fastq files. The fastq files should be compressed, with extensions being either '.fastq.gz' or 'fq.gz'. Paired-end files should be in the form 'name_1.fq.gz' and 'name_2.fq.gz'. The directory can contain both paired-end and single-end fastq files.</p>
<p>The MTBC barcode file can be downloaded from [TBA](https://www). Alternatively, you can build and test your own kmer barcodes using the Python scripts available in that directory.</p> 


### Output file
Fastlin output consists on a tab-delimited file with the following fields:
+ sample: sample name
+ nb_files: 'single' or 'paired'-end files
+ k_cov: theoretical kmer coverage of the fastq files(s) based on the number extracted kmer ()
+ mixture: pure ('no') or mixed ('yes') sample
+ lineages: all lineages detected in the sample with their kmer occurences within paratheses
+ log_barcodes: all SNP barcodes (indicated by their kmer occurences) passing the minimum occurence threshold

Here is a simple example:
> #sample&nbsp;&nbsp;&nbsp;&nbsp;nb_files&nbsp;&nbsp;&nbsp;&nbsp;k_cov&nbsp;&nbsp;&nbsp;&nbsp;mixture&nbsp;&nbsp;&nbsp;&nbsp;lineages&nbsp;&nbsp;&nbsp;&nbsp;log_barcodes  
ERRxxxxx&nbsp;&nbsp;&nbsp;&nbsp;paired&nbsp;&nbsp;&nbsp;&nbsp;118&nbsp;&nbsp;&nbsp;&nbsp;no&nbsp;&nbsp;&nbsp;&nbsp;2&nbsp;&nbsp;&nbsp;&nbsp;(45)&nbsp;&nbsp;&nbsp;&nbsp;2 (42, 48, 39, 43, 54, 47, 45), 4.1 (4)

The sample ERRxxxxx contains a single strain belonging to lineage 2. This typing is supported by 7 kmer barcodes, with a median number of occurences of 45. Since the abundance of the strain is far below the theoretical kmer coverage (equal here to 118), we can conclude that the sample is likely to contain high level of contaminations or sequencing errors.


### TO DO LIST
+ multi-threading
+ possiblity to analyse FASTA files (genome assemblies)



