use crate::header::ident::ElfEndian;

pub trait IntoArray<T> {
    fn to_array<const LENGTH: usize>(&self) -> [T; LENGTH];
}

impl<T: Default + Copy> IntoArray<T> for [T] {
    fn to_array<const LENGTH: usize>(&self) -> [T; LENGTH] {
        let mut array = [T::default(); LENGTH];
        array.copy_from_slice(&self);
        return array
    }
}

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
                let slice = &slice[offset..=(offset + std::mem::size_of::<Self>())];
                match endian {
                    ElfEndian::Big => Some(Self::from_be_bytes(slice.to_array())),
                    ElfEndian::Little => Some(Self::from_le_bytes(slice.to_array())),
                    _ => None
                }
            }
        }
    }
}

impl_endian_reader!(u16);
impl_endian_reader!(u32);
impl_endian_reader!(u64);