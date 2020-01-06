//use crate::csi::{ bins_for_range };

use nom::{IResult};
//use nom::bytes::streaming::{tag, take};
//use nom::number::streaming::{le_u32};
use nom::bytes::complete::{ tag };
use nom::number::complete::{ le_u32, le_u64 };


#[derive(Clone,Debug,PartialEq,Eq)]
pub struct BAI {
    pub magic: String,
    //pub n_refs: u32, # already on Vec<Ref>.size(), no need to store it
    pub refs: Vec<Ref>,
    pub n_no_coor: u64,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Ref {
    //pub n_bins: u32, # already on Vec<Bin>.size(), no need to store it
    pub bins: Vec<Bin>,
    pub intervals: Vec<u64>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Bin {
    pub bin_id: u32,
    //pub n_chunk: u32, # already on Vec<ChunkPos>.size, no need to store it
    pub chunks: Vec<ChunkPos>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ChunkPos {
    pub chunk_beg: u64,
    pub chunk_end: u64,
}

pub struct VirtualOffset {
    /// compressed offset
    pub coffset: u32,
    /// uncompressed offset
    pub uoffset: u32,
}

pub fn parse_bai(input: &'static[u8]) -> IResult<&[u8], BAI> {
    let (input, magic) = parse_magic(input)?;
    let (input, n_refs) = le_u32(input)?;

    // "List of indices (n=n_ref)" section on SAMv1 spec
    // https://samtools.github.io/hts-specs/SAMv1.pdf
    let mut refs = Vec::<Ref>::with_capacity(n_refs as usize);
    let mut input2 = input; // XXX: no need to copy input on upper scope
    for _ in 0..n_refs {
        let (input, aref) = parse_refs(input2)?;
        refs.push(aref);
        input2 = input;
    }

    let (input, n_no_coor) = le_u64(input)?; // XXX: Should perhaps not be here?
    Ok((input, BAI { magic, refs, n_no_coor }))
}

pub fn parse_refs(input: &[u8]) -> IResult<&[u8], Ref> {
    //bins_for_range(region.start, region.end, 14,5);

    let (input, n_bins) = le_u32(input)?;

    // "List of distinct bins (n=n_bin)" on SAMv1 spec
    let mut bins = Vec::<Bin>::with_capacity(n_bins as usize);
    let mut input2 = input; // XXX: no need to copy input on upper scope
    for _ in 0..n_bins {
        let (input, bin) = parse_bins(input2)?;
        bins.push(bin);
        input2 = input;
    }

    // # 16kbp intervals (for the linear index)
    let (input, n_intv) = le_u32(input)?;

    let mut intervals = Vec::<u64>::with_capacity(n_intv as usize);
    let mut input2 = input; // XXX: no need to copy input on upper scope
    let (input, n_intv) = le_u32(input)?;
    for _ in 0..n_intv {
        let (input, interval) = parse_intervals(input2)?;
        intervals.push(interval);
        input2 = input;
    }

    Ok((input, Ref { bins, intervals }))
}

pub fn parse_intervals (input: &[u8]) -> IResult<&[u8], u64> {
    let (input, ioffset) = le_u64(input)?;
    Ok((input, ioffset))
}

pub fn parse_bins(input: &[u8]) -> IResult<&[u8], Bin> {
    let (input, bin_id) = le_u32(input)?;
    let (input, n_chunk) = le_u32(input)?;
    let mut chunks = Vec::<ChunkPos>::with_capacity(n_chunk as usize);
    let mut input2 = input; // XXX: no need to copy input on upper scope
    for _ in 0..n_chunk {
        let (input, chunk) = parse_chunks(input2)?;
        chunks.push(chunk);
        input2 = input;
    }
    Ok((input, Bin { bin_id, chunks }))
}

pub fn parse_chunks(input: &[u8]) -> IResult<&[u8], ChunkPos> {
    let (input, chunk_beg) = le_u64(input)?;
    let (input, chunk_end) = le_u64(input)?;

    //let offsets_beg = from_offset(chunk_beg);
    let offsets_end = from_offset(chunk_end);
    //println!("{}", offsets_beg.coffset);
    //println!("{}", offsets_end.coffset);

    Ok((input, ChunkPos { chunk_beg, chunk_end }))
}

fn from_offset(offsets: u64) -> VirtualOffset {
    let coffset = ((offsets >> 16) & 0xffff) as u32;
    let uoffset = (offsets & 0xffff ) as u32;
    VirtualOffset { coffset, uoffset }
}

pub fn parse_magic(input: &[u8]) -> IResult<&[u8], String> {
    let (input, magic) = tag("BAI\x01")(input)?;
    Ok((input, String::from_utf8_lossy(magic).to_string()))
}


#[cfg(test)]
mod tests {
    use super::*;

    const BAI_FILE: &'static [u8] = include_bytes!("../tests/data/htsnexus_test_NA12878.bam.bai");

    #[test]
    fn magic_test() {
        let field = "BAI\x01";
        let res = parse_magic(BAI_FILE);
        match res {
            Ok((_, output)) => assert_eq!(field, output),
            _ => assert!(false)
        }
    }

    #[test]
    fn chunks_test() {
        let input = vec![
            0x04, 0x03, 0x02, 0x01, // bin_id
            0x02, 0x00, 0x00, 0x00, // n_bins
                // chunk_1
                0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, // chunk_beg
                0x28, 0x27, 0x26, 0x25, 0x24, 0x23, 0x22, 0x21, // chunk_end
                // chunk_2
                0x38, 0x37, 0x36, 0x35, 0x34, 0x33, 0x32, 0x31, // chunk_beg
                0x48, 0x47, 0x46, 0x45, 0x44, 0x43, 0x42, 0x41, // chunk_end
        ];

        let res = parse_bins(&input);
        match res {
            Ok((_, output)) => {
                assert_eq!(output.bin_id, 0x01020304);
                assert_eq!(output.chunks.len(), 2);

                let expected_chunks = vec![
                    ChunkPos {
                        chunk_beg: 0x1112131415161718,
                        chunk_end: 0x2122232425262728,
                    },
                    ChunkPos {
                        chunk_beg: 0x3132333435363738,
                        chunk_end: 0x4142434445464748,
                    }

                ];
                assert_eq!(output.chunks, expected_chunks)
            }
            _ => assert!(false)
        }
    }
}