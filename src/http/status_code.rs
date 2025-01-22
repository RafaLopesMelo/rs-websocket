pub struct StatusCode(u16);

impl StatusCode {
    pub fn new(code: u16) -> StatusCode {
        return StatusCode(code);
    }
}
