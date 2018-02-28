use ::bits;

pub trait GateEn {
    fn en(&self) -> bits::U1;
    fn set_en<V: Into<bits::U1>>(&self, value: V) -> &Self; 
    fn enable(&self) -> &Self { self.set_en(true) }
    fn disable(&self) -> &Self { self.set_en(false) }
}    

pub trait GateRst {
    fn rst(&self) -> bits::U1;
    fn set_rst<V: Into<bits::U1>>(&self, value: V) -> &Self;
    fn toggle_reset(&self) -> &Self { self.set_rst(true).set_rst(false) }
}    

pub trait GateSleepEn {
    fn sleep_en(&self) -> bits::U1;
    fn set_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self; 
    fn sleep_enable(&self) -> &Self { self.set_sleep_en(true) }
    fn sleep_disable(&self) -> &Self { self.set_sleep_en(false) }
}

pub trait GateDeepSleepEn {
    fn deep_sleep_en(&self) -> bits::U1;
    fn set_deep_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self; 
    fn deep_sleep_enable(&self) -> &Self { self.set_deep_sleep_en(true) }
    fn deep_sleep_disable(&self) -> &Self { self.set_deep_sleep_en(false) }
}

pub trait GateStopEn {
    fn stop_en(&self) -> bits::U1;
    fn set_stop_en<V: Into<bits::U1>>(&self, value: V) -> &Self; 
    fn stop_enable(&self) -> &Self { self.set_stop_en(true) }
    fn stop_disable(&self) -> &Self { self.set_stop_en(false) }
}
