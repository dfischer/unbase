/*
    SlabRef intends to provide an abstraction for refering to a remote slab.
    Posessing a SlabRef does not confer ownership, or even imply locality.
    It does however provide us with a way to refer to a slab abstractly,
    and a means of getting messages to it.

    I labored a fair bit about whether this is materially different from
    the sender itself, but I think it is important, at least conceptually.
    Also, the internals of the sender could vary dramatically, whereas the
    SlabRef can continue to serve its purpose without material change.
*/

pub mod serde;

use super::*;
use network::{TransportAddress,Transmitter};

use std::ops::Deref;
use std::mem;
use std::fmt;
use std::sync::{Arc,Mutex};

/// A reference to a Slab
///
/// The referenced slab may be resident locally or Remotely
#[derive(Clone)]
pub struct SlabRef(pub Arc<SlabRefInner>);
impl Deref for SlabRef {
    type Target = SlabRefInner;
    fn deref(&self) -> &SlabRefInner {
        &*self.0
    }
}
pub struct SlabRefInner {
    pub slab_id: SlabId,
    pub owning_slab_id: SlabId, // for assertions only?
    pub presence: RwLock<Vec<SlabPresence>>,
    pub tx: Mutex<Transmitter>,
    pub return_address: RwLock<TransportAddress>,
}

impl SlabRef{
    //pub fn new (to_slab_id: SlabId, owning_slab_id: SlabId, presence: Vec<Slab) -> SlabRef {
    //}
    pub fn send (&self, from: &SlabRef, memoref: &MemoRef ) {
        println!("# SlabRef({},{}).send_memo({})", self.owning_slab_id, self.slab_id, memoref.id );

        if let Some(memo) = memoref.get_memo_if_resident() {
            self.tx.lock().unwrap().send(from, memo.clone());
        }else{
            // NOTE: we should actually implement this
            //       it is a totally reasonable use case that we might want to send a memo
            //       to a remote slab that we do not ourselves have
            unimplemented!();
        }
    }

    pub fn get_return_address(&self) -> TransportAddress {
        self.return_address.read().unwrap().clone()
    }
    pub fn apply_presence ( &self, presence: &SlabPresence ) -> bool {
        let mut list = self.presence.write().unwrap();
        for p in list.iter_mut(){
            if p == presence {
                mem::replace(p,presence.clone()); // Update anticipated liftime
                return false;
            }
        }
        list.push(presence.clone());
        return true
    }
    pub fn get_presence(&self) -> Vec<SlabPresence> {
        self.presence.read().unwrap().clone()
    }
    pub fn compare(&self, other: &SlabRef) -> bool {
        // When comparing equality, we can skip the transmitter
        self.slab_id == other.slab_id && *self.presence.read().unwrap() == *other.presence.read().unwrap()
    }
}

impl fmt::Debug for SlabRef {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("SlabRef")
            .field("owning_slab_id", &self.slab_id)
            .field("to_slab_id",     &self.slab_id)
            .field("presence",       &self.presence.read().unwrap())
            .finish()
    }
}

impl Drop for SlabRefInner{
    fn drop(&mut self) {
        println!("# SlabRefInner({},{}).drop",self.owning_slab_id, self.slab_id);
    }
}
