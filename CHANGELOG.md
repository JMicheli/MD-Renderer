# Changelog

New versions of MD Renderer in descending chronological order.

## [0.3.0](https://github.com/JMicheli/MD-Render/releases/tag/release-v0.3.0) - 2026-05-25

This release updates the project to use the current latest version of Vulkano
(v0.35.2) and updates other dependencies as well.

As a result of changes in winit, this release reworks the way that engine
applications are defined. Instead of providing a closure, the user now
implements the MdrApplication trait to provide handlers the initialization,
update, and shutdown stages of the application's lifecycle. The Basic example
has been adjusted to use this API.

## [0.2.0](https://github.com/JMicheli/MD-Render/releases/tag/release-v0.2.0) - 2022-08-14

Provides a more advanced example with fully-implemented Blinn-Phong shading and
textures. The example allows basic FPS-style movement using the WASD keys and
mouse. The right mouse button must be held to change the camera rotation.

## [0.1.0](https://github.com/JMicheli/MD-Render/releases/tag/release-v0.1.0) - 2022-06-28

This is the initial public release of the application and includes a very basic
example. The example allows rotating/pitching the camera with the arrow keys,
and implements basic diffuse shading.
