use fphics::maths::*;
use fphics::errors::FphicsError;

fn main() -> Result<(), FphicsError> {
	let m = 5.0;
	let cm = 183.0;

	let metre = Unit::new().from_metric_length(MetricLengthUnit::Metre, m);
	let centimetre = Unit::new().from_metric_length(MetricLengthUnit::CentiMetre, cm);

	let ft = metre.metric_to_imperial(ImperialUnitLength::Foot)?;
	let yard = metre.metric_to_imperial(ImperialUnitLength::Yard)?;
	let inch = centimetre.metric_to_imperial(ImperialUnitLength::Inch)?;
	let cm1 = metre.metric_to_metric(MetricLengthUnit::CentiMetre)?;

	println!("{m} metre(s) = {} feet or {} yards", ft, yard);
	println!();
	println!("{m} metre(s) = {} centimetre(s)", cm1);
	println!();
	println!("{cm} centimetre(s) = {} inches", inch);

	Ok(())
}
