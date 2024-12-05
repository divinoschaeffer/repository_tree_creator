use std::io;
use std::io::Read;
use std::path::PathBuf;

use dit_file_encryptor::CompressedFile;

/// **Description**  
/// This function locates and opens an object file based on the given object ID and object path. 
/// It splits the object ID into two parts: the first two characters as the directory name 
/// and the remaining characters as the file name. It then constructs the full path and attempts to open the file.
///
/// **Parameters**  
/// - `object_id`: A reference to a `String` containing the unique identifier of the object.  
/// - `object_path`: A reference to a `PathBuf` representing the base path to the object files.  
///
/// **Returns**  
/// - `Result<File, io::Error>`:  
///   - `Ok(File)` if the file is found and successfully opened.  
///   - `Err(io::Error)` if the file or its directory does not exist.  
pub fn open_object_file(object_id: &String, object_path: &PathBuf) -> Result<Box<dyn Read>, io::Error> {
    let b_hash = &object_id[..2];
    let e_hash = &object_id[2..];

    let object_dir = object_path.join(b_hash);
    if object_dir.exists() {
        let object_file = object_dir.join(e_hash);
        if object_file.exists() {
            let reader = CompressedFile::new(object_file)
                .open_for_read()
                .map_err(|_| {
                    io::ErrorKind::InvalidData
                })?;
            return Ok(reader);
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "Error file not found in objects: {object_id}"))
}