/// Combined enum of errors that occur in queries
#[derive(Debug)]
pub enum QueryError {
    /// Occurs when reading a file
    IO(std::io::Error),
    /// Occurs during deserialization
    CSV(csv::Error),
    /// Occurs when value is not presented in csv file
    NotPresented,
}