use impl_error::MyError;
use this_error::ThisError;

mod impl_error;
mod this_error;

fn main() {
    println!("Server Error: {}", ThisError::ServerError);
    println!("Validation Error: {}", ThisError::ValidationError { field_name: "username".to_string(), failure_str: "username cannot be empty".to_string() });
    println!("Network Error: {}", ThisError::NetworkError(std::io::Error::new(std::io::ErrorKind::ConnectionReset, MyError::NetworkError("network failed".to_string()))))
}
