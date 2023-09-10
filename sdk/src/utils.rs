use anyhow::anyhow;

use crate::proto::general::Request;
use anyhow::Result;
use bytes::Bytes;
use prost::Message;

pub(crate) fn proto_to_vec<T>(proto: T) -> Vec<u8>
where
    T: Message,
{
    let mut buf = Vec::new();
    buf.reserve(proto.encoded_len());
    proto.encode(&mut buf).unwrap();
    buf
}

// pub(crate) fn response_to_proto<T>(response: Response) ->
// where
//     T: Message,
// {
//     let mut buf = Vec::new();
//     buf.reserve(proto.encoded_len());
//     proto.encode(&mut buf).unwrap();
//     buf
// }

impl ResponseToProto for Bytes {
    fn proto<T>(&mut self) -> Result<T>
    where
        T: Message + Default,
    {
        let response = Request::decode(self)?;
        match response.success {
            true => {
                let bytes = Bytes::from(response.data);
                let proto = T::decode(bytes)?;
                return Ok(proto);
            }
            false => return Err(anyhow!(format!("{:#?}", response.errors))),
        }
    }
}

pub(crate) trait ResponseToProto {
    fn proto<T>(&mut self) -> Result<T>
    where
        T: Message + Default;
}
