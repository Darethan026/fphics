use fphics::physics::suvat::*;
use fphics::errors::FphicsError;

fn main() -> Result<(), FphicsError> {
    let times = SuvatOps1D::new()
    .displacement(-2.5)
    .initial_velocity(24.0)
    .final_velocity(25.0)
    .acceleration(-9.8)
    .calculate_time()?;
    
    for t in times {
    	println!("{t} seconds");
    }

    Ok(())
}
