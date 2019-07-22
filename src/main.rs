use std::any::Any;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use serde_json::to_string_pretty;
use downcast_rs::{Downcast, impl_downcast};

pub struct ActorId;

pub struct CantSerde;

#[derive(Serialize, Deserialize, Debug)]
pub struct CanSerde;

#[typetag::serde(tag = "type")]
pub trait Message: Downcast + Send + Sync + 'static {}
impl_downcast!(Message);

// not yet supported (might be eventually, though):
//
// #[typetag::serde]
// impl<T: Any + Send + Sync + Serialize + DeserializeOwned + 'static> Message for T {}

#[macro_export]
macro_rules! impl_message {
    ($ty:ty) => {
        #[typetag::serde]
        impl Message for $ty {}
    }
}

impl_message!(CanSerde);
impl_message!(i32);

pub trait DowncastArc: Sized where
    for<'a> &'a Self: Into<&'a Arc<dyn Message>>
{
    fn downcast_ref<T2: Any + Send + Sync + 'static>(&self) -> Option<&T2> {
        <&Self as Into<&Arc<dyn Message>>>::into(self)
            .as_ref().as_any().downcast_ref::<T2>()
    }
}

impl DowncastArc for Arc<dyn Message> {}

pub fn send(_aid: &Arc<ActorId>, msg: Arc<dyn Message>) {
    println!("{}", to_string_pretty(&*msg).unwrap());
    println!("{:?}", msg.downcast_ref::<i32>());
}

pub fn main() {
    send(&Arc::new(ActorId), Arc::new(CanSerde));
    send(&Arc::new(ActorId), Arc::new(25i32));
    //send(&Arc::new(ActorId), Arc::new(CantSerde));
}
