pub mod gen3;

pub fn get_gen(len: usize) -> &'static str {
    match len {
        131088 => "Gen3",
        _ => "NotImplemented"
    }
}

fn get_gen3_data(bytes: Vec<u8>) {

}