use fphics::maths::*;
use fphics::errors::FphicsError;

fn main() -> Result<(), FphicsError> {
	let cm = 1_234.5;

	let centimetre = Unit::new().from_metric_length(MetricLengthUnit::CentiMetre, cm);
	
	let metre = centimetre.metric_to_metric(MetricLengthUnit::Metre)?;

	println!("{}cm is {}m", cm, metre);

    Ok(())
}
