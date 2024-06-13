use std::{fmt::Debug, marker::PhantomData};

use bevy_ecs::{
    schedule::{IntoSystemConfigs, ScheduleLabel, Schedules},
    system::Resource,
    world::World,
};
use bevy_reflect::Reflect;

/// A Marker [`Resource`] for *something* that has been initialized.
///
/// Usefull if you need to add a system after the app has started but want to ensure that it only happens once (since there is no way to know if the system has already been added).
///
/// # Examples
///
/// ```rust
/// # use bevy_init_marker::Initialized;
/// # use bevy::prelude::*;
/// #
/// # let mut world = World::new();
/// #
/// struct MyMarker;
///
/// if Initialized::<MyMarker>::init(&mut world) {
///     // do stuff once
/// }
///
/// fn my_system() {
///     // do stuff
/// }
///
/// if Initialized::init_systems(&mut world, Update, my_system) {
///     println!("initialized my_system!");
/// }
/// ```
#[derive(Resource, Reflect)]
pub struct Initialized<M: Send + Sync + 'static>(PhantomData<M>);

impl<M: Send + Sync + 'static> Debug for Initialized<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Initialized<{}>", std::any::type_name::<M>())
    }
}

impl<M: Send + Sync + 'static> Clone for Initialized<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M: Send + Sync + 'static> Copy for Initialized<M> {}

impl<M: Send + Sync + 'static> Default for Initialized<M> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<M: Send + Sync + 'static> Initialized<M> {
    /// Initializes the `Initialized<M>` resource if it hasn't been initialized yet.
    ///
    /// Returns `true` if the resource was not previously initialized, `false` otherwise.
    ///
    /// See also [`Initialized::init_system`].
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_init_marker::Initialized;
    /// # use bevy::prelude::*;
    /// #
    /// # let mut world = World::new();
    /// #
    /// struct MyMarker;
    ///
    /// if Initialized::<MyMarker>::init(&mut world) {
    ///     // do stuff once
    /// }
    /// ```
    #[must_use]
    pub fn init(world: &mut World) -> bool {
        if !world.contains_resource::<Self>() {
            bevy_log::trace!("Initialized `{}`", std::any::type_name::<M>());
            world.init_resource::<Self>();
            true
        } else {
            false
        }
    }
}

impl<Marker, S> Initialized<(S, Marker)>
where
    S: IntoSystemConfigs<Marker> + Send + Sync + 'static,
    Marker: Send + Sync + 'static,
{
    /// Initialize the `systems` if it hasn't been initialized for the `schedule` yet.
    ///
    /// This will not work as expected if the `systems` have already been added to the `schedule` by other means.
    ///
    /// See also [`Initialized::init`].
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_init_marker::Initialized;
    /// # use bevy::prelude::*;
    /// #
    /// # let mut world = World::new();
    /// #
    /// # let mut schedule = Schedule::default();
    /// # let mut schedule = schedule.add_systems(init_system::<MyMarker>);
    /// #
    /// fn my_system() {
    ///     // do stuff
    /// }
    ///
    /// if Initialized::init_systems(&mut world, Update, my_system) {
    ///     println!("initialized my_system!");
    /// }
    /// ```
    #[track_caller]
    pub fn init_systems<L: ScheduleLabel>(world: &mut World, schedule: L, systems: S) -> bool {
        if Initialized::<(L, S)>::init(world) {
            let mut schedules = world.resource_mut::<Schedules>();
            let schedule = schedules
                .get_mut(schedule.intern())
                .unwrap_or_else(|| panic!("Schedule `{schedule:?}` not found"));
            schedule.add_systems(systems);
            return true;
        }
        false
    }
}
