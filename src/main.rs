use bio_index_formats::parser_bai::parse_bai;

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    //const BAI_FILE: &'static [u8] = include_bytes!("../tests/data/htsnexus_test_NA12878.bam.bai");
    const BAI_FILE: &'static [u8] = include_bytes!("../tests/data/SBJ00154_PRJ190634_LPRJ190634-ready.bam.bai");

    println!("{:?}", parse_bai(BAI_FILE));
    Ok(())
}