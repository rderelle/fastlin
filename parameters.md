
<h1 align="center">fastlin parameters</h1>


```

      fastlin     

Usage: fastlin [OPTIONS] --dir <DIR> --barcodes <BARCODES>

Options:
  -d, --dir <DIR>                directory containing the fastq.gz files
  -b, --barcodes <BARCODES>      file containing the reference barcodes
  -o, --output <OUTPUT>          output file [out_fastlin.txt] [default: output_fastlin.txt]
  -k, --kmer-size <KMER_SIZE>    kmer size [default: 25]
  -c, --min-count <MIN_COUNT>    minimum number of kmer occurences [default: 4]
  -n, --n-barcodes <N_BARCODES>  minimum number of barcodes [default: 3]
  -x, --max-cov <MAX_COV>        maximum kmer coverage
  -h, --help                     Print help
  -V, --version                  Print version

```



# Mandatory parameters

### dir

Path to the directory containing the fastq files to be analysed. The fastq files should be compressed, with extensions being either '.fastq.gz' or 'fq.gz'.
Names of paired-end files should be in the form 'name_1.fq.gz' and 'name_2.fq.gz'. The directory can contain both paired-end and single-end fastq files.

### barcodes

Path to the tabular text file containing the barcode SNPs. The MTBC barcode file can be downloaded from [here](https://www.github.com/rderelle/barcodes-fastlin).
Alternatively, you can build and test your own kmer barcodes using the Python scripts available in that directory.



# Optional parameters


### kmer-size (default = 25)

The kmer size should be an odd number between 11 and 99 nucleotides.
The minimum k-mer size should be determined empirically ([see scripts here](https://www.github.com/rderelle/barcodes-fastlin)).
Higher kmer sizes increase the specificity of barcode SNP detection (i.e., fewer false positives) but reduce fastlin's sensitivity at low k-mer coverages due to sequencing errors (longer kmers will occur less frequently).

### min-count (default = 4)

This parameter sets the minimum number of times a kmer should be found to be considered valid and not the result of sequencing errors, with the vast majority of spurious kmers being found at occurrences of 1 or 2.
Increasing the min-count parameter increases specificity but reduces sensitivity at low coverages.

### n-barcodes (default = 3)

This parameter sets the minimum number of barcode SNPs to be found for a lineage to be inferred.
Its value should be equal to or lower than the minimum number of barcode SNPs defining a lineage (e.g., in Phelan et al., 2019, the minimum number of barcode SNPs for an MTBC lineage was 4). Similar to the min-count parameter, increasing this parameter increases specificity but reduces sensitivity at low coverages.

### max-cov

This parameter allows users to define a maximum kmer coverage limit, beyond which kmers will not be extracted from the fastq files in order to reduce runtimes. 
This parameter, based on the assumption of a random read distribution in fastq files, should not be used if BAM-derived fastq files are to be analyzed. We recommend using a max-cov of no less than 80 to ensure that mixtures of strains are properly detected by fastlin (see publication).


