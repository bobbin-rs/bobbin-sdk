use ::bits;

pub trait GateEn {
    fn gate_en(&self) -> bits::U1;
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self; 
    fn gate_enable(&self) -> &Self { self.set_gate_en(true) }
    fn gate_disable(&self) -> &Self { self.set_gate_en(false) }
}    

pub trait GateRst {
    fn gate_rst(&self) -> bits::U1;
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self;
    fn toggle_gate_reset(&self) -> &Self { self.set_gate_rst(true).set_gate_rst(false) }
}    

pub trait GateSleepEn {
    fn gate_sleep_en(&self) -> bits::U1;
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self; 
    fn gate_sleep_enable(&self) -> &Self { self.set_gate_sleep_en(true) }
    fn gate_sleep_disable(&self) -> &Self { self.set_gate_sleep_en(false) }
}

pub trait GateDeepSleepEn {
    fn gate_deep_sleep_en(&self) -> bits::U1;
    fn set_gate_deep_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self; 
    fn gate_deep_sleep_enable(&self) -> &Self { self.set_gate_deep_sleep_en(true) }
    fn gate_deep_sleep_disable(&self) -> &Self { self.set_gate_deep_sleep_en(false) }
}

pub trait GateStopEn {
    fn gate_stop_en(&self) -> bits::U1;
    fn set_gate_stop_en<V: Into<bits::U1>>(&self, value: V) -> &Self; 
    fn gate_stop_enable(&self) -> &Self { self.set_gate_stop_en(true) }
    fn gate_stop_disable(&self) -> &Self { self.set_gate_stop_en(false) }
}
