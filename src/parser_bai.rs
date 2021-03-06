use nom::{ IResult };
use nom::bytes::streaming::{ tag };
use nom::number::streaming::{ le_u32, le_u64 };
use nom::multi::many_m_n;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct BAI {
    pub magic: String,
    //pub n_refs: u32, # on spec but already present implicitly in Vec<Ref>.size().
    pub refs: Vec<Ref>,
    pub n_no_coor: u64,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Ref {
    //pub n_bins: u32, # already on Vec<Bin>.size(), no need to store it separatedly
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

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct VirtualOffset {
    pub coffset: u32,
    pub uoffset: u32,
}


pub fn parse_bai(input: &[u8]) -> IResult<&[u8], BAI> {
    let (input, magic) = parse_magic(input)?;
    let (input, n_refs) = le_u32(input)?;
    let (input, refs) = many_m_n(0, n_refs as usize, parse_refs)(input)?;
    let (input, n_no_coor) = le_u64(input)?;
    Ok((input, BAI { magic, refs, n_no_coor }))
}

pub fn parse_refs(input: &[u8]) -> IResult<&[u8], Ref> {
    let (input, n_bins) = le_u32(input)?;
    // "List of distinct bins (n=n_bin)" on SAMv1 spec
    let (input, bins) = many_m_n(0, n_bins as usize, parse_bins)(input)?;
    // # 16kbp intervals (for the linear index)
    let (input, n_intv) = le_u32(input)?;
    let (input, intervals) = many_m_n(0, n_intv as usize, parse_intervals)(input)?;

    Ok((input, Ref { bins, intervals }))
}

pub fn parse_intervals (input: &[u8]) -> IResult<&[u8], u64> {
    let (input, ioffset) = le_u64(input)?;
    Ok((input, ioffset))
}

pub fn parse_bins(input: &[u8]) -> IResult<&[u8], Bin> {
    let (input, bin_id) = le_u32(input)?;
    let (input, n_chunk) = le_u32(input)?;
    let (input, chunks) = many_m_n(0, n_chunk as usize, parse_chunks)(input)?;

    Ok((input, Bin { bin_id, chunks }))
}

pub fn parse_chunks(input: &[u8]) -> IResult<&[u8], ChunkPos> {
    let (input, chunk_beg) = le_u64(input)?;
    let (input, chunk_end) = le_u64(input)?;

    Ok((input, ChunkPos { chunk_beg, chunk_end }))
}

pub fn parse_voffset(voffset: u64) -> (u32, u32) {
    (coffset(voffset), uoffset(voffset))
}

pub fn coffset(voffset: u64) -> u32 {
    ((voffset >> 16) & 0xffff) as u32
}

pub fn uoffset(voffset: u64) -> u32 {
    (voffset & 0xffff ) as u32
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
