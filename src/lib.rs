use std::marker::PhantomData;

use bevy_ecs::system::Resource;
use bevy_reflect::Reflect;

/// A Marker [`Resource`] for "something" that has been initialized.
///
/// Usefull if you need to add a system at runtime but want to ensure that it only happens once (since there is no way to know if the system has already been added).
#[derive(Resource, Reflect, Debug, Clone, Copy)]
pub struct Initialized<M>(PhantomData<M>);

impl<M> Default for Initialized<M> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
