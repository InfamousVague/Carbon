use either::*;

pub mod query;
pub mod message;
pub mod user;
pub mod events;

// Username + Last 4 of DID
pub type UserID = [String; 2];

pub trait Registry {
    /// Search a string partial and return possible identities.
    fn search(&self, p: String) -> Result<Vec<user::User>, std::fmt::Error>;
    /// Exchange a UserID or KeyPair for an identity::Identity.
    fn whois(&self, k: Either<did_key::Ed25519KeyPair, UserID>) -> Result<user::User, std::fmt::Error>;
    /// Will either create a new identity::Identity, or update an existing one. As long as the we have ownership.
    fn set(&self, i: user::User) -> Result<UserID, std::fmt::Error>;
}

pub trait Carbon {
    /// Put a file or message into wherever, return a Cid that can be used to fetch it later.
    fn put(&self, d: Either<std::fs::File, message::Message>, r: Vec<did_key::Ed25519KeyPair>) -> Result<cid::Cid, std::fmt::Error>;
    /// Get a a file or message given a Cid.
    fn get(&self, c: cid::Cid) -> Result<Either<message::Message, std::fs::File>, std::fmt::Error>;
    /// Broadcast that we don't care about a given thing anymore. 
    /// This could mean actually deleting a file, or just relinquishing ownership, un-pinning, etc.
    fn del(&self, c: cid::Cid) -> Option<std::fmt::Error>;
    /// Finds a list of data and or files given a query.
    fn list(&self, q: query::Query) -> Result<Vec<Either<message::Message, std::fs::File>>, std::fmt::Error>;
    /// Stores a file somehow and returns it's Cid.
    fn store(&self, f: std::fs::File, r: Option<Vec<did_key::Ed25519KeyPair>>) -> cid::Cid;
    /// Stream can close with an fmt::Error, or gracefully.
    fn stream<F: Fn(cid::Cid) -> ()>(&self, s: Vec<webrtc_media::track::setting::Media>, r: Vec<did_key::Ed25519KeyPair>, h: F) -> Option<std::fmt::Error>;
    /// Listen to a stream by cid, a closure is expected to handle the media from the stream.
    fn listen<F: Fn(Vec<webrtc_media::track::setting::Media>) -> ()>(&self, c: cid::Cid, h: F) -> Option<std::fmt::Error>;
}