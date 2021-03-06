pub enum Line {

}
pub enum ScannerError {

}
struct Scanner<'a> {
	content: &'a str,
	position: usize,
}
impl<'a> Scanner<'a> {
	pub fn new(content: &str) -> Scanner {
		Scanner {
			content,
			position: 0
		}
	}
	pub fn rest(&self) -> &str {
		&self.content[self.position..]
	}
	pub fn next_line(&mut self) -> Option<Result<Line, ScannerError>> {
		let test = self.rest().split(char::is_whitespace);
		None
	}
}