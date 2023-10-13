use crate::header::ident::ElfIdent;

pub mod ident;

pub struct FileHeader {
    pub(crate) ident: ElfIdent
}

impl FileHeader {
    #[inline]
    pub fn ident(&self) -> &ElfIdent {
        &self.ident
    }
}