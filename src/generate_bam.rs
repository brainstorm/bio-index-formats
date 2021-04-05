use bam_builder::{BamBuilder, bam_order::BamSortOrder};

pub fn generate_bam() {
    // Create a builder with all defaults except the read_len is 100
    let mut builder = BamBuilder::new(
        100,                        // default read length
        30,                         // default base quality
        "HtsGetTestBamUnmapped".to_owned(), // name of sample
        None,                       // optional read group id
        BamSortOrder::Unsorted,     // how to sort reads when `.sort` is called
        None,                       // optional sequence dictionary
        Some(666),                  // optional seed used for generating random bases
    );

    // Create a single read pair with only 2 unmapped reads
    let records = builder
        .pair_builder()
        .contig(0)               // reads are mapped to tid 0
        .start1(0)               // start pos of read1
        .start2(200)             // start pos of read2
        .unmapped1(false)         // override default of unmapped
        .unmapped2(false)         // override default of unmapped
        .build()                 // inflate the underlying records and set mate info
        .unwrap();

    // Add the pair to bam builder
    builder.add_pair(records);

    // Write records to a file
    let tmp_file = builder.to_tmp().unwrap();
    let (file, path) = tmp_file.keep().unwrap();
    dbg!("Temp BAM file written to", path, file);
    
}