#![allow(unused)]

//! fphics maths engine
//!
//! This module handles maths conversion operations between units regarding length, motion, mass...
//
// Copyright (c) 2026 [Darell Ethan Kiganga]
// SPDX-License-Identifier: MIT
/*
========================
Length conversions
========================

Millimetre to Centimetre = / 10
Millimetre to Metre = / 1000
Millimetre to Km = / 1,000,000
Centimetre to Millimetre = * 10
Centimetre to Metre = / 100
Centimetre to Kilometre = / 100,000

Metre to Millimetre = * 1000
Metre to Centimetre = * 100
Metre to Kilometre = / 1000

Kilometre to Millimetre = * 1,000,000
Kilometre to Centimetre = * 100,000
Kilometre to Metre = * 1000

======================
Metric Mass conversions
======================

Gram to Kilogram = / 1000
Gram to lb = * 0.00220462262
 
Kilogram to Gram = * 1000
Kilogram to lb = * 2.20462262

lb to Gram = * 453.59237
lb to Kilogram = * 0.45359237

======================
Imperial Mass conversions
======================

in to yd = * 0.0277777778
in to ft = * 0.0833333333
in to mile = * 0.0000157828283

yd to in = * 36
yd to ft = * 3
yd to mile = * 0.000568181818

ft to in = * 12
ft to yd = * 0.333333333
ft to mile = * 0.000189393939

mile to in = * 63,360
mile to yd = * 1760
mile to ft = * 5280

==============================
Metric to Imperial conversions
==============================
mm to in = * 0.0393700787

cm to in = * 0.393700787
cm to yd = * 0.010936133
Metres to Yard = * 1.0936133
Metres to Foot = * 3.2808399
Metres to Mile = * 0.000621371192

Kilometre to Foot = * 3280.8399
Kilometre to Mile = * 0.621371192

==============================
Imperial to Metric conversions
==============================

in to mm = * 25.4

in to cm = * 2.54
yd to cm = * 91.44

Yard to Metres = * 0.9144
Foot to Metres = * 0.3048
Mile to Metres = * 1609.344

Foot to Kilometre = * 0.0003048
Mile to Kilometre = * 1.609344

*/

use crate::errors::FphicsError;

/// Enum containing the metric system length units
#[derive(Debug)]
pub enum MetricLengthUnit {
	MilliMetre, // Millimetre unit
	CentiMetre, // Centimetre unit
	Metre, // Metre unit
	KiloMetre, // Kilometre unit
}

/// Enum containing units for mass
#[derive(Debug)]
pub enum MassUnit {
	Gram, // Gram unit
	Kilogram, // Kilogram unit
	Pound, // Pound (lb) unit
}

/// Enum containing the imperial system length units
#[derive(Debug)]
pub enum ImperialUnitLength {
	Inch, // Inch unit
	Yard, // Yard unit
	Foot, // Foot unit
	Mile, // Mile unit
}

/// Enum containing units for speed
#[derive(Debug)]
pub enum SpeedUnit {
	CmPerSecond, // cm/s
	MPerSecond, // m/s
	KmPerSecond, // km/s
	KPerHour, // km/h
	MPerHour, // mph
}

/// Struct holding all enum units
#[derive(Debug)]
pub struct Unit {
	metric_length_unit: (Option<MetricLengthUnit>, Option<f64>), // Struct filed holding the Metric length enum and its Option<f64> value
	imperial_length_unit: (Option<ImperialUnitLength>, Option<f64>), // Struct filed holding the Imperial length enum and its Option<f64> value
	mass_unit: (Option<MassUnit>, Option<f64>), // Struct filed holding the Mass enum and its Option<f64> value
	speed_unit: (Option<SpeedUnit>, Option<f64>), // Struct filed holding the Speed enum and its Option<f64> value
}

// Mass conversion multiplier constants

const G_TO_LB: f64 = 0.00220462262; // Multiplier for Grams to Pounds (lb)
const LB_TO_G: f64 = 453.59237; // Multiplier for Pounds (lb) to Grams
const LB_TO_KG: f64 = 0.45359237; // Multiplier for Pounds (lb) to Kilograms
const KG_TO_LB: f64 = 2.20462262; // Multiplier for Kilograms to Pounds (lb)

// Imperial to imperial conversion multiplier constants

const IN_TO_YD: f64 = 0.0277777778; // Multiplier for Inches to Yards
const IN_TO_FT: f64 = 0.0833333333; // Multiplier for Inches to Feet
const IN_TO_MILE: f64 = 0.0000157828283; // Multiplier for Inches to Miles
const YD_TO_MILE: f64 = 0.000568181818; // Multiplier for Yards to Miles
const FT_TO_YD: f64 = 0.333333333; // Multiplier for Feet to Yards
const FT_TO_MILE: f64 = 0.000189393939; // Multiplier for Feet to Miles

// Metric to imperial conversion multiplier constants

const MM_TO_IN: f64 = 0.0393700787; // Multiplier for Milimetres to Inches
const CM_TO_IN: f64 = 0.393700787; // Multiplier for Centimetres to Inches
const CM_TO_YD: f64 = 0.010936133; // Multiplier for Centimetres to Yards
const METRES_TO_YD: f64 = 1.0936133; // Multiplier for Metres to Yards
const METRES_TO_FT: f64 = 3.2808399; // Multiplier for Metres to Feet
const METRES_TO_MILE: f64 = 0.000621371192; // Multiplier for Metres to Miles
const KM_TO_FT: f64 = 3280.8399; // Multiplier for Kilometres to Feet
const KM_TO_MILE: f64 = 0.621371192; // Multiplier for Kilometres to Miles

// Imperial to metric conversion multiplier constants

const IN_TO_MM: f64 = 25.4; // Multiplier for Inches to Milimetres
const IN_TO_CM: f64 = 2.54; // Multiplier for Inches to Centimetres
const YD_TO_CM: f64 = 91.44; // Multiplier for Yards to Centimetres
const YD_TO_METRES: f64 = 0.9144; // Multiplier for Yards to Metres
const FT_TO_METRES: f64 = 0.3048; // Multiplier for Feet to Metres
const MILES_TO_METRES: f64 = 1609.344; // Multiplier for Miles to Metres
const FT_TO_KM: f64 = 0.0003048; // Multiplier for Feet to Kilometres
const MILES_TO_KM: f64 = 1.609344; // Multiplier for Miles to Kilometres

impl Unit {
	/// Function to create a new instance of the `Unit` struct with all its fields set to none
	pub fn new() -> Self {
		Self {
			metric_length_unit: (None, None), // Set the Unit and value to None
			imperial_length_unit: (None, None), // Set the Unit and value to None
			mass_unit: (None, None), // Set the Unit and value to None
			speed_unit: (None, None), // Set the Unit and value to None
		}
	}

	/// Function to set the metric length value with the enum variant for the unit
	pub fn from_metric_length(mut self, mut unit: MetricLengthUnit, val: f64) -> Self {
		self.metric_length_unit = (Some(unit), Some(val)); // Set the metric length unit and value to the user's input

		self
	}

	/// Function to set the imperial length value with the enum variant for the unit
	pub fn from_imperial_length(mut self, unit: ImperialUnitLength, val: f64) -> Self {
		self.imperial_length_unit = (Some(unit), Some(val)); // Set the imperial length unit and value to the user's input

		self
	}

	/// Function to set the speed value with the enum variant for the unit
	pub fn from_speed(mut self, unit: SpeedUnit, val: f64) -> Self {
		self.speed_unit = (Some(unit), Some(val)); // Set the speed unit and value to the user's input

		self
	}

	/// Function to set the mass value with the enum variant for the unit
	pub fn from_mass(mut self, unit: MassUnit, val: f64) -> Self {
		self.mass_unit = (Some(unit), Some(val)); // Set the mass unit and value to the user's input

		self
	}

	// ============//
	// CONVERSIONS //
	// =========== //

	/// Function to convert a metric unit to another metric unit
	/// # Error info
	/// - If no value is used while creating a new instance of the `Unit` struct and you try using this method, i.e.,
	/// doing something similar to `let unit = Unit::new();`, and use this method on it, the error
	/// `Error: MissingMetricUnit` will be returned. If there's a missing value, the error
	pub fn metric_to_metric(&self, unit: MetricLengthUnit) -> Result<f64, FphicsError> {
		// Match for the user's unit
		match unit {
			MetricLengthUnit::MilliMetre => {
				// Match for the value the struct holds for the unit
				match &self.metric_length_unit.0 {
					Some(stored_val) => {
						// Match the stored value
						match stored_val {
							MetricLengthUnit::MilliMetre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Millimetre to Millimetre
									Some(v) => Ok(v),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::CentiMetre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Centimetre to Millimetre
									Some(v) => Ok(v * 10.0),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::Metre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Metre to Millimetre
									Some(v) => Ok(v * 1000.0),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::KiloMetre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Kilometre to Millimetre
									Some(v) => Ok(v * 1_000_000.0),

									None => Err(FphicsError::MissingVal),
								}
							}
						}
					},

					None => Err(FphicsError::MissingMetricUnit)
				}
			}

			MetricLengthUnit::CentiMetre => {
				// Match for the value the struct holds for the unit
				match &self.metric_length_unit.0 {
					Some(stored_val) => {
						// Match the stored value
						match stored_val {
							MetricLengthUnit::MilliMetre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Millimetre to Centimetre
									Some(v) => Ok(v / 10.0),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::CentiMetre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Centimetre to Centimetre
									Some(v) => Ok(v),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::Metre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Metre to Centimetre
									Some(v) => Ok(v * 100.0),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::KiloMetre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Kilometre to Centimetre
									Some(v) => Ok(v * 100_000.0),

									None => Err(FphicsError::MissingVal),
								}
							}
						}
					},

					None => Err(FphicsError::MissingMetricUnit)
				}
			}

			MetricLengthUnit::Metre => {
				// Match for the value the struct holds for the unit
				match &self.metric_length_unit.0 {
					Some(stored_val) => {
						// Match the stored value
						match stored_val {
							MetricLengthUnit::MilliMetre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Millimetre to Metre
									Some(v) => Ok(v / 1_000.0),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::CentiMetre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Centimetre to Metre
									Some(v) => Ok(v / 100.0),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::Metre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Metre to Metre
									Some(v) => Ok(v),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::KiloMetre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Kilometre to Metre
									Some(v) => Ok(v * 1_000.0),

									None => Err(FphicsError::MissingVal),
								}
							}
						}
					},

					None => Err(FphicsError::MissingMetricUnit)
				}
			}

			MetricLengthUnit::KiloMetre => {
				// Match for the value the struct holds for the unit
				match &self.metric_length_unit.0 {
					Some(stored_val) => {
						// Match the stored value
						match stored_val {
							MetricLengthUnit::MilliMetre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Millimetre to Kilometre
									Some(v) => Ok(v / 1_000_000.0),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::CentiMetre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Centimetre to Kilometre
									Some(v) => Ok(v / 100_000.0),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::Metre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Metre to Kilometre
									Some(v) => Ok(v / 1_000.0),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::KiloMetre => {
								// Match for the value of the metric length
								match self.metric_length_unit.1 {
									// Value to return for Kilometre to Kilometre
									Some(v) => Ok(v),

									None => Err(FphicsError::MissingVal),
								}
							}
						}
					},

					None => Err(FphicsError::MissingMetricUnit)
				}
			}
		}
	}

	/// Function to convert between the Imperial length units
	/// # Error info
	/// - If no value is present in the `Unit` struct for the field `imperial_unit_length`, the error
	/// `MissingVal` will be returned.
	/// - If no unit is present in the `Unit` struct for the field `imperial_unit_length`, the error
	/// `MissingImperialUnit` will be returned.
	pub fn imperial_to_imperial(&self, unit: ImperialUnitLength) -> Result<f64, FphicsError> {
		match unit {
			ImperialUnitLength::Inch => {
				// Check if the unit is available on the struct
				match &self.imperial_length_unit.0 {
					Some(stored_val) => {
						match stored_val {
							ImperialUnitLength::Inch => {
								// Check if the value is available on the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Yard => {
								// Check if the value is available in the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * 36.0),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Foot => {
								// Check if the value is in the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * 12.0),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Mile => {
								// Check if the value is in the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * 63_360.0),

									None => Err(FphicsError::MissingVal),
								}
							}
						}
					}

					None => Err(FphicsError::MissingImperialUnit)
				}
			}

			ImperialUnitLength::Yard => {
				// Check if the unit is available on the struct
				match &self.imperial_length_unit.0 {
					Some(stored_val) => {
						match stored_val {
							ImperialUnitLength::Inch => {
								// Check if the value is available on the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * IN_TO_YD),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Yard => {
								// Check if the value is available in the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Foot => {
								// Check if the value is in the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * FT_TO_YD),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Mile => {
								// Check if the value is in the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * 1_760.0),

									None => Err(FphicsError::MissingVal),
								}
							}
						}
					}

					None => Err(FphicsError::MissingImperialUnit)
				}
			}

			ImperialUnitLength::Foot => {
				// Check if the unit is available on the struct
				match &self.imperial_length_unit.0 {
					Some(stored_val) => {
						match stored_val {
							ImperialUnitLength::Inch => {
								// Check if the value is available on the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * IN_TO_FT),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Yard => {
								// Check if the value is available in the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * 3.0),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Foot => {
								// Check if the value is in the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Mile => {
								// Check if the value is in the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * 5280.0),

									None => Err(FphicsError::MissingVal),
								}
							}
						}
					}

					None => Err(FphicsError::MissingImperialUnit)
				}
			}

			ImperialUnitLength::Mile => {
				// Check if the unit is available on the struct
				match &self.imperial_length_unit.0 {
					Some(stored_val) => {
						match stored_val {
							ImperialUnitLength::Inch => {
								// Check if the value is available on the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * IN_TO_MILE),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Yard => {
								// Check if the value is available in the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * YD_TO_MILE),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Foot => {
								// Check if the value is in the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * FT_TO_MILE),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Mile => {
								// Check if the value is in the struct
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v),

									None => Err(FphicsError::MissingVal),
								}
							}
						}
					}

					None => Err(FphicsError::MissingImperialUnit)
				}
			}
		}
	}

	/// Function to convert a metric unit to an imperial unit
	/// # Error info
	/// - There's no support for conversions between metres or kilometres to inches, milimetres or kilometres to yards,
	/// milimetres or centimetres to feet, and milimetres or centimetres to miles.
	/// - If any unsupported conversions are attempted, the error `ConversionNotSupported` will
	/// be returned.
	pub fn metric_to_imperial(&self, unit: ImperialUnitLength) -> Result<f64, FphicsError> {
		match unit {
			ImperialUnitLength::Inch => {
				// Check the stored metric unit
				match &self.metric_length_unit.0 {
					Some(m_unit) => {
						match m_unit {
							MetricLengthUnit::MilliMetre => {
								match self.metric_length_unit.1 {
									Some(v) => Ok(v * MM_TO_IN),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::CentiMetre => {
								match self.metric_length_unit.1 {
									Some(v) => Ok(v * CM_TO_IN),

									None => Err(FphicsError::MissingVal),
								}
							}

							_ => Err(FphicsError::ConversionNotSupported)
						}
					}

					None => Err(FphicsError::MissingMetricUnit)
				} 
			}

			ImperialUnitLength::Yard => {
				// Check the stored metric unit
				match &self.metric_length_unit.0 {
					Some(m_unit) => {
						match m_unit {
							MetricLengthUnit::CentiMetre => {
								match self.metric_length_unit.1 {
									Some(v) => Ok(v * CM_TO_YD),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::Metre => {
								match self.metric_length_unit.1 {
									Some(v) => Ok(v * METRES_TO_YD),

									None => Err(FphicsError::MissingVal),
								}
							}

							_ => Err(FphicsError::ConversionNotSupported)
						}
					}

					None => Err(FphicsError::MissingMetricUnit)
				}
			}

			ImperialUnitLength::Foot => {
				// Check the stored metric unit
				match &self.metric_length_unit.0 {
					Some(m_unit) => {
						match m_unit {
							MetricLengthUnit::Metre => {
								match self.metric_length_unit.1 {
									Some(v) => Ok(v * METRES_TO_FT),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::KiloMetre => {
								match self.metric_length_unit.1 {
									Some(v) => Ok(v * KM_TO_FT),

									None => Err(FphicsError::MissingVal),
								}
							}

							_ => Err(FphicsError::ConversionNotSupported)
						}
					}

					None => Err(FphicsError::MissingMetricUnit)
				}
			}

			ImperialUnitLength::Mile => {
				// Check the stored metric unit
				match &self.metric_length_unit.0 {
					Some(m_unit) => {
						match m_unit {
							MetricLengthUnit::Metre => {
								match self.metric_length_unit.1 {
									Some(v) => Ok(v * METRES_TO_MILE),

									None => Err(FphicsError::MissingVal),
								}
							}

							MetricLengthUnit::KiloMetre => {
								match self.metric_length_unit.1 {
									Some(v) => Ok(v * KM_TO_MILE),

									None => Err(FphicsError::MissingVal),
								}
							}

							_ => Err(FphicsError::ConversionNotSupported)
						}
					}

					None => Err(FphicsError::MissingMetricUnit)
				}
			}
		}	
	}

	/// Function to convert an Imperial unit to an metric unit
	/// # Error info
	/// - There's no support for conversions between inches to metres or kilometres, yards to milimetres
	/// or kilometres, feet to milimetres or centimetres, and miles to milimetres or centimetres.
	/// - If any unsupported conversions are attempted, the error `ConversionNotSupported` will be returned.
	pub fn imperial_to_metric(&self, unit: MetricLengthUnit) -> Result<f64, FphicsError> {
		match unit {
			MetricLengthUnit::MilliMetre => {
				// Check the stored metric unit
				match &self.imperial_length_unit.0 {
					Some(stored_unit) => {
						match stored_unit {
							ImperialUnitLength::Inch => {
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * IN_TO_MM),

									None => Err(FphicsError::MissingVal),
								}
							}

							_ => Err(FphicsError::ConversionNotSupported)
						}
					}

					None => Err(FphicsError::MissingImperialUnit)
				}
			}

			MetricLengthUnit::CentiMetre => {
				// Check the stored metric unit
				match &self.imperial_length_unit.0 {
					Some(stored_unit) => {
						match stored_unit {
							ImperialUnitLength::Inch => {
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * IN_TO_CM),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Yard => {
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * YD_TO_CM),

									None => Err(FphicsError::MissingVal),
								}
							}

							_ => Err(FphicsError::ConversionNotSupported)
						}
					}

					None => Err(FphicsError::MissingImperialUnit)
				}
			}

			MetricLengthUnit::Metre => {
				// Check the stored metric unit
				match &self.imperial_length_unit.0 {
					Some(stored_unit) => {
						match stored_unit {
							ImperialUnitLength::Yard => {
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * YD_TO_METRES),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Foot => {
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * FT_TO_METRES),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Mile => {
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * MILES_TO_METRES),

									None => Err(FphicsError::MissingVal),
								}
							}

							_ => Err(FphicsError::ConversionNotSupported)
						}
					}

					None => Err(FphicsError::MissingImperialUnit)
				}
			}

			MetricLengthUnit::KiloMetre => {
				// Check the stored metric unit
				match &self.imperial_length_unit.0 {
					Some(stored_unit) => {
						match stored_unit {
							ImperialUnitLength::Foot => {
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * FT_TO_KM),

									None => Err(FphicsError::MissingVal),
								}
							}

							ImperialUnitLength::Mile => {
								match self.imperial_length_unit.1 {
									Some(v) => Ok(v * MILES_TO_KM),

									None => Err(FphicsError::MissingVal),
								}
							}

							_ => Err(FphicsError::ConversionNotSupported)
						}
					}

					None => Err(FphicsError::MissingImperialUnit)
				}
			}
		}
	}













	/// Function to convert between the available mass units
	pub fn to_mass(&self, unit: MassUnit) -> Result<f64, FphicsError> {
		// Match for the user's unit
		match unit {
			MassUnit::Gram => {
				// Match for the value the struct holds for the unit
				match &self.mass_unit.0 {
					Some(stored_val) => {
						match stored_val {
							MassUnit::Gram => {
								// Match for its stored value
								match self.mass_unit.1 {
									Some(v) => Ok(v),

									None => Err(FphicsError::MissingVal),
								}
							}

							MassUnit::Kilogram => {
								match self.mass_unit.1 {
									Some(v) => Ok(v * 1000.0),

									None => Err(FphicsError::MissingVal),
								}
							}

							MassUnit::Pound => {
								match self.mass_unit.1 {
									Some(v) => Ok(v * LB_TO_G),

									None => Err(FphicsError::MissingVal),
								}
							}
						}
					}

					None => Err(FphicsError::MissingMassUnit)
				}
			}

			MassUnit::Kilogram => {
				// Match for the value the struct holds for the unit
				match &self.mass_unit.0 {
					Some(stored_val) => {
						match stored_val {
							MassUnit::Gram => {
								// Match for its stored value
								match self.mass_unit.1 {
									Some(v) => Ok(v / 1000.0),

									None => Err(FphicsError::MissingVal),
								}
							}

							MassUnit::Kilogram => {
								match self.mass_unit.1 {
									Some(v) => Ok(v),

									None => Err(FphicsError::MissingVal),
								}
							}

							MassUnit::Pound => {
								match self.mass_unit.1 {
									Some(v) => Ok(v * LB_TO_KG),

									None => Err(FphicsError::MissingVal),
								}
							}
						}
					}

					None => Err(FphicsError::MissingMassUnit)
				}
			}

			MassUnit::Pound => {
				// Match for the value the struct holds for the unit
				match &self.mass_unit.0 {
					Some(stored_val) => {
						match stored_val {
							MassUnit::Gram => {
								// Match for its stored value
								match self.mass_unit.1 {
									Some(v) => Ok(v * G_TO_LB),

									None => Err(FphicsError::MissingVal),
								}
							}

							MassUnit::Kilogram => {
								match self.mass_unit.1 {
									Some(v) => Ok(v * KG_TO_LB),

									None => Err(FphicsError::MissingVal),
								}
							}

							MassUnit::Pound => {
								match self.mass_unit.1 {
									Some(v) => Ok(v),

									None => Err(FphicsError::MissingVal),
								}
							}
						}
					}

					None => Err(FphicsError::MissingMassUnit)
				}
			}
		}
	}
}