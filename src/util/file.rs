use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub fn read_bytes(
    reader: &mut BufReader<impl Read>,
    buffer: &mut [u8],
    n_bytes: usize,
) -> Result<(), io::Error> {
    let bytes_read = reader.read(buffer)?;
    if bytes_read != n_bytes {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Expected to read {n_bytes} bytes but instead only read {bytes_read}"),
        ));
    }

    Ok(())
}
