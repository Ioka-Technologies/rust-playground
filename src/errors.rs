pub mod base;
mod io_error;

pub fn test_error(file_name: String) -> Result<(), base::Error> {
    let file = std::fs::File::open(file_name)?;

    println!("File opened: {:?}", file);

    Ok(())
}