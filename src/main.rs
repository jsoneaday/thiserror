use impl_error::MyError;
use this_error::ThisError;

mod impl_error;
mod this_error;

fn main() {
    println!("ServerError {}", ThisError::ServerError);
    println!("validationError {}", ThisError::ValidationError { 
        field_name: "username".to_string(), 
        failure_str: "username cannot be empty".to_string()
    });
    println!("NetworkError {}", ThisError::NetworkError(std::io::Error::new(
        std::io::ErrorKind::ConnectionReset,
        MyError::NetworkError("my custom network error".to_string())
    )));
}
