pub struct Group {
    key: DIDKey,
    name: String,
    desc: String,
    photo: Cid,
    meta: Cid,
    members: Vec<User>
}

pub trait Identity {
    fn key(&self) -> &DIDKey;

    fn name(&self) -> String;

    fn desc(&self) -> String;

    fn photo(&self) -> Cid;

    fn meta(&self) -> Cid;

    fn members(&self) -> Vec<User>;
}