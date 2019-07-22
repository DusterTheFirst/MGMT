pub mod protocol;
pub mod read_write;

pub use read_write::PacketReadWriter;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
