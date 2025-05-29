use winprint::printer::PrinterDevice;

use crate::error::*;

pub struct PrinterManager{
    default_printer: Option<PrinterDevice>,
    printers: Vec<PrinterDevice>,
}

impl PrinterManager {
    pub fn new() -> PrinterManager {
        let devices = PrinterDevice::all()
            .unwrap_or_else(|_| {
                println!("Failed to retrieve printer devices.");
                vec![]
            });

        PrinterManager { default_printer: None, printers: devices }
    }

    pub fn set_default_printer(&mut self, idx: usize) {
        if idx >= self.printers.len() {
            println!("Invalid printer index: {}", idx);
            return;
        }
        let printer = self.printers.get(idx)
            .expect("Printer index out of bounds");

        let device = self.printers.iter().find(|d| d.name() == printer.name())
            .expect("Printer not found in the list").clone();

        self.default_printer = Some(device);
    }

    pub fn default_printer(&self) -> Option<&PrinterDevice> {
        self.default_printer.as_ref()
    }

    pub fn list_printers(&self) -> Vec<String> {
        let devices = PrinterDevice::all()
            .unwrap_or_else(|_| {
                println!("Failed to retrieve printer devices.");
                vec![]
            });
    
        for device in &devices {
            println!("Device: {}", device.name());
        }
    
        let printers: Vec<String> = devices.into_iter().map(|d| d.name().to_string()).collect();
    
        printers
    }

    pub fn get_printer_by_name(&self, name: &str) -> Result<PrinterDevice> {
        let devices = PrinterDevice::all()
            .unwrap_or_else(|_| {
                println!("Failed to retrieve printer devices.");
                vec![]
            });
    
        devices.into_iter()
            .find(|d| d.name() == name)
            .ok_or(Error::PrinterNotFound(name.to_string()))
    }
}
