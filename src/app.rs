#[derive(Default)]
pub struct AppState {
    pub pnl_pct: f64,
    pub exposure_pct: f64,
    pub var_95: f64,
    pub kill_switch: bool,
    pub symbol: String,
}

impl AppState {
    pub fn refresh_portfolio(&mut self) {
                self.pnl_pct = 2.4;
        self.exposure_pct = 12.0;
        self.var_95 = 1240.0;
    }
}
