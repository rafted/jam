use std::io::{Read, Write};

use crate::array::CountedArray;

use super::Encodable;

impl<K, U> Encodable for CountedArray<K, U>
where
    usize: PartialEq<K>,
    K: Encodable + Into<usize> + From<usize>,
    U: Encodable,
{
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self> {
        let mut vec = Vec::<U>::new();
        let len: usize = K::decode(reader)?.into();

        for _ in 0..len {
            vec.push(U::decode(reader)?);
        }

        Ok(CountedArray {
            len: len.into(),
            arr: vec,
        })
    }

    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()> {
        assert!(self.arr.len() == self.len);
        self.len.encode(writer)?;

        for el in self.arr.iter() {
            el.encode(writer)?;
        }

        Ok(())
    }
}
