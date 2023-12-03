mod calibration_value;

use self::calibration_value::CalibrationValue;

pub struct CalibrationDocument<'a>(Vec<CalibrationValue<'a>>);

impl<'a> CalibrationDocument<'a> {
    pub fn new(raw_document: &'a str) -> CalibrationDocument<'a> {
        let calibration_values = raw_document.lines().map(CalibrationValue::new).collect();

        Self(calibration_values)
    }

    pub fn parse(&self) -> Option<u32> {
        self.0
            .iter()
            .filter_map(|calibration_value| calibration_value.parse())
            .reduce(|acc, value| acc + value)
    }
}
