use std::path::PathBuf;

use winprint::printer::{FilePrinter, PrinterDevice, WinPdfPrinter};

use super::Printer;

use crate::error::*;

#[derive(Clone, Debug)]
pub struct WinPrinter{
    pub device: PrinterDevice,
    pub folder_path: PathBuf
}

impl Printer for WinPrinter{
    fn print(&self, filename: String) -> Result<()> {
        let printer = WinPdfPrinter::new(self.device.clone());
        let path = self.folder_path.join(filename);
        
        printer.print(path.as_path(), Default::default())
            .map_err(|e| Error::PrintFailed(e.to_string()))?;

       Ok(())
    }
}