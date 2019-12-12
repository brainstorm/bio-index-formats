use bio_index_formats::parser_bai::parse_bai;

fn main() {
    const BAI_FILE: &'static [u8] = include_bytes!("../tests/data/htsnexus_test_NA12878.bam.bai");

    parse_bai(BAI_FILE).map_err(|err| println!("{:?}", err));
}