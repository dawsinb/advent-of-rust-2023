use calibration_document::CalibrationDocument;

mod calibration_document;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let calibration_document = CalibrationDocument::new(input);
    calibration_document.parse()
}

pub fn part_two(input: &str) -> Option<u32> {
    let calibration_document = CalibrationDocument::new(input);
    calibration_document.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
