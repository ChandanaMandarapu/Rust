// custom error types
use std::fmt
#[derive(Debug)]

enum MathError{
        DivisionByZero,
        NegativeSquareRoot,
        Overflow,
}

enum FileError {
    NotFound(String),
    PermissionDenied(String),
    TooLarge{filename:String,size_mb:u64,limit_mb:u64},
}
impl fmt::Display for FileError{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        match self{
            FileError::NotFound(filename) => {
                write!(f,"filenotfount : {}",filename)
            }
            FileError::PermissionDenied(filename) => {
                write!(f,"permission denied when accessing : {}",filename);
            }
            FileError::TooLarge { filename, size_mb, limit_mb } => {
                write!(f, "File {} is too large ({} MB). Maximum allowed is {} MB",
                       filename, size_mb, limit_mb)
            }
        }
    }
}

let error = FileError::TooLarge{
    filename : String::from("video.mp4");
    size_mb: 500,
    limit_mb:100,
}

println!("{}",error);
println!("{:?}",error);
fn divide(a:f64, b:f64) -> Result<f64, MathError> {
    if b==0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok (a/b)
    }
}

fn square_root(n:f64) -> Result<f64, MathError> {
    if n<0.0>{
        Err(MathError::NegativeSquareRoot)
    }
    else{
        Ok(n.sqrt())
    }
}
