#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum CtrlReg0Value {
    PullUpConnectedSdoSa0Pin,
    PullUpDisconnectedSdoSa0Pin = 16,
}

impl Default for CtrlReg0Value {
    fn default() -> Self {
        CtrlReg0Value::PullUpDisconnectedSdoSa0Pin
    }
}
