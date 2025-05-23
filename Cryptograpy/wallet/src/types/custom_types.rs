pub struct KeyData<T, U> {
    pub private_key: T,
    pub public_key: U,
}

impl<T, U> KeyData<T, U> {
    pub fn new(private_key: T, public_key: U) -> Self {
        Self {
            private_key,
            public_key,
        }
    }
}
