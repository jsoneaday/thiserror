use impl_error::MyError;

mod impl_error;

fn main() {
    println!("Show me server: error {}", MyError::ServerError);
    println!("Show me validation: error {}", MyError::ValidationError { field_name: "username".to_string(), failure_str: "username cannot be empty".to_string() });
    println!("Show me server error: {}", MyError::NetworkError("127.0.0.1".to_string()));
}
