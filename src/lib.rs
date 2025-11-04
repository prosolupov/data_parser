// use std::error::Error;
// use calamine::{open_workbook_auto, Reader, };

// fn read_xlsx() -> Result<(), Box<dyn Error>> {
//     let path: String = String::from("sample.xlsx");
//     let mut workbook = open_workbook_auto(&path)?;
//
//     for sheet_name in workbook.sheet_names().to_owned() {
//         println!("{}", sheet_name);
//
//         if let Ok(range) = workbook.worksheet_range(&sheet_name) {
//             for row in range.rows() {
//                 println!("{:?}", row);
//             }
//         }
//     }
//     Ok(())
//
// }

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
