use excel::{Workbook, Worksheet};
use std::error::Error;

pub fn excel_run() -> Result<(), Box<dyn Error>> {
    let workbook = Workbook::create("example.xlsx");
    let sheet = workbook.create_sheet("Sheet1");

    // A 2D array of strings
    let data = vec![
        vec!["Name", "Age", "Occupation"],
        vec!["John Doe", "30", "Engineer"],
        vec!["Mary Smith", "25", "Teacher"],
        vec!["Bob Johnson", "45", "Manager"],
    ];

    // Write the data to the worksheet
    for row in data {
        let mut row_writer = sheet.new_row()?;
        for cell in row {
            row_writer.write_string(cell)?;
        }
    }

    // Save the workbook as an Excel file
    workbook.close()?;

    Ok(())
}
