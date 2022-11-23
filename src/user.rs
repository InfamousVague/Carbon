use cid::Cid;
use did_key::DIDKey;

pub struct User {
    key: DIDKey,
    username: String,
    status: String,
    photo: Cid,
    banner: Cid,
    meta: Cid,
}

pub trait Identity {
    fn key(&self) -> &DIDKey;

    fn username(&self) -> String;

    fn status(&self) -> String;

    fn photo(&self) -> Cid;

    fn banner(&self) -> Cid;

    fn meta(&self) -> Cid;
}
