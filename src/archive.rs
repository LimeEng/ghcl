use flate2::{Compression, read::GzEncoder, write::GzDecoder};
use std::{
    io::{self, Read, Write},
    path::Path,
};

pub fn bundle<P: AsRef<Path> + Clone>(source: P, target: impl Read + Write) -> io::Result<()> {
    let s = source.clone();
    let directory = s
        .as_ref()
        .file_name()
        .ok_or(io::Error::from(io::ErrorKind::InvalidData))?;

    let mut builder = tar::Builder::new(target);
    builder.append_dir_all(directory, source)?;

    builder.finish()
}

pub fn compress<P: AsRef<Path> + Clone>(source: P, target: impl Read + Write) -> io::Result<()> {
    // let mut target = GzEncoder::new(target, Compression::best());
    // let mut file = std::fs::File::open(&source)?;
    // io::copy(&mut file, &mut target)?;
    // target.finish()?;
    // std::fs::remove_file(&source)?;
    // Ok(())
    todo!()
}
