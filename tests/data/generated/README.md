# Quick tests

```
$ samtools view temp_mapped.bam

0000000000000000	97	chr1	1	60	100M	=	201	300	AAAGGGGCCCCACATTATCTCGGCTGAGAGTACCACTGGGAGTTGCATACCTCGCTCGACCTGAAAATGGACCCTCCCTTCCGGATCAAAAGCGGGCATA	````````````````````````````````````````````````````````````````````````````````````````````````````	RG:Z:A	MQ:A:<	MC:Z:100M
0000000000000000	145	chr1	201	60	100M	=	1	300	GGCAGACCACGCGGTTGGTCGACTAAGGGCAGCCCCGAGCTAATATCTCACGCGTCGCACTTAAGGCACTAAGTACCGTTCCGTGGCTATGACCGCTGCT	````````````````````````````````````````````````````````````````````````````````````````````````````	RG:Z:A	MQ:A:<	MC:Z:100M
```

The corresponding `.bam.bai`, generated with regular `samtools` after bam-building the `.bam` from code...:

```
$ samtools index temp_mapped.bam
```

...looks like the following when running `cargo run` and seeing the contents of the `.BAI` parsed by my NOM parser:

```
[src/main.rs:6] parse_bai(BAI_FILE) = Ok(
    (
        [],
        BAI {
            magic: "BAI\u{1}",
            refs: [
                Ref {
                    bins: [
                        Bin {
                            bin_id: 4681,
                            chunks: [
                                ChunkPos {
                                    chunk_beg: 13697024,
                                    chunk_end: 25624576,
                                },
                            ],
                        },
                        Bin {
                            bin_id: 37450,
                            chunks: [
                                ChunkPos {
                                    chunk_beg: 13697024,
                                    chunk_end: 25624576,
                                },
                                ChunkPos {
                                    chunk_beg: 2,
                                    chunk_end: 0,
                                },
                            ],
                        },
                    ],
                    intervals: [
                        13697024,
                    ],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
                Ref {
                    bins: [],
                    intervals: [],
                },
            ],
            n_no_coor: 0,
        },
    ),
)
```
