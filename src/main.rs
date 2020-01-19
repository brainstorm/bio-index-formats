use std::path::Path;

use bio_index_formats::parser_bai::{parse_bai, parse_voffset, Ref};
use bio_index_formats::csi::{ reg2bin };
use rust_htslib::bam::{ Reader, Read };

pub fn reference_ids(fname: String) -> Vec<String> {
    let reader = Reader::from_path(&Path::new(fname.as_str())).expect("Cannot read BAM file");

    reader.header().target_names().into_iter()
        .map(|raw_name| String::from_utf8_lossy(raw_name).to_string())
        .collect()
}

fn htsget_query(reference: &Ref, start: u32, end: u32) -> (u32, u32) {
    let mut range_beg = u32::max_value(); // Must be Option instead of Integer... if it does not find anything, then None.
    let mut range_end: u32 = 0;
    let bin_id: u32;

    bin_id = reg2bin(start, end);

    for bin in reference.bins.iter() {
        if bin_id == bin.bin_id {
            for chunk in bin.chunks.iter() {
                let chunk_beg = parse_voffset(chunk.chunk_beg).0;
                let chunk_end = parse_voffset(chunk.chunk_end).0;

                range_beg = range_beg.min(chunk_beg);
                range_end = range_end.max(chunk_end);
            }
        }
    }

    // Only interested in compressed offset for the final htsget range (request to BAM)
    (range_beg, range_end)
}

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    const BAI_FILE: &'static [u8] = include_bytes!("../tests/data/htsnexus_test_NA12878.bam.bai");
    //const BAM_FNAME: &'static [u8] = include_bytes!("../tests/data/htsnexus_test_NA12878.bam");

    // 27MB BAI file
    //const BAI_FILE: &'static [u8] = include_bytes!("../tests/data/SBJ00154_PRJ190634_LPRJ190634-ready.bam.bai");

    let bam_fname = "tests/data/htsnexus_test_NA12878.bam";
    let ref_names = reference_ids(bam_fname.to_string());

    let bai = parse_bai(BAI_FILE);
    let refs = bai.map(|r| r.1.refs)?;

    let chrom = "11";

    let ref_id = ref_names.iter().position(|name| name == chrom).unwrap();
    let reference = &refs[ref_id];
    
    let range = htsget_query(reference, 4999976, 5002147);
    dbg!(range);

    Ok(())
}