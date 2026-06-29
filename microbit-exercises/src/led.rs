use embassy_nrf::{
    Peri,
    gpio::{Level, Output, OutputDrive},
    peripherals::{P0_11, P0_15, P0_19, P0_21, P0_22, P0_24, P0_28, P0_30, P0_31, P1_05},
};

pub struct SimpleLedMatrix {
    pub rows: [Output<'static>; 5],
    pub cols: [Output<'static>; 5],
}

pub type RowPins = (
    Peri<'static, P0_21>,
    Peri<'static, P0_22>,
    Peri<'static, P0_15>,
    Peri<'static, P0_24>,
    Peri<'static, P0_19>,
);
pub type ColPins = (
    Peri<'static, P0_28>,
    Peri<'static, P0_11>,
    Peri<'static, P0_31>,
    Peri<'static, P1_05>,
    Peri<'static, P0_30>,
);
impl SimpleLedMatrix {
    pub fn new(row_pins: RowPins, col_pins: ColPins) -> Self {
        Self {
            rows: [
                Output::new(row_pins.0, Level::Low, OutputDrive::Standard),
                Output::new(row_pins.1, Level::Low, OutputDrive::Standard),
                Output::new(row_pins.2, Level::Low, OutputDrive::Standard),
                Output::new(row_pins.3, Level::Low, OutputDrive::Standard),
                Output::new(row_pins.4, Level::Low, OutputDrive::Standard),
            ],
            cols: [
                Output::new(col_pins.0, Level::Low, OutputDrive::Standard),
                Output::new(col_pins.1, Level::Low, OutputDrive::Standard),
                Output::new(col_pins.2, Level::Low, OutputDrive::Standard),
                Output::new(col_pins.3, Level::Low, OutputDrive::Standard),
                Output::new(col_pins.4, Level::Low, OutputDrive::Standard),
            ],
        }
    }
}

impl SimpleLedMatrix {
    pub fn line_strip(&mut self, row: usize) -> Option<LedStrip<'_>> {
        if row > self.rows.len() {
            return None;
        }
        Some(LedStrip::new(&mut self.rows[row], &mut self.cols))
    }

    pub fn col_strip(&mut self, col: usize) -> Option<LedStrip<'_>> {
        if col > self.cols.len() {
            return None;
        }
        Some(LedStrip::new(&mut self.cols[col], &mut self.rows))
    }
}

pub struct LedStrip<'matrix> {
    #[allow(dead_code)]
    always_high_row_or_col: &'matrix mut Output<'static>,
    line_control: &'matrix mut [Output<'static>; 5],
}

impl<'matrix> LedStrip<'matrix> {
    pub fn new(
        always_high_row_or_col: &'matrix mut Output<'static>,
        line_control: &'matrix mut [Output<'static>; 5],
    ) -> Self {
        always_high_row_or_col.set_high();
        for col in line_control.iter_mut() {
            col.set_high();
        }
        Self {
            always_high_row_or_col,
            line_control,
        }
    }
}

impl LedStrip<'_> {
    pub fn on(&mut self, index: usize) {
        if index > self.line_control.len() {
            return;
        }
        self.line_control[index].set_low();
    }

    pub fn off(&mut self, index: usize) {
        if index > self.line_control.len() {
            return;
        }
        self.line_control[index].set_high();
    }

    pub fn toggle(&mut self, index: usize) {
        if index > self.line_control.len() {
            return;
        }
        self.line_control[index].toggle();
    }
}
