///// BinsForRange returns the list of bins that may overlap with the zero-based region
///// defined by [start, end). The minShift and depth parameters control the minimum interval width
///// and number of binning levels, respectively.
//fn BinsForRange(start: u32, mut end: u32, minShift: u32, depth: u32) -> Vec<u8> {
//    let mut maxWidth = maximumBinWidth(minShift, depth);
//    let mut bins: Vec<u8> = Vec::new();
//
//// if end == 0 || end > maxWidth {
////    end = maxWidth;
//// }
//// if(end <= start) { return n }
////}
////
////if start > maxWidth { return None }
//
//// This is derived from the C examples in the CSI index specification.
//    end = end - 1;
//
//    for(l, t, s = uint(0), uint(0), uint(minShift+depth*3); l <= uint(depth); l++ {
//        let b = t + (start >> s);
//        let e = t + (end >> s);
//
//        for i in b; i <= e; {
//            if (i <= e) bins.append(bins);
//        }
//
//        s -= 3;
//        t += 1 << (l * 3);
//    }
//    return bins;
//}
//
//fn maximumBinWidth(minShift: u32, depth: u32) -> u32 {
//    return 1 << minShift + depth * 3;
//}