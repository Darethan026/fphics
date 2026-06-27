# fphics

A lightweight mechanics, physics and maths solver written with zero dependencies - enforcing type safety with useful error messages.

### What does fphics do, and what is it?

- First of all, the name is an abbreviation of the following:
  - **f** - Fast (Since it's a lightweight library)
  - **ph** - Physics (Includes physics formulas for motion, forces, electricity...etc)
  - **ic** - Mechanics (Includes mechanics formulas, which of course are interchangeable with physics ones in most cases)
  - **s** - Mathematics (Includes maths formulas for percentages, unit conversions...etc)

fphics is a lightweight library that can be used for accurate and robust physics calculations, mathematical conversions - including unit conversions, all while being lightweight and efficient.

## Features

There's various mathematical and scientific operations possible with this library:
  - **Kinematics equations operations**
  - **Unit conversions**
  - **Electricity equations operations**
  - **Operations dealing with forces and motion**

**NOTE:** This library does not use any dependencies and all operations dealt with involve accurate mathematics with verified equations to produce exact accurate values without estimations!


### How to use? (Simple code example for calculating time)

```rust
use fphics::physics::suvat::*;
use fphics::errors::FphicsError;

fn main() -> Result<(), FphicsError> {
    let times = SuvatOps1D::new()
    .displacement(-2.5) // Set the displacement
    .initial_velocity(24.0) // Set the initial velocity
    //.final_velocity(25.0) // Set the final velocity (Optional, but not required since only 3 variables are required
    .acceleration(-9.8) // Set the acceleration
    .calculate_time()?; // Calculate time and propagate any possible errors
    
    // Loop through the times vector holding the positive and negative values from the calculation
    for t in times {
      println!("{t} seconds"); // Print the positive and negative value for the time
    }

    Ok(())
}

```

### An example using the maths module to convert cm to m

```rust
use fphics::maths::*;
use fphics::errors::FphicsError;

fn main() -> Result<(), FphicsError> {
  let cm = 1_234.5; // Use any floating point value for the Centimetre value

  let centimetre = Unit::new().from_metric_length(MetricLengthUnit::CentiMetre, cm); // Create an instance of the Unit struct with its unit as Centimetre
  
  let metre = centimetre.metric_to_metric(MetricLengthUnit::Metre)?; // Get the Metre value from the cm value

  println!("{}cm is {}m", cm, metre); // Print the result to stdout

    Ok(())
}

```

**NOTE:** You can also run the examples located in the `examples/` directory.

You can find the documentation on:
  - [fphics](https://crates.io/crates/fphics)
  - [docs](https://docs.rs/fphics)

Thank you for checking out **fphics**!
