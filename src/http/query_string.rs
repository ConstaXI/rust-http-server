use std::collections::HashMap;

pub enum Value<'buf> {
    Single(&'buf str), 
    Multiple(Vec<&'buf str>),
}

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}