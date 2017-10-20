#[cfg(unix)]
#[derive(Debug)]
pub struct ReadAtFile
{
	file : ::std::fs::File,
}
#[cfg(unix)]
impl ReadAtFile
{
	pub fn new(f : ::std::fs::File) -> ReadAtFile
	{
		ReadAtFile { file: f }
	}
	pub fn read_at(&self, buf: &mut [u8], offset: u64)
		-> ::std::io::Result<usize>
	{
		::std::os::unix::fs::FileExt::read_at(&self.file, buf, offset)
	}

}
#[cfg(windows)]
#[derive(Debug)]
pub struct ReadAtFile
{
	file : ::std::sync::Mutex<::std::fs::File>,
}
#[cfg(windows)]
impl ReadAtFile
{
	pub fn new(f : ::std::fs::File) -> ReadAtFile
	{
		ReadAtFile { file: ::std::sync::Mutex::new(f) }
	}
	pub fn read_at(&self, buf: &mut [u8], offset: u64)
		-> ::std::io::Result<usize>
	{
		::std::os::windows::fs::FileExt::seek_read(
			&*self.file.lock().map_err(|x| x.into_inner()).unwrap(),
			buf, offset
		)
	}
}

