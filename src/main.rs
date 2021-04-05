use bio_index_formats::parser_bai::parse_bai;
use bio_index_formats::generate_bam::generate_bam;

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    const BAI_FILE: &'static [u8] = include_bytes!("../tests/data/generated/temp_mapped.bam.bai");
    dbg!(parse_bai(BAI_FILE));
    //generate_bam();
    Ok(())
}
