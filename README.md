# MD Renderer

A pure Rust game engine using [vulkano](https://crates.io/crates/vulkano) to
safely interact with graphics hardware through the Vulkan API.

## Running the example

Because the engine is in active development, it is recommended to build the
example from source (see instructions below). The release packages may be
significantly behind the current state of the engine, and may not even run on
some platforms, as the build automation scripts have not been extensively
tested. If you wish to download a release, do the following:

1. Download the appropriate example for your platform from the
   [releases page](https://github.com/JMicheli/MD-Render/releases).
2. Unzip the folder and run the binary inside to view the example.

### About the example

The example should display a simple scene containing a high-poly version of the
[Blender monkey mesh](https://docs.blender.org/manual/en/latest/modeling/meshes/primitives.html#monkey)
(her name is Suzanne), an icosphere, a cube, and a ground plane, all lit by one
point light.

The scene provides very rudimentary controls, with the <kbd>W</kbd>,
<kbd>A</kbd>, <kbd>S</kbd>, and <kbd>D</kbd> keys controlling directional
movement along the scene's `x` and `z` basis vectors (_not_ along the camera's
forward axis), and the mouse controlling rotation of the camera when
<kbd>RMB</kbd> is held. The arrow keys can also be used to move the point light
along the same directions as the camera.

This example will continue to develop as more features are added to the engine.

## Compiling from source

Requires a working Rust installation with cargo.

Clone the repository, recursing over submodules.

`git clone --recurse-submodules https://github.com/JMicheli/MD-Render.git`

Navigate into the directory and run the `basic` example with cargo. This will
cause cargo to download the dependencies and build the project.

`cargo run --example basic`

© Joseph W. Micheli 2022, all rights reserved. See
[license.txt](https://github.com/JMicheli/MD-Renderer/blob/main/license.txt) for
further information.
