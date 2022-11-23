use std::time::SystemTime;

use did_key::DIDKey;
use either::Either;

use crate::events::Event;

pub struct Query {
    kind: Either<Event, Vec<Event>>,
    before: SystemTime,
    after: SystemTime,
    sender: Either<DIDKey, Vec<DIDKey>>,
    recipient: Either<DIDKey, Vec<DIDKey>>,
}
