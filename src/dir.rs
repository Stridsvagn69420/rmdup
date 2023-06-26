use std::io;
use std::env;
use std::fs;
use std::path::PathBuf;

/// List Current Working Directory
/// 
/// 1. [Get](env::current_dir) CWD
/// 2. [List](fs::read_dir) CWD
/// 3. Filter for successful [DirEntry](fs::DirEntry)s
/// 4. Filter for files
/// 5. Map to [PathBuf] of files
/// 6. Collect to [Vec]
pub(crate) fn list_cwd() -> io::Result<(PathBuf, Vec<PathBuf>)> {	
	let cwd = env::current_dir()?;

	let fpaths = fs::read_dir(&cwd)?
	.filter_map(|x| x.ok())
	.filter(|e| e.metadata().is_ok_and(|m| m.is_file()))
	.map(|f| f.path())
	.collect();

	Ok((cwd, fpaths))
}