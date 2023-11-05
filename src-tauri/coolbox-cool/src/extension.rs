pub trait StringExt {
    fn truncate_string(&self, max_len: usize) -> String;

    fn render(&self, context: &tera::Context, auto_escape: bool) -> tera::Result<String>;
}

impl<T> StringExt for T
where
    T: AsRef<str>,
{
    fn truncate_string(&self, max_len: usize) -> String {
        if self.as_ref().len() <= max_len {
            self.as_ref().to_string()
        } else {
            format!("{}...", &self.as_ref()[..max_len - 3]) // 为"..."留出3个字符的空间
        }
    }

    fn render(&self, context: &tera::Context, auto_escape: bool) -> tera::Result<String> {
        tera::Tera::one_off(self.as_ref(), context, auto_escape)
    }
}
