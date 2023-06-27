
use std::io::{self, };

use read_write_ic::{write_data, read_data};
mod read_write_ic;


fn main() -> io::Result<()> {
    let file_path = "1234.txt";
    let oid = "0xe0e1";

    write_data(file_path, oid)?;
    read_data(oid)?;

    Ok(())
}
