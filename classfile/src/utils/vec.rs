
pub unsafe fn uninitialized<T>(size: usize) -> Vec<T> {
    let mut data = Vec::with_capacity(size);
    data.set_len(size);
    data
}
