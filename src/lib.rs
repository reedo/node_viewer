mod app;
mod file_loading;
pub mod init;
mod xml_parsing;

pub use app::App;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
