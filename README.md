A marker `Resource` for *something* that has been initialized.

Usefull if you need to add a system after the app has started but want to ensure that it only happens once (since there is no way to know if the system has already been added otherwise).

## Example

```rust
use bevy::prelude::*;
use bevy_init_marker::Initialized;

let mut world = World::new();

struct MyMarker;

if Initialized::<MyMarker>::init(&mut world) {
    // do stuff once
}
```

```rust
fn my_system() {
    // do stuff
}

if Initialized::init_systems(&mut world, Update, my_system) {
    println!("initialized my_system!");
}

if Initialized::init_systems(&mut world, Update, my_system) {
    unreachable!("my_system has already been initialized in Update so this will never run");
}
```
