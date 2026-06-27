# Changelog

Changes in **fphics**

## [Version 0.2.0] - 2026-06-27

## Changed

- Time calculation in `SuvatOps1D` was fixed to give accurate time values

- Added check for acceleration to return a `DivisionByZero` error if acceleration is zero for either axis in `SuvatOps2D`



## [Version 0.1.0] - 2026-06-27

### Added
- Physics module: Currently usable for Suvat operations in 1D and 2D. (**NOTE:** Later versions will include support for materials, electricity and dynamics). It contains accurate kinematic equations for each case where there's 3 different unit variables provided. The `SuvatOps2D` struct used for 2D suvat operations uses the `SuvatOps1D` struct for 1D operations on each axis to provide the final tuple values.

- Maths module: Can be used to convert between supported Metric length units, Imperial length units, Speed units, and also Mass units. (**NOTE:** Error information is included in module functions)

- Errors module: This module contains custom errors used in different parts of the maths and physics modules.
