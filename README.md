Learning Rust through [PNGme: An Intermediate Rust Project](https://jrdngr.github.io/pngme_book/)

- in the Idat chunk, we will not be able to see the raw data because it is required to be compressed using the DEFLATE algorithm.
    - The DEFLATE algorithm is a combination of the LZ77 algorithm and Huffman coding.
    - The LZ77 algorithm is a lossless data compression algorithm that replaces repeated occurrences
         of data with references to a single copy of that data existing earlier in the uncompressed data stream.
    - So for a 1x1 pixel image, the data will be 12 bytes instead of 3 (1 byte for each of the RGB channels).
        - 2 zlib header bytes, (120, 156) and 1 zlib checksum byte (0, 0)
        - 1 byte for the block header (0)
        - 4 bytes the actual data (3 for RGB and 1 for the filter type)
        - 4 for zlib footer (adler32 checksum)
        - 2 bytes for zlib stream termination

## Getting the actual value of the compressed data
```rust
extern crate flate2;
use flate2::read::ZlibDecoder;
use std::io::prelude::*;

fn main() {
    let compressed_data: Vec<u8> = vec![99, 248, 207, 192]; // Your compressed data
    let mut decoder = ZlibDecoder::new(&compressed_data[..]);

    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();

    println!("Decompressed data: {:?}", decompressed_data); // [255, 0, 0] (red pixel)
}
```
