use std::{fmt::Debug, marker::PhantomData};

use bevy_ecs::{
    schedule::{IntoSystemConfigs, Schedule, ScheduleLabel, Schedules},
    system::Resource,
    world::World,
};
use bevy_reflect::Reflect;

/// A Marker [`Resource`] for *something* that has been initialized.
///
/// Usefull if you need to add a system after the app has started but want to ensure that it only happens once (since there is no way to know if the system has already been added otherwise).
///
/// # Examples
///
/// ```
/// # use bevy_init_marker::Initialized;
/// # use bevy::prelude::*;
/// #
/// # let mut world = World::new();
/// # world.init_resource::<Schedules>();
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

impl Initialized<()> {
    /// Initialize the `systems` if they hasn't been initialized for the `schedule` yet.
    ///
    /// See also [`Initialized::init`].
    ///
    /// # Panics
    ///
    /// Panics if the [`Schedules`] resource does not exist int the `world`.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_init_marker::Initialized;
    /// # use bevy::prelude::*;
    /// #
    /// # let mut world = World::new();
    /// # world.init_resource::<Schedules>();
    /// #
    /// fn my_system() {
    ///     // do stuff
    /// }
    ///
    /// if Initialized::init_systems(&mut world, Update, my_system) {
    ///     println!("initialized my_system!");
    /// }
    /// ```
    ///
    /// # Quirks
    ///
    /// ```
    /// # use bevy_init_marker::Initialized;
    /// # use bevy::prelude::*;
    /// #
    /// # let mut app = App::new();
    /// # app.init_resource::<Schedules>();
    /// #
    /// # fn my_system() {}
    /// # fn sys1() {}
    /// # fn sys2() {}
    /// #
    /// // `my_system` will be initialized twice here
    /// app.add_systems(Update, my_system);
    /// Initialized::init_systems(&mut app.world, Update, my_system);
    ///
    /// // `sys1` will be initialized twice here
    /// Initialized::init_systems(&mut app.world, Update, sys1);
    /// Initialized::init_systems(&mut app.world, Update, (sys1, sys2));
    ///
    /// // these are two different systems and both will be initialized
    /// Initialized::init_systems(&mut app.world, Update, || {});
    /// Initialized::init_systems(&mut app.world, Update, || {});
    /// ```
    #[track_caller]
    pub fn init_systems<L, S, Marker>(world: &mut World, schedule: L, systems: S) -> bool
    where
        L: ScheduleLabel,
        S: IntoSystemConfigs<Marker> + Send + Sync + 'static,
    {
        if Initialized::<(L, S)>::init(world) {
            let mut schedules = world.resource_mut::<Schedules>();
            match schedules.get_mut(schedule.intern()) {
                Some(schedule) => {
                    schedule.add_systems(systems);
                }
                None => {
                    let mut schedule = Schedule::new(schedule);
                    schedule.add_systems(systems);
                    schedules.insert(schedule);
                }
            }
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Initialized;
    use bevy::prelude::*;

    #[test]
    fn test_init() {
        let mut world = World::new();
        assert!(Initialized::<()>::init(&mut world));
        assert!(!Initialized::<()>::init(&mut world));
    }

    #[test]
    fn test_init_systems() {
        fn sys1() {}
        fn sys2() {}

        let mut world = World::new();
        world.init_resource::<Schedules>();

        assert!(Initialized::init_systems(&mut world, Update, sys1));
        assert!(!Initialized::init_systems(&mut world, Update, sys1));

        assert!(Initialized::init_systems(&mut world, FixedUpdate, sys1));
        assert!(!Initialized::init_systems(&mut world, FixedUpdate, sys1));

        assert!(Initialized::init_systems(&mut world, Update, (sys1, sys2)));
        assert!(!Initialized::init_systems(&mut world, Update, (sys1, sys2)));
    }

    #[test]
    fn test_init_closure_system() {
        let mut world = World::new();
        world.init_resource::<Schedules>();

        let mut n = 0;
        for _ in 0..2 {
            if Initialized::init_systems(&mut world, Update, || {}) {
                n += 1;
            }
        }
        assert_eq!(n, 1);
    }
}
