pub mod scanner;
pub mod token;
pub mod error;
pub mod chunk;
pub mod debug;
pub mod vm;
pub mod compiler;

#[macro_export]
macro_rules! return_err {
    ( $fun:expr ) => {
        {
                match $fun {
                    Err(e) => return Err(e),
                    _ => {}
                }
        }
    };
}
