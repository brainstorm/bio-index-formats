use bio_index_formats::parser_bai::{parse_bai, parse_voffsets, VirtualOffset};

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    const BAI_FILE: &'static [u8] = include_bytes!("../tests/data/htsnexus_test_NA12878.bam.bai");
    //const BAI_FILE: &'static [u8] = include_bytes!("../tests/data/SBJ00154_PRJ190634_LPRJ190634-ready.bam.bai");

    let bai = parse_bai(BAI_FILE);
    let a_chunk = bai.map(|r| r.1.refs)
                               .map(|b| b[10].clone().bins)
                               .map(|c| c[1].clone().chunks)
                               .map(|cp| cp[0].clone())?;

    dbg!(parse_voffsets(a_chunk.chunk_beg));
    dbg!(parse_voffsets(a_chunk.chunk_end));

    Ok(())
}