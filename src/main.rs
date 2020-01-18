use std::path::Path;
//use std::collections::HashSet;

use bio_index_formats::parser_bai::{parse_bai, parse_voffset, Ref};
use bio_index_formats::csi::reg2bins;
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
    let bin_ids:Vec<u32>;

//    let bin_ids: HashSet::<u32> = reg2bins(start, end).iter().cloned().collect();
    bin_ids = reg2bins(start, end);
    println!("{:?}", &bin_ids);

    //println!("{:?}", reference);
    for bin in reference.bins.iter() {
        if bin_ids.contains(&bin.bin_id) { // XXX: Explore sets instead. contains is not efficient
            for chunk in bin.chunks.iter() {
                let chunk_beg = parse_voffset(chunk.chunk_beg).1;
                let chunk_end = parse_voffset(chunk.chunk_end).1;
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
    //println!("{:?}", ref_names);
    //dbg!(ref_names);

    let bai = parse_bai(BAI_FILE);
    let refs = bai.map(|r| r.1.refs)?;
    //println!("References = {:?}", refs);
    //println!("Number of refs in the BAI = {}", refs.len());

    let chrom = "11";

    let ref_id = ref_names.iter().position(|name| name == chrom).unwrap();
    //println!("ref_id = {}", ref_id);
    let reference = &refs[ref_id];
    println!("reference = {:?}", reference);
    
    let range = htsget_query(reference, 4999976, 5002147);
    dbg!(range);

//    let a_bin = &a_chunk[10].bins[0].chunks;
//    dbg!(a_bin);
//                               .map(|b| b[10].bins)
//                               .map(|c| c[1].clone().chunks)
//                               .map(|cp| cp[0].clone())?;

//    dbg!(parse_voffsets(a_chunk.chunk_beg)?);
//    dbg!(parse_voffsets(a_chunk.chunk_end)?);

    Ok(())
}