pub fn byte_counter<T>(arg: T) -> usize
where
    T: AsRef<[u8]>,
{
    arg.as_ref().len()
}

pub fn char_counter<T>(arg: T) -> usize
where
    T: AsRef<str>,
{
    arg.as_ref().chars().count()
}
