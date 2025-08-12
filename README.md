# geo-nd

A collection of vector, matrix and quaternion types and traits for
Rust, particularly aimed at 2D, 3D and OpenGL/Vulkan applications,
where the elements use arrays and slices of floats.

The usage model in 3D graphics is through ownership of large arrays of
floats by the application, encompassing sets of vertices and matrices,
with the library enabling in-place operations on those vertices and
matrices.

This crate is been in use since 2023, and is relatively stable, but
the API is alpha; it is used in a small number of applications, and
the functionality is mature. 

## Releases

Release notes are available in [RELEASES.md](RELEASES.md).

## License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
