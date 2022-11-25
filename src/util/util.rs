pub type GetSetValue<'a, T> = Box<dyn 'a + FnMut(Option<T>) -> T>;
