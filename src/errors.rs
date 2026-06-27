//! fphics errors
//!
//! This module handles errors related with mathematical and physics operations
//
// Copyright (c) 2026 [Darell Ethan Kiganga]
// SPDX-License-Identifier: MIT

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
/// Error enum containing different errors for maths and physics operations
pub enum FphicsError {
	DivisionByZero,
	IncompleteData,
	NegativeSquareRoot,
	MissingVal,
	MissingMassUnit,
	MissingMetricUnit,
	MissingImperialUnit,
	MissingSpeedUnit,
	ConversionNotSupported,
}