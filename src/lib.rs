use std::marker::PhantomData;

use bevy_ecs::system::Resource;

#[derive(Resource, Debug, Clone, Copy)]
pub struct Initialized<M>(PhantomData<M>);

impl<M> Default for Initialized<M> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
