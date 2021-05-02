pub struct Ident {
    pub ident: String,
}

pub trait Provider: Send + Sync {
    fn check(&self, token: &String) -> Option<&Ident>;
}
