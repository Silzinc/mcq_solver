pub fn vec_from_fn<T, F: FnMut() -> T>(size: usize, f: F) -> Vec<T> {
    let mut vector = Vec::<T>::with_capacity(size);
    unsafe {vector.set_len(size);}
    vector.fill_with(f);
    vector
}