use bytes::{Buf, BufMut};

use crate::array::CountedArray;

use super::Encodable;

impl<K, U> Encodable for CountedArray<K, U>
where
    usize: PartialEq<K>,
    K: Encodable + Into<usize> + From<usize>,
    U: Encodable,
{
    fn decode(reader: &mut dyn Buf) -> anyhow::Result<Self> {
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

    fn encode(&self, writer: &mut dyn BufMut) -> anyhow::Result<()> {
        assert!(self.arr.len() == self.len);
        self.len.encode(writer)?;

        for el in self.arr.iter() {
            el.encode(writer)?;
        }

        Ok(())
    }
}
