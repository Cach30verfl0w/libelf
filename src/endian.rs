use crate::header::ident::ElfEndian;

impl ElfEndian {
    #[inline]
    pub fn read<T: EndianReader>(&self, slice: &[u8], offset: Option<usize>) -> Option<T> {
        T::read_with_endian(slice, *self, offset)
    }
}

pub trait EndianReader {
    fn read_with_endian(slice: &[u8], endian: ElfEndian, offset: Option<usize>) -> Option<Self> where Self: Sized;
}

macro_rules! impl_endian_reader {
    ($ty: ty) => {
        impl EndianReader for $ty {
            fn read_with_endian(slice: &[u8], endian: ElfEndian, offset: Option<usize>) -> Option<Self> {
                let offset = offset.unwrap_or(0);
                let slice = slice.get(offset..(offset + std::mem::size_of::<Self>())).unwrap();
                match endian {
                    ElfEndian::Big => Some(Self::from_be_bytes(slice.try_into().unwrap())),
                    ElfEndian::Little => Some(Self::from_le_bytes(slice.try_into().unwrap())),
                    _ => None
                }
            }
        }
    }
}

impl_endian_reader!(u16);
impl_endian_reader!(u32);
impl_endian_reader!(u64);