# Release 0.6.4 (2025-05-06)

- Add quaternion apply3_arr and apply4_arr methods, to be able to operate on arrays of 'F'

# Release 0.6.3 (2025-05-06)

- Use better constants and num_traits (makes some values use Num instead of Float, for example)
- Added LUP decomposition/inversion to matrix operations
- Reduced need of NumAssignOps
- Added mapping_vector_pair_to_vector_pair to quaternions
- Add sum_scaled, set_bbox, update_bbox, expand_bbox to Vector

# Release 0.6.1 (2025-08-26)

- Add mapping_vector_pair_to_vector_pair to quaternion

# Release 0.6.0 (2025-08-15)

- Completely rejig the traits to make them more useful

  The traits are no longer generic on each other; they refer back to Rust arrays
  much more frequently, and this reduces some of the conversion work

  It needs more analysis probably

- Reduced local traits Num and Float to just amalgamate those of num_traits

  This means means dropping Float functions 'pi', 'tau', and
  conversions from isize and fractions; instead values can be created
  with f32.into(), and the constants are in functions 'PI', 'TAU' (and
  more)

# Release 0.5.2 (2024-08-04)

- Fix scaling of matrix by F

- Fix identity matrices

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
