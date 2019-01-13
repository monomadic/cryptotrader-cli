type BoxResult<T> = Result<T,Box<std::error::Error>>;

pub fn positions(pairs: Vec<&str>) -> BoxResult<&'static str> {
    Ok("positions")
}
