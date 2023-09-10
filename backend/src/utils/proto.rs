use prost::Message;

pub fn proto_to_vec<T>(proto: T) -> Vec<u8>
where
    T: Message,
{
    let mut buf = Vec::new();
    buf.reserve(proto.encoded_len());
    proto.encode(&mut buf).unwrap();
    buf
}
