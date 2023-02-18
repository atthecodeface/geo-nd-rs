# Release 0.5.0 (2023-02-18)

- Added multiply_dyn to matrix

- Added serialize/deserialize of types

- Added float 'pi' method

- Added uniform distribution of vectors on a unit sphere

- Added quaternion weighted averaging

- Much addition of inline and must_use

- Moved to Rust 2021 edition

- Added 'into_array' method

- Fixed SqMatrix mulitplication by Self to be matrix multiplication

- Removed SqMatrix division by Self

- Improved library documentation and examples

- Added glsl module, which is not ready yet

# Release 0.1.6 (2023-01-31)

- Improved formatting of vectors

# Release 0.1.4 (2023-01-16)

- Added From<[F;]> implementations for FArray, FArray2, QArray

# Release 0.1.3 (2021-09-06)

- Added the Transform trait, FQArray type, and added Trans to the Geometry3D

- Added look_at to quaternion, and some rotation operations (e.g. apply) and more tests

- Added SqMatrix4 trait methods such as perspective and look_at

# Release 0.1.2 (2021-09-05)

- Added the Quaternion trait, QArray type, and added Quat to the Geometry3D

- There are tests for most of the quaternion operations now

**Contributors**: @atthecodeface

# Release 0.1.0 (2021-06-22)

- Publishing on crates.io for the first time

**Contributors**: @atthecodeface

