#![allow(unused)]

//! fphics physics engine
//!
//! This module handles physics operations including suvat equations, forces, motion...etc, accurately without estimations, with the accurate formulas!

// Copyright (c) 2026 [Darell Ethan Kiganga]
// SPDX-License-Identifier: MIT

/*

S = Displacement in metres
U = Initial velocity in m/s
V = Final velocity in m/s
A = Acceleration in m/s^2
T = Time in seconds

s = (v^2 - u^2) / (2 * a)
s = ut + (1/2) * (a * t^2)
s = vt - (1/2) * (a * t^2)
s = (1/2) * (u + v) * t

u = (s/t) - (1/2 * a * t)
u^2 = (v^2 - 2as)
u = (2s/t) - v
u = v - at

v^2 = u^2 + 2as
v = s/t + (1/2 * a * t)
v = (2s/t) - u
v = u + at

a = (v^2 - u^2) / 2s
a = (2s - 2ut) / t^2
a = (2vt - 2s) / t^2
a = (v - u) / t

t = -ut + or - sqrt(ut^2 - (4 * 0.5a * -s)) / 2 * (0.5a)
t = -vt + or - sqrt(vt^2 - (4 * -0.5a * -s)) / 2 * (-0.5a)
t = (v - u) / a
t = 2s / (u + v)

*/

use crate::errors;

/// Constant for the speed of light
const SPEED_OF_LIGHT: f64 = 3.0 * 100_000_000.0;
/// Value for acceleration due to gravity on earth in m/s^2
const GRAVITY_ON_EARTH: f64 = 9.81;
/// Value for acceleration due to gravity on the moon in m/s^2
const GRAVITY_ON_MOON: f64 = 1.625;

/// Struct containing the letters symbolising the vector/scalar definitions in 1D
pub struct SuvatOps1D {
	s: Option<f64>, // Displacement in metres (m)
	u: Option<f64>, // Initial velocity in metres per second (m/s)
	v: Option<f64>, // Final velocity in metres per second (m/s)
	a: Option<f64>, // Acceleration in metres per second squared (m/s^2)
	t: Option<f64>, // Time in seconds (s)
}

/// Struct containing symbols for 2D kinematics
pub struct SuvatOps2D {
	s: (Option<f64>, Option<f64>), // Displacement in metres (m)
	u: (Option<f64>, Option<f64>), // Initial velocity in metres per second (m/s)
	v: (Option<f64>, Option<f64>), // Final velocity in metres per second (m/s)
	a: (Option<f64>, Option<f64>), // Acceleration in metres per second squared (m/s^2)
	t: Option<f64>, // Time in seconds (s)
}

/// Module containing operations for suvat equations
pub mod suvat {

	use crate::errors::FphicsError;
	pub use crate::physics::*;

	/// An implementatin of the SuvatOps1D struct for suvat operations in 1D
	impl SuvatOps1D {
		/// Create a new instance of the struct and begin operations.
		/// s = displacement, u = initial velocity, v = final velocity, t = time, a = acceleration
		pub fn new() -> Self {
			Self {
				s: None, // Set to default state
				u: None, // Set to default state
			 	v: None, // Set to default state
			 	a: None, // Set to default state
			 	t: None, // Set to default state
			}
		}

		/// Method to equate the value of s
		pub fn displacement(mut self, val: f64) -> Self {
			self.s = Some(val); // Equate the value of s in the struct to the val used by the user

			self
		}

		/// Method to equate the value of u
		pub fn initial_velocity(mut self, val: f64) -> Self {
			self.u = Some(val); // Equate the value of u in the struct to the val used by the user

			self
		}

		/// Method to equate the value of v
		pub fn final_velocity(mut self, val: f64) -> Self {
			self.v = Some(val); // Equate the value of v in the struct to the val used by the user

			self
		}

		/// Method to equate the value of a
		pub fn acceleration(mut self, val: f64) -> Self {
			self.a = Some(val); // Equate the value of a in the struct to the val used by the user

			self
		}

		/// Method to equate the value of t
		pub fn time(mut self, val: f64) -> Self {
			self.t = Some(val); // Equate the value of t in the struct to the val used by the user

			self
		}

		/// Method to calculate displacement based on available variables.
		/// # Error info
		/// - If there's a possibility of division by zero, `FphicsError::DivisionByZero` is returned
		/// - If there's less than 3 parameters, `FphicsError::IncompleteData` is returned
		pub fn calculate_displacement(&self) -> Result<f64, errors::FphicsError> {
			match (self.u, self.v, self.a, self.t) {
				// When only u, a, and t are present
				// s = ut + (1/2 * a * t^2)
				(Some(u_val), _, Some(a_val), Some(t_val)) => {
					Ok((u_val * t_val) + (0.5 * a_val * (t_val * t_val)))
				},

				// When only u, v, and a are present
				// s = (v^2 - u^2) / 2a
				(Some(u_val), Some(v_val), Some(a_val), _) => {
					if a_val == 0.0 {
						Err(errors::FphicsError::DivisionByZero)
					} else {
						Ok(((v_val * v_val) - (u_val * u_val)) / (2.0 * a_val))
					}
				},

				// When only v, a, and t are present
				// s = vt - (1/2 * a * t^2)
				(_, Some(v_val), Some(a_val), Some(t_val)) => {
					Ok((v_val * t_val) - (0.5 * a_val * (t_val * t_val)))
				},

				// When only u, v and t are present
				// s = ((u + v) / 2) * t
				(Some(u_val), Some(v_val), _, Some(t_val)) => {
					Ok(((u_val + v_val) / 2.0) * t_val)
				},

				_ => Err(errors::FphicsError::IncompleteData)
			}
		}

		/// Find the initial velocity of an object or variable
		/// # Error info
		/// - If there's a possibility of division by zero, `FphicsError::DivisionByZero` is returned
		/// - If there's less than 3 parameters, `FphicsError::IncompleteData` is returned
		pub fn calculate_initial_velocity(&self) -> Result<f64, errors::FphicsError> {
			match (self.s, self.v, self.a, self.t) {
				// When only s, a and t are present
				// u = (s/t) - (1/2 * a * t)
				(Some(s_val), _, Some(a_val), Some(t_val)) => {
					if t_val == 0.0 {
						Err(errors::FphicsError::DivisionByZero)
					} else {
						Ok((s_val / t_val) - (0.5 * a_val * t_val))
					}
				},

				// When only v, a, and s are present
				// u^2 = (v^2 - 2as)
				(Some(s_val), Some(v_val), Some(a_val), _) => {
					if ((v_val * v_val) - (2.0 * a_val * s_val)) < 0.0 {
						Err(errors::FphicsError::NegativeSquareRoot)
					} else {
						Ok(((v_val * v_val) - (2.0 * a_val * s_val)).sqrt())
					}
				},

				// When only v, s and t are present
				// u = (2s/t) - v
				(Some(s_val), Some(v_val), _, Some(t_val)) => {
					if t_val == 0.0 {
						Err(errors::FphicsError::DivisionByZero)
					} else {
						Ok(((2.0 * s_val) / t_val) - v_val)
					}
				},

				// When only v, a, and t are present
				// u = v - at
				(_, Some(v_val), Some(a_val), Some(t_val)) => {
					Ok(v_val - (a_val * t_val))
				},

				_ => Err(errors::FphicsError::IncompleteData)
			}
		}

		/// Calculate the final velocity
		/// # Error info
		/// - If there's a possibility of division by zero, `FphicsError::DivisionByZero` is returned
		/// - If there's less than 3 parameters, `FphicsError::IncompleteData` is returned
		pub fn calculate_final_velocity(&self) -> Result<f64, errors::FphicsError> {
			match (self.s, self.u, self.a, self.t) {
				// When only u, a, and s are present
				// v^2 = u^2 + 2as
				(Some(s_val), Some(u_val), Some(a_val), _) => {
					let val = (u_val * u_val) + (2.0 * a_val * s_val);

					if val < 0.0 {
						Err(errors::FphicsError::NegativeSquareRoot)
					} else {
						Ok(val.sqrt())
					}
				},

				// Where only s, a and t are present
				// v = s/t + (1/2 * a * t)
				(Some(s_val), _, Some(a_val), Some(t_val)) => {
					if t_val == 0.0 {
						Err(errors::FphicsError::DivisionByZero)
					} else {
						Ok((s_val / t_val) + (0.5 * a_val * t_val))
					}
				},

				// Where only s, u and t are present
				// v = (2s/t) - u
				(Some(s_val), Some(u_val), _, Some(t_val)) => {
					if t_val == 0.0 {
						Err(errors::FphicsError::DivisionByZero)
					} else {
						Ok(((2.0 * s_val) / t_val) - u_val)
					}
				},

				// Where only u, a and t are present
				// v = u + at
				(_, Some(u_val), Some(a_val), Some(t_val)) => {
					Ok(u_val + (a_val * t_val))
				},

				_ => Err(errors::FphicsError::IncompleteData)
			}
		}

		/// Calculate the acceleration
		/// # Error info
		/// - If there's a possibility of division by zero, `FphicsError::DivisionByZero` is returned
		/// - If there's less than 3 parameters, `FphicsError::IncompleteData` is returned
		pub fn calculate_acceleration(&self) -> Result<f64, errors::FphicsError> {
			match (self.s, self.u, self.v, self.t) {
				// When we have s, u and v
				// a = (v^2 - u^2) / 2s
				(Some(s_val), Some(u_val), Some(v_val), _) => {
					if s_val == 0.0 {
						Err(errors::FphicsError::DivisionByZero)
					} else {
						Ok(((v_val * v_val) - (u_val * u_val)) / (2.0 * s_val))
					}
				},

				// When we have s, u and t
				// a = (2s - 2ut) / t^2
				(Some(s_val), Some(u_val), _, Some(t_val)) => {
					if t_val == 0.0 {
						Err(errors::FphicsError::DivisionByZero)
					} else {
						Ok(((2.0 * s_val) - (2.0 * u_val * t_val)) / (t_val * t_val))
					}
				},

				// When we have s, v and t
				// a = (2vt - 2s) / t^2
				(Some(s_val), _, Some(v_val), Some(t_val)) => {
					if t_val == 0.0 {
						Err(errors::FphicsError::DivisionByZero)
					} else {
						Ok(((2.0 * v_val * t_val) - (2.0 * s_val)) / (t_val * t_val))
					}
				},

				// When we have u, v and t
				// a = (v - u) / t
				(_, Some(u_val), Some(v_val), Some(t_val)) => {
					if t_val == 0.0 {
						Err(errors::FphicsError::DivisionByZero)
					} else {
						Ok((v_val - u_val) / t_val)
					}
				},

				_ => Err(errors::FphicsError::IncompleteData)
			}
		}

		/// Calculate the time
		/// # Error info
		/// - If there's a possibility of division by zero, `FphicsError::DivisionByZero` is returned
		/// - If there's less than 3 parameters, `FphicsError::IncompleteData` is returned
		pub fn calculate_time(&self) -> Result<Vec<f64>, errors::FphicsError> {
			let mut vals = Vec::with_capacity(2);

			match (self.s, self.u, self.v, self.a) {
				// When we only have u, a and s
				// t = -ut +/- sqrt(ut^2 - (4 * 0.5a * -s)) / 2 * (0.5a)
				(Some(s_val), Some(u_val), _, Some(a_val)) => {
					let discriminant = (u_val * u_val) - (2.0 * a_val * -s_val);

					if discriminant < 0.0 {
						Err(errors::FphicsError::NegativeSquareRoot)
					} else {
						vals.push(((- u_val) + discriminant.sqrt()) / a_val);
						vals.push(((- u_val) - discriminant.sqrt()) / a_val);

						Ok(vals)
					}
				},

				// When we only have s, v and a
				// t = -vt + or - sqrt(vt^2 - (4 * -0.5a * -s)) / 2 * (-0.5a)
				(Some(s_val), _, Some(v_val), Some(a_val)) => {
					let discriminant = (v_val * v_val) - (2.0 * a_val * s_val);

					if discriminant < 0.0 {
						Err(errors::FphicsError::NegativeSquareRoot)
					} else {
						vals.push(((-v_val) + discriminant.sqrt()) / -a_val);
						vals.push(((-v_val) - discriminant.sqrt()) / -a_val);

						Ok(vals)
					}
				},

				// When we only have u, v and a
				// t = (v - u) / a
				(_, Some(u_val), Some(v_val), Some(a_val)) => {
					if a_val == 0.0 {
						Err(errors::FphicsError::DivisionByZero)
					} else {
						vals.push((v_val - u_val) / a_val);

						Ok(vals)
					}
				},

				// When we only have s, u and v
				// t = 2s / (u + v)
				(Some(s_val), Some(u_val), Some(v_val), _) => {
					if u_val + v_val == 0.0 {
						Err(errors::FphicsError::DivisionByZero)
					} else {
						vals.push((2.0 * s_val) / (u_val + v_val));

						Ok(vals)
					}
				},

				_ => Err(errors::FphicsError::IncompleteData)
			}
		}
	}

	/// An implementatin of the SuvatOps1D struct for suvat operations in 1D
	impl SuvatOps2D {
		/// Create a new instance of the struct and begin operations.
		/// s = displacement, u = initial velocity, v = final velocity, t = time, a = acceleration
		pub fn new() -> Self {
			Self {
				s: (None, None), // Displacement in metres (m)
				u: (None, None), // Initial velocity in metres per second (m/s)
				v: (None, None), // Final velocity in metres per second (m/s)
				a: (None, None), // Acceleration in metres per second squared (m/s^2)
				t: None, // Time in seconds (s)
			}
		}

		/// Method to equate the value of s
		pub fn displacement(mut self, val: (f64, f64)) -> Self {
			self.s.0 = Some(val.0);
			self.s.1 = Some(val.1);

			self
		}

		/// Method to equate the value of u
		pub fn initial_velocity(mut self, val: (f64, f64)) -> Self {
			self.u.0 = Some(val.0);
			self.u.1 = Some(val.1);

			self
		}

		/// Method to equate the value of v
		pub fn final_velocity(mut self, val: (f64, f64)) -> Self {
			self.v.0 = Some(val.0);
			self.v.1 = Some(val.1);

			self
		}

		/// Method to equate the value of a
		pub fn acceleration(mut self, val: (f64, f64)) -> Self {
			self.a.0 = Some(val.0);
			self.a.1 = Some(val.1);

			self
		}

		/// Method to equate the value of t
		pub fn time(mut self, val: f64) -> Self {
			self.t = Some(val);

			self
		}

		/// Calculate the displacement in 2D
		/// # Error info
		/// - If there's a possibility of division by zero, `FphicsError::DivisionByZero` is returned
		/// - If there's less than 3 parameters, `FphicsError::IncompleteData` is returned
		pub fn calculate_displacement(&self) -> Result<(f64, f64), errors::FphicsError> {
			let u = self.u;
			let v = self.v;
			let a = self.a;
			let t = self.t;

			// Create an instance of the 1D struct to fill the i components
			let i_component = SuvatOps1D {
				s: None,
				u: u.0,
				v: v.0,
				a: a.0,
				t: t,
			}.calculate_displacement()?;

			// Create an instance of the 1D struct to fill the j components
			let j_component = SuvatOps1D {
				s: None,
				u: u.1,
				v: v.1,
				a: a.1,
				t: t,
			}.calculate_displacement()?;

			Ok((i_component, j_component))
		}

		/// Calculate the initial velocity in 2D
		/// # Error info
		/// - If there's a possibility of division by zero, `FphicsError::DivisionByZero` is returned
		/// - If there's less than 3 parameters, `FphicsError::IncompleteData` is returned
		pub fn calculate_initial_velocity(&self) -> Result<(f64, f64), FphicsError> {
			let s = self.s;
			let v = self.v;
			let a = self.a;
			let t = self.t;

			// Create an instance of the 1D struct to fill the i components
			let i_component = SuvatOps1D {
				s: s.0,
				u: None,
				v: v.0,
				a: a.0,
				t: t,
			}.calculate_initial_velocity()?;

			// Create an instance of the 1D struct to fill the j components
			let j_component = SuvatOps1D {
				s: s.1,
				u: None,
				v: v.1,
				a: a.1,
				t: t,
			}.calculate_initial_velocity()?;

			Ok((i_component, j_component))
		}

		/// Calculate the final velocity in 2D
		/// # Error info
		/// - If there's a possibility of division by zero, `FphicsError::DivisionByZero` is returned
		/// - If there's less than 3 parameters, `FphicsError::IncompleteData` is returned
		pub fn calculate_final_velocity(&self) -> Result<(f64, f64), FphicsError> {
			let s = self.s;
			let u = self.u;
			let a = self.a;
			let t = self.t;

			// Create an instance of the 1D struct to fill the i components
			let i_component = SuvatOps1D {
				s: s.0,
				u: u.0,
				v: None,
				a: a.0,
				t: t,
			}.calculate_final_velocity()?;

			// Create an instance of the 1D struct to fill the j components
			let j_component = SuvatOps1D {
				s: s.1,
				u: u.1,
				v: None,
				a: a.1,
				t: t,
			}.calculate_final_velocity()?;

			Ok((i_component, j_component))
		}

		/// Calculate the acceleration in 2D
		/// # Error info
		/// - If there's a possibility of division by zero, `FphicsError::DivisionByZero` is returned
		/// - If there's less than 3 parameters, `FphicsError::IncompleteData` is returned
		pub fn calculate_acceleration(&self) -> Result<(f64, f64), FphicsError> {
			let s = self.s;
			let u = self.u;
			let v = self.v;
			let t = self.t;

			// Create an instance of the 1D struct to fill the i components
			let i_component = SuvatOps1D {
				s: s.0,
				u: u.0,
				v: v.0,
				a: None,
				t: t,
			}.calculate_acceleration()?;

			// Create an instance of the 1D struct to fill the j components
			let j_component = SuvatOps1D {
				s: s.1,
				u: u.1,
				v: v.1,
				a: None,
				t: t,
			}.calculate_acceleration()?;

			Ok((i_component, j_component))
		}

		/// Calculate the time
		/// # INFO (PLEASE READ)
		/// - If there's a possibility of division by zero, `FphicsError::DivisionByZero` is returned
		/// - If there's less than 3 parameters, `FphicsError::IncompleteData` is returned
		/// - This method returns a tuple of two f64 vectors, each holding a positive and negative time value from the time calculation
		pub fn calculate_time(&self) -> Result<(Vec<f64>, Vec<f64>), FphicsError> {
			let s = self.s;
			let u = self.u;
			let v = self.v;
			let a = self.a;

			// Create an instance of the 1D struct to fill the i components
			let i_component = SuvatOps1D {
				s: s.0,
				u: u.0,
				v: v.0,
				a: a.0,
				t: None,
			}.calculate_time()?;

			// Create an instance of the 1D struct to fill the j components
			let j_component = SuvatOps1D {
				s: s.1,
				u: u.1,
				v: v.1,
				a: a.1,
				t: None,
			}.calculate_time()?;

			Ok((i_component, j_component))
		}
	}
}

/// Future implementation
mod materials {}

/// Future implementation
mod dynamics {}

/// Future implementation
mod electricity {}