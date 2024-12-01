use crate::data::Data;
use crate::datastore::DBIndex;
use bincode;
use std::fs::{File, OpenOptions};
use std::io::{
    BufRead, BufReader, BufWriter, Error, ErrorKind, Read, Result, Seek, SeekFrom, Write,
};

/// This function returns the last offset data was stored at
fn get_log_offset(file_path: &str) -> Result<i64> {
    let mut file: File = OpenOptions::new()
        .read(true)
        .open(file_path)?;

    let mut content: String = String::new();
    file.read_to_string(&mut content)?;

    let offset: i64 = content.trim().parse().unwrap_or(0);

    Ok(offset)
}

/// This function is to update the last offset data was stored at
fn incr_log_offset(file_path: &str) -> Result<()> {
    let mut file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)?;

    let mut content: String = String::new();
    file.read_to_string(&mut content)?;

    let mut offset: i64 = content.trim().parse().unwrap_or(0);
    offset += 1;

    // seek to beginning of the file to overwrite content
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;

    write!(file, "{}", offset)?;

    Ok(())
}

/// This function returns a list of DB Index objects
fn get_log_indexes(file_path: &str) -> Result<Vec<DBIndex>> {
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);

    let indexes: Vec<DBIndex> = bincode::deserialize_from(reader)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    Ok(indexes)
}

/// This function is to append a DB Index object to the index log file
fn append_log_index(file_path: &str, index: &DBIndex) -> Result<()> {
    let file: File = OpenOptions::new()
        .append(true)
        .open(file_path)?;

    let mut writer: BufWriter<File> = BufWriter::new(file);

    let serialized_index = bincode::serialize(index)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    writer.write_all(&serialized_index)?;
    writer.write_all(b"\n")?;

    Ok(())
}

/// This function is to append data to the db log file
pub fn append_data_to_log(file_path: &str, data: &Data) -> Result<()> {
    let file: File = OpenOptions::new()
        .append(true)
        .open(file_path)?;

    let mut writer: BufWriter<File> = BufWriter::new(file);

    let serialized_data = bincode::serialize(data)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    writer.write_all(&serialized_data)?;
    writer.write_all(b"\n")?;

    Ok(())
}

pub fn find_data_in_log(file_path: &str, search_key: &str) -> Result<Option<Data>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?; // read line as string
        let bytes = line.as_bytes(); //convert to byte

        let data: Data = bincode::deserialize(bytes)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        if search_key == data.key {
            return Ok(Some(data));
        }
    }

    Ok(None)
}
