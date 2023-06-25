use std::fs::File;
use std::path::Path;
use std::io;
use std::io::Read;
use blake3::{Hash, Hasher};

/// Hasher
///
/// Gets the hash of a file by reading into a buffer.
/// This is used so that files that are potentially ultra large can still be read without your RAM collapsing.
pub(crate) fn hasher(path: &Path) -> io::Result<Hash> {
	// Open file and get metadata
	let mut f = File::open(path)?;
	let fsize = f.metadata()?.len();
	
	// Start Hashing
	let mut hasher = Hasher::new();
	let bsize = buffer_size(fsize);
	let mut buffer = vec![0; bsize];

	loop {
        let bytes_read = f.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    };
	Ok(hasher.finalize())
}

/// Buffer Size
/// 
/// Selects a buffer size depending on the file size.
fn buffer_size(file_size: u64) -> usize {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if file_size <= 512 * MB {
        128 * KB as usize
	} else if file_size <= 2 * GB {
		32 * MB as usize
    } else if file_size <= 6 * GB {
        192 * MB as usize
    } else {
        512 * MB as usize
    }
}