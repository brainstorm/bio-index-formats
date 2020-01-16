use bio_index_formats::parser_bai::{parse_bai, parse_voffset, Ref};
use bio_index_formats::csi::reg2bins;

fn htsget_query(refs: &Vec<Ref>, _chrom: u32, start: u16, end: u16) -> (u32, u32) {
    let mut range_beg = u64::max_value();
    let mut range_end: u64 = 0;

    let bins_range = reg2bins(start, end);

    for reference in refs.iter() {
        // XXX: Check that this reference corresponds to the chrom we are looking for
        // XXX: Read BAM header to retrieve chrom names (code in htsget-aws from rust-htslib)
        for bin in &reference.bins {
            if bins_range.contains(&(bin.bin_id as u16)) {
                for chunk in bin.chunks.iter() {
                    range_beg = range_beg.min(chunk.chunk_beg);
                    range_end = range_end.max(chunk.chunk_end);
                }
            }
        }
    }

    // Only interested in compressed offset for the final htsget range (request to BAM)
    let (_, start_coffset) = parse_voffset(range_beg);
    let (_, end_coffset) = parse_voffset(range_end);

    (start_coffset, end_coffset)
}

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    const BAI_FILE: &'static [u8] = include_bytes!("../tests/data/htsnexus_test_NA12878.bam.bai");
    //const BAI_FILE: &'static [u8] = include_bytes!("../tests/data/SBJ00154_PRJ190634_LPRJ190634-ready.bam.bai");

    let bai = parse_bai(BAI_FILE);
    let refs = bai.map(|r| r.1.refs)?;

    let range = htsget_query(&refs, 1, 10, 3);
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