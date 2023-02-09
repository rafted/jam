use std::io::{Read, Write};

use crate::encoding::Encodable;

pub mod clientbound;
pub mod serverbound;

pub enum PacketDirection {
    Clientbound,
    Serverbound,
}

pub trait Packet: Encodable
where
    Self: Sized,
{
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self>;
    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()>;
}

impl<P: Packet> Encodable for P {
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self> {
        <Self as Packet>::decode(reader)
    }

    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()> {
        <Self as Packet>::encode(self, writer)
    }
}

pub mod test {
    use std::marker::PhantomData;

    use protocol_macro::PacketDef;

    use crate::encoding::Encodable;

    #[derive(PacketDef)]
    pub struct TestPacket<'a> {
        test: Test<'a>,
    }

    pub struct Test<'a> {
        val: String,
        phantom: PhantomData<&'a str>,
    }

    impl<'a> Encodable for Test<'a> {
        fn decode<T: std::io::Read>(reader: &mut T) -> anyhow::Result<Self> {
            Ok(Test {
                val: String::decode(reader)?,
                phantom: PhantomData,
            })
        }

        fn encode<T: std::io::Write>(&self, writer: &mut T) -> anyhow::Result<()> {
            self.val.encode(writer)?;
            Ok(())
        }
    }

    #[test]
    pub fn encode() {
        let packet = TestPacket {
            test: Test {
                val: String::from("hi"),
                phantom: PhantomData,
            },
        };

        let mut data = Vec::<u8>::new();
        packet.encode(&mut data).unwrap();

        assert_eq!([2, 104, 105], data.as_slice());
    }
}
