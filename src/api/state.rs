use crate::printer::Printer;

#[derive(Clone)]
pub struct AppState<PT: Printer> {
    printer: PT
}

impl<PT: Printer> AppState<PT> {
    pub fn new(printer: PT) -> Self {
        AppState { printer }
    }

    pub fn printer(&self) -> &PT {
        &self.printer
    }
}