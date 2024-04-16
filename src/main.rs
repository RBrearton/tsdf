// Declare the project structure.
pub(crate) mod core;

use std::path::Path;

use crate::core::structs::TsdfFile;
use crate::core::traits::TsdfFileTrait;

fn main() {
    // Make a path to a test file.
    let path = Path::new("test.tsdf");

    // Create a new TsdfFile overwriting writer.
    let tsdf_file =
        TsdfFile::new_overwriting_writer(path, None, None).expect("Failed to create file.");

    // Print the version of the file, the write mode, the path, the size, and the file format.
    println!(
        "Version: {}\nIoMode: {:?}\nPath: {:?}\nSize: {}\nFileFormat: {:?}",
        tsdf_file.get_version(),
        tsdf_file.get_io_mode(),
        tsdf_file.get_path(),
        tsdf_file.get_size(),
        tsdf_file.get_file_format()
    );
}
