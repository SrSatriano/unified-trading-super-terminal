use crate::app::AppState;

pub fn refresh_mock(state: &mut AppState) {
    state.symbol = "BTCUSDT".into();
    state.pnl_pct = 2.4;
    state.exposure_pct = 12.0;
    state.var_95 = 1240.0;
}
