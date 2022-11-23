use cid::{Cid, multihash::Multihash};
use either::Either;

use crate::{user::User, message::Message};

pub enum Event {
    /// Identity is typing
    Typing(User),
    /// Identity is calling
    Calling(User),
    /// Identity is calling with video on
    CallingWithVideo(User),
    /// Identity is calling with screen sharing enabled
    CallingWithScreen(User),
    /// Identity muted their mic
    Mute(User),
    /// Identity unmuted their mic
    Unmute(User),
    /// Identity deafened their audio
    Deafen(User),
    /// Identity undeafened their audio
    UnDeafen(User),
    /// Messaage was sent by identity
    Message(Message, User),
    /// Message was marked by identity
    Mark(Message, User),
    /// Message was supplied as a reply to message by identity
    /// Reply, RepliedTo, Sender
    Reply(Message, Message, User),
    /// Reaction supplied to message by identity
    React(String, Message, User),
    /// Group created by identity
    Group(User, User),
    /// File event by identity
    File(Cid, User),
    /// Profile update by identity
    Profile(User),
    /// Status update by identity
    Status(String, User),
    /// Invite to group by identity
    Invite(User, User),
    /// Identity sent friend request to identity
    Friend(User, User),
    /// Identity unfriended identity
    UnFriend(User, User),
    /// Identity blocked identity
    Block(User, User),
}

impl Event {
    pub fn to_hash(&self) -> Multihash {
        // TODO: convert event to CID, just use ms or ns time it's plenty enough for auditable UI events (for now).
        match self {
            Event::Typing(i) |
            Event::Calling(i) |
            Event::CallingWithVideo(i) |
            Event::CallingWithScreen(i) |
            Event::Mute(i) |
            Event::Unmute(i) |
            Event::Deafen(i) |
            Event::Profile(i) |
            Event::UnDeafen(i) => todo!(),
            Event::Message(_, _) => todo!(),
            Event::Mark(_, _) => todo!(),
            Event::Reply(_, _, _) => todo!(),
            Event::React(_, _, _) => todo!(),
            Event::Group(_, _) => todo!(),
            Event::File(_, _) => todo!(),
            Event::Status(_, _) => todo!(),
            Event::Invite(_, _) => todo!(),
            Event::Friend(_, _) => todo!(),
            Event::UnFriend(_, _) => todo!(),
            Event::Block(_, _) => todo!(),
        }
    }
}

pub trait Listen {
    // Listen for events fire fn on event.
    fn four(event: Either<Event, Vec<Event>>, f: fn(Event));
}