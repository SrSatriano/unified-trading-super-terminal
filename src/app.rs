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
        crate::connectors::mock::refresh_mock(self);
    }
}
