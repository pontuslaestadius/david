pub mod doggo;
use doggo::generate::generate;
use doggo::decode::decode;

use std::path::Path;
use std::fs::File;

fn main() {
    // Retrieve input from a file.
    let path = Path::new("resources/input/input.txt");
    let mut file = File::open(&path).unwrap();

    // Decode the file from input to readable data.
    let vec = decode(&mut file);

    // Encode the image with the file data.
    generate(vec);

    // Save the file and display it.

}
