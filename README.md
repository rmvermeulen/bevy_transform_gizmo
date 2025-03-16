
## Fork for Bevy 0.15 using the Bevy Picking
> This fork is a very rough update of the plugin to be compatible with Bevy 0.15 and the new [Bevy Picking](https://docs.rs/bevy/latest/bevy/picking/index.html) 
It is more or less a Tech Demo.


<div align="center">
    
# Bevy Transform Gizmo


**Simple 3D transform gizmo for bevy**

https://github.com/user-attachments/assets/4fec4e20-9068-4821-a967-72d07562777d


</div>


# Demo

Run a minimal implementation of the gizmo by cloning this repository and running:

```shell
cargo run --example minimal
```

# Features

* Prebuilt transform gizmo appears when you select a designated mesh
* Translation handles (axis, plane, and normal to camera)
* Rotation handles
* Gizmo always renders on top of the main render pass
* Gizmo is always the same size at it moves closer/further from the camera
* **New in this Fork:** Gizmo rotates with the Object

# Usage

See the [minimal](examples/minimal.rs) demo for an example of a minimal implementation.

# License

bevy_transform_gizmo is free and open source! All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
