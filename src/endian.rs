use crate::header::ident::ElfEndian;

impl ElfEndian {
    #[inline]
    pub fn read<T: EndianReader>(&self, slice: &[u8], offset: Option<&mut usize>) -> Option<T> {
        T::read_with_endian(slice, *self, offset)
    }
}

pub trait EndianReader {
    fn read_with_endian(slice: &[u8], endian: ElfEndian, offset: Option<&mut usize>) -> Option<Self> where Self: Sized;
}

macro_rules! impl_endian_reader {
    ($ty: ty) => {
        impl EndianReader for $ty {
            fn read_with_endian(slice: &[u8], endian: ElfEndian, offset: Option<&mut usize>) -> Option<Self> {
                const SELF_SIZE: usize = crate::std::mem::size_of::<$ty>();

                let offset_usize = offset.as_ref().map(|value| **value).unwrap_or(0);
                let slice = slice.get(offset_usize..(offset_usize + SELF_SIZE)).unwrap();
                if let Some(offset) = offset {
                    *offset += SELF_SIZE;
                }

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
