#![no_std]
#![no_main]
#![allow(dead_code, unused_variables)]

#[macro_use]
extern crate can_cam as board;

use board::hal::i2c::*;
use board::hal::gpio::*;
use board::common::bits::*;

// Address = 0x35

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running I2C");
    
    let addr: U7 = U7::from(0b0101001);
    
    let i2c = I2C4;
    let i2c_port = GPIOD;
    let i2c_scl = PD12;
    let i2c_sda = PD13; 

    GPIOA.rcc_enable();
    PA6.mode_input().open_drain();
    PA5.mode_input().open_drain();

    i2c.rcc_enable();
    i2c_port.rcc_enable();

    i2c_scl.mode_i2c_scl(&i2c).open_drain();
    i2c_sda.mode_i2c_sda(&i2c).open_drain();

    println!("# Configuring I2C");

    // i2c.set_config(|c| c.set_timing(0x8.into(), 0x3.into(), 0x0.into(), 0xd.into(), 0x12.into()));
    i2c.set_enabled(false);
    // i2c.set_timingr(|_| Timingr(0x00300619));
    i2c.set_timingr(|r| r
        .set_presc(0x0)
        .set_scldel(0x3)
        .set_sdadel(0x0)
        .set_sclh(0xF)
        .set_scll(0x12)
    );

    println!("I2C Configuration Complete");

    let mut lox = Vl53L0x::new(i2c, addr);


    println!("Model ID:    {:02x}", lox.read_reg(IDENTIFICATION_MODEL_ID));
    println!("Revision ID: {:02x}", lox.read_reg(IDENTIFICATION_REVISION_ID));
    println!("I2C Addr:    {:02x}", lox.read_reg(I2C_SLAVE_DEVICE_ADDRESS));
    println!("SYSRANGE_START:                 {:02x}", lox.read_reg(SYSRANGE_START));
    println!("SYSTEM_THRESH_HIGH:             {:02x}", lox.read_reg(SYSTEM_THRESH_HIGH));
    println!("SYSTEM_THRESH_LOW :             {:02x}", lox.read_reg(SYSTEM_THRESH_LOW));
    println!("SYSTEM_SEQUENCE_CONFIG:         {:02x}", lox.read_reg(SYSTEM_SEQUENCE_CONFIG));
    println!("SYSTEM_RANGE_CONFIG:            {:02x}", lox.read_reg(SYSTEM_RANGE_CONFIG));
    println!("SYSTEM_INTERMEASUREMENT_PERIOD: {:02x}", lox.read_reg(SYSTEM_INTERMEASUREMENT_PERIOD));
    println!("SYSTEM_INTERRUPT_CONFIG_GPIO:   {:02x}", lox.read_reg(SYSTEM_INTERRUPT_CONFIG_GPIO));

    lox.init();

    loop {
        let mut range = lox.read_range_single_millimeters();        
        print!("{:5} | ", range);
        if range > 1024 {
            range = 1024;
        }
        for _ in 0..(range >> 4) {
            print!("=");            
        }
        println!("");
        board::delay(500);
    }
}

pub const SYSRANGE_START                              :u8 = 0x00;
pub const SYSTEM_THRESH_HIGH                          :u8 = 0x0C;
pub const SYSTEM_THRESH_LOW                           :u8 = 0x0E;
pub const SYSTEM_SEQUENCE_CONFIG                      :u8 = 0x01;
pub const SYSTEM_RANGE_CONFIG                         :u8 = 0x09;
pub const SYSTEM_INTERMEASUREMENT_PERIOD              :u8 = 0x04;
pub const SYSTEM_INTERRUPT_CONFIG_GPIO                :u8 = 0x0A;
pub const GPIO_HV_MUX_ACTIVE_HIGH                     :u8 = 0x84;
pub const SYSTEM_INTERRUPT_CLEAR                      :u8 = 0x0B;
pub const RESULT_INTERRUPT_STATUS                     :u8 = 0x13;
pub const RESULT_RANGE_STATUS                         :u8 = 0x14;
pub const RESULT_CORE_AMBIENT_WINDOW_EVENTS_RTN       :u8 = 0xBC;
pub const RESULT_CORE_RANGING_TOTAL_EVENTS_RTN        :u8 = 0xC0;
pub const RESULT_CORE_AMBIENT_WINDOW_EVENTS_REF       :u8 = 0xD0;
pub const RESULT_CORE_RANGING_TOTAL_EVENTS_REF        :u8 = 0xD4;
pub const RESULT_PEAK_SIGNAL_RATE_REF                 :u8 = 0xB6;
pub const ALGO_PART_TO_PART_RANGE_OFFSET_MM           :u8 = 0x28;
pub const I2C_SLAVE_DEVICE_ADDRESS                    :u8 = 0x8A;
pub const MSRC_CONFIG_CONTROL                         :u8 = 0x60;
pub const PRE_RANGE_CONFIG_MIN_SNR                    :u8 = 0x27;
pub const PRE_RANGE_CONFIG_VALID_PHASE_LOW            :u8 = 0x56;
pub const PRE_RANGE_CONFIG_VALID_PHASE_HIGH           :u8 = 0x57;
pub const PRE_RANGE_MIN_COUNT_RATE_RTN_LIMIT          :u8 = 0x64;
pub const FINAL_RANGE_CONFIG_MIN_SNR                  :u8 = 0x67;
pub const FINAL_RANGE_CONFIG_VALID_PHASE_LOW          :u8 = 0x47;
pub const FINAL_RANGE_CONFIG_VALID_PHASE_HIGH         :u8 = 0x48;
pub const FINAL_RANGE_CONFIG_MIN_COUNT_RATE_RTN_LIMIT :u8 = 0x44;
pub const PRE_RANGE_CONFIG_SIGMA_THRESH_HI            :u8 = 0x61;
pub const PRE_RANGE_CONFIG_SIGMA_THRESH_LO            :u8 = 0x62;
pub const PRE_RANGE_CONFIG_VCSEL_PERIOD               :u8 = 0x50;
pub const PRE_RANGE_CONFIG_TIMEOUT_MACROP_HI          :u8 = 0x51;
pub const PRE_RANGE_CONFIG_TIMEOUT_MACROP_LO          :u8 = 0x52;
pub const SYSTEM_HISTOGRAM_BIN                        :u8 = 0x81;
pub const HISTOGRAM_CONFIG_INITIAL_PHASE_SELECT       :u8 = 0x33;
pub const HISTOGRAM_CONFIG_READOUT_CTRL               :u8 = 0x55;
pub const FINAL_RANGE_CONFIG_VCSEL_PERIOD             :u8 = 0x70;
pub const FINAL_RANGE_CONFIG_TIMEOUT_MACROP_HI        :u8 = 0x71;
pub const FINAL_RANGE_CONFIG_TIMEOUT_MACROP_LO        :u8 = 0x72;
pub const CROSSTALK_COMPENSATION_PEAK_RATE_MCPS       :u8 = 0x20;
pub const MSRC_CONFIG_TIMEOUT_MACROP                  :u8 = 0x46;
pub const SOFT_RESET_GO2_SOFT_RESET_N                 :u8 = 0xBF;
pub const IDENTIFICATION_MODEL_ID                     :u8 = 0xC0;
pub const IDENTIFICATION_REVISION_ID                  :u8 = 0xC2;
pub const OSC_CALIBRATE_VAL                           :u8 = 0xF8;
pub const GLOBAL_CONFIG_VCSEL_WIDTH                   :u8 = 0x32;
pub const GLOBAL_CONFIG_SPAD_ENABLES_REF_0            :u8 = 0xB0;
pub const GLOBAL_CONFIG_SPAD_ENABLES_REF_1            :u8 = 0xB1;
pub const GLOBAL_CONFIG_SPAD_ENABLES_REF_2            :u8 = 0xB2;
pub const GLOBAL_CONFIG_SPAD_ENABLES_REF_3            :u8 = 0xB3;
pub const GLOBAL_CONFIG_SPAD_ENABLES_REF_4            :u8 = 0xB4;
pub const GLOBAL_CONFIG_SPAD_ENABLES_REF_5            :u8 = 0xB5;
pub const GLOBAL_CONFIG_REF_EN_START_SELECT           :u8 = 0xB6;
pub const DYNAMIC_SPAD_NUM_REQUESTED_REF_SPAD         :u8 = 0x4E;
pub const DYNAMIC_SPAD_REF_EN_START_OFFSET            :u8 = 0x4F;
pub const POWER_MANAGEMENT_GO1_POWER_FORCE            :u8 = 0x80;
pub const VHV_CONFIG_PAD_SCL_SDA__EXTSUP_HV           :u8 = 0x89;
pub const ALGO_PHASECAL_LIM                           :u8 = 0x30;
pub const ALGO_PHASECAL_CONFIG_TIMEOUT                :u8 = 0x30;

pub struct Vl53L0x {
    i2c: I2cPeriph,
    addr: U7,
    stop_variable: u8,
}

impl Vl53L0x {
    pub fn new<I: Into<I2cPeriph>>(i2c: I, addr: U7) -> Self {
        Vl53L0x {
            i2c: i2c.into(),
            addr: addr,
            stop_variable: 0,
        }
    }

    pub fn init(&mut self) {
        println!("Initializing");        

        println!("Set 2.8v IO");

        self.mod_reg(VHV_CONFIG_PAD_SCL_SDA__EXTSUP_HV, |v| v | 0x01);

        println!("Set I2C standard mode");

        self.write_reg(0x88, 0x00);        
        self.write_reg(0x80, 0x01);
        self.write_reg(0xFF, 0x01);
        self.write_reg(0x00, 0x00);
        self.stop_variable = self.read_reg(0x91);
        self.write_reg(0x00, 0x01);
        self.write_reg(0xFF, 0x00);
        self.write_reg(0x80, 0x00);        

        // disable SIGNAL_RATE_MSRC (bit 1) and SIGNAL_RATE_PRE_RANGE (bit 4) limit checks
        self.mod_reg(MSRC_CONFIG_CONTROL, |v| v | 0x12);

        // set final range signal rate limit to 0.25 MCPS (million counts per second)
        // self.set_signal_rate_limit(0.25);

        self.write_reg(SYSTEM_SEQUENCE_CONFIG, 0xFF);

        // VL53L0X_DataInit() end

        // VL53L0X_StaticInit() begin

        let (spad_count, spad_type_is_aperture) = self.get_spad_info();
        println!("{}, {}", spad_count, spad_type_is_aperture);

        // The SPAD map (RefGoodSpadMap) is read by VL53L0X_get_info_from_device() in
        // the API, but the same data seems to be more easily readable from
        // GLOBAL_CONFIG_SPAD_ENABLES_REF_0 through _6, so read it from there

        let mut spad_map = [
            self.read_reg(GLOBAL_CONFIG_SPAD_ENABLES_REF_0),
            self.read_reg(GLOBAL_CONFIG_SPAD_ENABLES_REF_1),
            self.read_reg(GLOBAL_CONFIG_SPAD_ENABLES_REF_2),
            self.read_reg(GLOBAL_CONFIG_SPAD_ENABLES_REF_3),
            self.read_reg(GLOBAL_CONFIG_SPAD_ENABLES_REF_4),
            self.read_reg(GLOBAL_CONFIG_SPAD_ENABLES_REF_5),
        ];
        println!("spad_map: {:?}", spad_map);
        
        // -- VL53L0X_set_reference_spads() begin (assume NVM values are valid)
        self.write_reg(0xFF, 0x01);
        self.write_reg(DYNAMIC_SPAD_REF_EN_START_OFFSET, 0x00);
        self.write_reg(DYNAMIC_SPAD_NUM_REQUESTED_REF_SPAD, 0x2C);
        self.write_reg(0xFF, 0x00);
        self.write_reg(GLOBAL_CONFIG_REF_EN_START_SELECT, 0xB4);

        // 12 is the first aperture spad
        let first_spad_to_enable = if spad_type_is_aperture { 12 } else { 0 };

        let mut spads_enabled = 0;

        for i in 0..48 {
            if i < first_spad_to_enable || spads_enabled == spad_count {
                // This bit is lower than the first one that should be enabled, or
                // (reference_spad_count) bits have already been enabled, so zero this bit        
                spad_map[i / 8] &= !(1 << (i % 8));                        
            } else if ((spad_map[i / 8] >> (i % 8)) & 0x1) != 0 {
                spads_enabled += 1;
            }
        }
        self.write_reg(GLOBAL_CONFIG_SPAD_ENABLES_REF_0, spad_map[0]);
        self.write_reg(GLOBAL_CONFIG_SPAD_ENABLES_REF_1, spad_map[1]);
        self.write_reg(GLOBAL_CONFIG_SPAD_ENABLES_REF_2, spad_map[2]);
        self.write_reg(GLOBAL_CONFIG_SPAD_ENABLES_REF_3, spad_map[3]);
        self.write_reg(GLOBAL_CONFIG_SPAD_ENABLES_REF_4, spad_map[4]);
        self.write_reg(GLOBAL_CONFIG_SPAD_ENABLES_REF_5, spad_map[5]);
        // -- VL53L0X_set_reference_spads() end
        // -- VL53L0X_load_tuning_settings() begin
        // DefaultTuningSettings from vl53l0x_tuning.h


        self.write_reg(0xFF, 0x01);
        self.write_reg(0x00, 0x00);

        self.write_reg(0xFF, 0x00);
        self.write_reg(0x09, 0x00);
        self.write_reg(0x10, 0x00);
        self.write_reg(0x11, 0x00);

        self.write_reg(0x24, 0x01);
        self.write_reg(0x25, 0xFF);
        self.write_reg(0x75, 0x00);

        self.write_reg(0xFF, 0x01);
        self.write_reg(0x4E, 0x2C);
        self.write_reg(0x48, 0x00);
        self.write_reg(0x30, 0x20);

        self.write_reg(0xFF, 0x00);
        self.write_reg(0x30, 0x09);
        self.write_reg(0x54, 0x00);
        self.write_reg(0x31, 0x04);
        self.write_reg(0x32, 0x03);
        self.write_reg(0x40, 0x83);
        self.write_reg(0x46, 0x25);
        self.write_reg(0x60, 0x00);
        self.write_reg(0x27, 0x00);
        self.write_reg(0x50, 0x06);
        self.write_reg(0x51, 0x00);
        self.write_reg(0x52, 0x96);
        self.write_reg(0x56, 0x08);
        self.write_reg(0x57, 0x30);
        self.write_reg(0x61, 0x00);
        self.write_reg(0x62, 0x00);
        self.write_reg(0x64, 0x00);
        self.write_reg(0x65, 0x00);
        self.write_reg(0x66, 0xA0);

        self.write_reg(0xFF, 0x01);
        self.write_reg(0x22, 0x32);
        self.write_reg(0x47, 0x14);
        self.write_reg(0x49, 0xFF);
        self.write_reg(0x4A, 0x00);

        self.write_reg(0xFF, 0x00);
        self.write_reg(0x7A, 0x0A);
        self.write_reg(0x7B, 0x00);
        self.write_reg(0x78, 0x21);

        self.write_reg(0xFF, 0x01);
        self.write_reg(0x23, 0x34);
        self.write_reg(0x42, 0x00);
        self.write_reg(0x44, 0xFF);
        self.write_reg(0x45, 0x26);
        self.write_reg(0x46, 0x05);
        self.write_reg(0x40, 0x40);
        self.write_reg(0x0E, 0x06);
        self.write_reg(0x20, 0x1A);
        self.write_reg(0x43, 0x40);

        self.write_reg(0xFF, 0x00);
        self.write_reg(0x34, 0x03);
        self.write_reg(0x35, 0x44);

        self.write_reg(0xFF, 0x01);
        self.write_reg(0x31, 0x04);
        self.write_reg(0x4B, 0x09);
        self.write_reg(0x4C, 0x05);
        self.write_reg(0x4D, 0x04);

        self.write_reg(0xFF, 0x00);
        self.write_reg(0x44, 0x00);
        self.write_reg(0x45, 0x20);
        self.write_reg(0x47, 0x08);
        self.write_reg(0x48, 0x28);
        self.write_reg(0x67, 0x00);
        self.write_reg(0x70, 0x04);
        self.write_reg(0x71, 0x01);
        self.write_reg(0x72, 0xFE);
        self.write_reg(0x76, 0x00);
        self.write_reg(0x77, 0x00);

        self.write_reg(0xFF, 0x01);
        self.write_reg(0x0D, 0x01);

        self.write_reg(0xFF, 0x00);
        self.write_reg(0x80, 0x01);
        self.write_reg(0x01, 0xF8);

        self.write_reg(0xFF, 0x01);
        self.write_reg(0x8E, 0x01);
        self.write_reg(0x00, 0x01);
        self.write_reg(0xFF, 0x00);
        self.write_reg(0x80, 0x00);

        // -- VL53L0X_load_tuning_settings() end

        // "Set interrupt config to new sample ready"
        // -- VL53L0X_SetGpioConfig() begin

        self.write_reg(SYSTEM_INTERRUPT_CONFIG_GPIO, 0x04);
        self.write_reg(GPIO_HV_MUX_ACTIVE_HIGH, self.read_reg(GPIO_HV_MUX_ACTIVE_HIGH) & !0x10); // active low
        self.write_reg(SYSTEM_INTERRUPT_CLEAR, 0x01);

        // -- VL53L0X_SetGpioConfig() end

        let measurement_timing_budget_us = self.get_measurement_timing_budget();
        println!("budget: {}", measurement_timing_budget_us);

        // "Disable MSRC and TCC by default"
        // MSRC = Minimum Signal Rate Check
        // TCC = Target CentreCheck
        // -- VL53L0X_SetSequenceStepEnable() begin

        self.write_reg(SYSTEM_SEQUENCE_CONFIG, 0xE8);

        // -- VL53L0X_SetSequenceStepEnable() end

        // "Recalculate timing budget"
        self.set_measurement_timing_budget(measurement_timing_budget_us);
        
        // VL53L0X_StaticInit() end

        // VL53L0X_PerformRefCalibration() begin (VL53L0X_perform_ref_calibration())

        // -- VL53L0X_perform_vhv_calibration() begin

        self.write_reg(SYSTEM_SEQUENCE_CONFIG, 0x01);

        self.perform_single_ref_calibration(0x40);
        // if (!self.performSingleRefCalibration(0x40)) { return false; }

        // -- VL53L0X_perform_vhv_calibration() end

        // -- VL53L0X_perform_phase_calibration() begin

        self.write_reg(SYSTEM_SEQUENCE_CONFIG, 0x02);

        self.perform_single_ref_calibration(0x00);
        // if (!performSingleRefCalibration(0x00)) { return false; }

        // -- VL53L0X_perform_phase_calibration() end

        // "restore the previous Sequence Config"
        self.write_reg(SYSTEM_SEQUENCE_CONFIG, 0xE8);

        // VL53L0X_PerformRefCalibration() end

        println!("Init Complete");
    }

    pub fn transfer(&self, tx_buf: &[u8], rx_buf: &mut [u8]) {
        self.i2c.transfer(self.addr, tx_buf, rx_buf);
    }

    pub fn mod_reg<F: FnOnce(u8) -> u8>(&self, reg: u8, f: F) {        
        self.write_reg(reg, f(self.read_reg(reg)))
    }

    pub fn read_reg(&self, reg: u8) -> u8 {
        self.i2c.read_reg(self.addr, reg)
    }
    
    pub fn write_reg(&self, reg: u8, val: u8) {
        // println!("0x{:02x} . 0x{:02x}", reg, val);
        self.i2c.write_reg(self.addr, reg, val);
    }

    pub fn read_reg_16(&self, reg: u8) -> u16 {
        let tx_buf = [reg];
        let mut rx_buf = [0u8; 2];
        self.transfer(&tx_buf, &mut rx_buf);
        (rx_buf[0] as u16) << 8 | (rx_buf[1] as u16)
    }

    pub fn write_reg_16(&self, reg: u8, val: u16) {
        let tx_buf = [reg, (val >> 8) as u8, val as u8];
        let mut rx_buf = [];
        self.transfer(&tx_buf, &mut rx_buf);
    }

    // Set the return signal rate limit check value in units of MCPS (mega counts
    // per second). "This represents the amplitude of the signal reflected from the
    // target and detected by the device"; setting this limit presumably determines
    // the minimum measurement necessary for the sensor to report a valid reading.
    // Setting a lower limit increases the potential range of the sensor but also
    // seems to increase the likelihood of getting an inaccurate reading because of
    // unwanted reflections from objects other than the intended target.
    // Defaults to 0.25 MCPS as initialized by the ST API and this library.
    pub fn set_signal_rate_limit(&self, _limit_mcps: f32) -> bool {
        unimplemented!()
        // if (limit_Mcps < 0 || limit_Mcps > 511.99) { return false; }

        //   // Q9.7 fixed point format (9 integer bits, 7 fractional bits)
        // self.write_reg_u16(FINAL_RANGE_CONFIG_MIN_COUNT_RATE_RTN_LIMIT, limit_Mcps * (1 << 7));
        // return true;
    }


    // Get reference SPAD (single photon avalanche diode) count and type
    // based on VL53L0X_get_info_from_device(),
    // but only gets reference SPAD count and type
    pub fn get_spad_info(&self) -> (u8, bool) {
        self.write_reg(0x80, 0x01);
        self.write_reg(0xFF, 0x01);
        self.write_reg(0x00, 0x00);

        self.write_reg(0xFF, 0x06);
        self.write_reg(0x83, self.read_reg(0x83) | 0x04);
        self.write_reg(0xFF, 0x07);
        self.write_reg(0x81, 0x01);

        self.write_reg(0x80, 0x01);

        self.write_reg(0x94, 0x6b);
        self.write_reg(0x83, 0x00);
        while self.read_reg(0x83) == 0x00 {}

        self.write_reg(0x83, 0x01);
        let tmp = self.read_reg(0x92);

        let count = tmp & 0x7f;
        let type_is_aperture = ((tmp >> 7) & 0x01) != 0;

        self.write_reg(0x81, 0x00);
        self.write_reg(0xFF, 0x06);
        self.write_reg(0x83, self.read_reg(0x83) & !0x04);
        self.write_reg(0xFF, 0x01);
        self.write_reg(0x00, 0x01);

        self.write_reg(0xFF, 0x00);
        self.write_reg(0x80, 0x00);
        (count, type_is_aperture)
    }

    // Get the measurement timing budget in microseconds
    // based on VL53L0X_get_measurement_timing_budget_micro_seconds()
    // in us
    pub fn get_measurement_timing_budget(&self) -> u32 {    
        let mut enables: SequenceStepEnables = SequenceStepEnables::default();
        let mut timeouts: SequenceStepTimeouts = SequenceStepTimeouts::default();

        const START_OVERHEAD: u32     = 1910; // note that this is different than the value in set_
        const END_OVERHEAD: u32        = 960;
        const MSRC_OVERHEAD: u32       = 660;
        const TCC_OVERHEAD: u32        = 590;
        const DSS_OVERHEAD: u32        = 690;
        const PRE_RANGE_OVERHEAD: u32   = 660;
        const FINAL_RANGE_OVERHEAD: u32 = 550;

        // "Start and end overhead times always present"
        let mut budget_us: u32 = START_OVERHEAD + END_OVERHEAD;

        self.get_sequence_step_enables(&mut enables);
        self.get_sequence_step_timeouts(&mut enables, &mut timeouts);

        if enables.tcc {        
            budget_us += timeouts.msrc_dss_tcc_us + TCC_OVERHEAD;
        }

        if enables.dss {
            budget_us += 2 * (timeouts.msrc_dss_tcc_us + DSS_OVERHEAD);
        } else if enables.msrc {        
            budget_us += timeouts.msrc_dss_tcc_us + MSRC_OVERHEAD;
        }

        if enables.pre_range {
            budget_us += timeouts.pre_range_us + PRE_RANGE_OVERHEAD;
        }

        if enables.final_range {
            budget_us += timeouts.final_range_us + FINAL_RANGE_OVERHEAD;
        }
        budget_us
    }    


    // Set the measurement timing budget in microseconds, which is the time allowed
    // for one measurement; the ST API and this library take care of splitting the
    // timing budget among the sub-steps in the ranging sequence. A longer timing
    // budget allows for more accurate measurements. Increasing the budget by a
    // factor of N decreases the range measurement standard deviation by a factor of
    // sqrt(N). Defaults to about 33 milliseconds; the minimum is 20 ms.
    // based on VL53L0X_set_measurement_timing_budget_micro_seconds()
    fn set_measurement_timing_budget(&self, budget_us: u32) -> bool {
        let mut enables: SequenceStepEnables = SequenceStepEnables::default();
        let mut timeouts: SequenceStepTimeouts = SequenceStepTimeouts::default();

        const START_OVERHEAD: u32     = 1320; // note that this is different than the value in set_
        const END_OVERHEAD: u32        = 960;
        const MSRC_OVERHEAD: u32       = 660;
        const TCC_OVERHEAD: u32        = 590;
        const DSS_OVERHEAD: u32        = 690;
        const PRE_RANGE_OVERHEAD: u32   = 660;
        const FINAL_RANGE_OVERHEAD: u32 = 550;
        const MIN_TIMING_BUDGET: u32 = 20000;

        if budget_us < MIN_TIMING_BUDGET { return false; }

        let mut used_budget_us: u32  = START_OVERHEAD + END_OVERHEAD;

        self.get_sequence_step_enables(&mut enables);
        self.get_sequence_step_timeouts(&enables, &mut timeouts);

        if enables.tcc {        
            used_budget_us += timeouts.msrc_dss_tcc_us + TCC_OVERHEAD;
        }

        if enables.dss {
            used_budget_us += 2 * (timeouts.msrc_dss_tcc_us + DSS_OVERHEAD);
        } else if enables.msrc {
            used_budget_us += timeouts.msrc_dss_tcc_us + MSRC_OVERHEAD;
        }

        if enables.pre_range {
            used_budget_us += timeouts.pre_range_us + PRE_RANGE_OVERHEAD;
        }

        if enables.final_range {
            used_budget_us += FINAL_RANGE_OVERHEAD;

            // "Note that the final range timeout is determined by the timing
            // budget and the sum of all other timeouts within the sequence.
            // If there is no room for the final range timeout, then an error
            // will be set. Otherwise the remaining time will be applied to
            // the final range."

            if used_budget_us > budget_us {            
                // "Requested timeout too big."
                return false;
            }

            let final_range_timeout_us: u32 = budget_us - used_budget_us;

            // set_sequence_step_timeout() begin
            // (SequenceStepId == VL53L0X_SEQUENCESTEP_FINAL_RANGE)

            // "For the final range timeout, the pre-range timeout
            //  must be added. To do this both final and pre-range
            //  timeouts must be expressed in macro periods MClks
            //  because they have different vcsel periods."

            let mut final_range_timeout_mclks: u32 = self.timeout_microseconds_to_mclks(final_range_timeout_us,
                                        timeouts.final_range_vcsel_period_pclks);

            if enables.pre_range {
                final_range_timeout_mclks += timeouts.pre_range_mclks as u32;
            }

            self.write_reg_16(FINAL_RANGE_CONFIG_TIMEOUT_MACROP_HI, self.encode_timeout(final_range_timeout_mclks as u16));

            // set_sequence_step_timeout() end
            // measurement_timing_budget_us = budget_us; // store for internal reuse            
        }
        return true;
    }


    // Get sequence step enables
    // based on VL53L0X_GetSequenceStepEnables()
    fn get_sequence_step_enables(&self, enables: &mut SequenceStepEnables) {
        let sequence_config = self.read_reg(SYSTEM_SEQUENCE_CONFIG);

        enables.tcc          = (sequence_config >> 4) & 0x1 != 0;
        enables.dss          = (sequence_config >> 3) & 0x1 != 0;
        enables.msrc         = (sequence_config >> 2) & 0x1 != 0;
        enables.pre_range    = (sequence_config >> 6) & 0x1 != 0;
        enables.final_range  = (sequence_config >> 7) & 0x1 != 0;
    }

    // Get sequence step timeouts
    // based on get_sequence_step_timeout(),
    // but gets all timeouts instead of just the requested one, and also stores
    // intermediate values
    fn get_sequence_step_timeouts(&self, enables: &SequenceStepEnables, timeouts: &mut SequenceStepTimeouts) {

        timeouts.pre_range_vcsel_period_pclks = self.get_vcsel_pulse_period(VcselPeriodType::VcselPeriodPreRange);

        timeouts.msrc_dss_tcc_mclks = (self.read_reg(MSRC_CONFIG_TIMEOUT_MACROP) + 1) as u16;
        timeouts.msrc_dss_tcc_us = self.timeout_mclks_to_microseconds(timeouts.msrc_dss_tcc_mclks, timeouts.pre_range_vcsel_period_pclks);

        timeouts.pre_range_mclks = self.decode_timeout(self.read_reg_16(PRE_RANGE_CONFIG_TIMEOUT_MACROP_HI));
        timeouts.pre_range_us = self.timeout_mclks_to_microseconds(timeouts.pre_range_mclks, timeouts.pre_range_vcsel_period_pclks);

        timeouts.final_range_vcsel_period_pclks = self.get_vcsel_pulse_period(VcselPeriodType::VcselPeriodFinalRange);

        timeouts.final_range_mclks = self.decode_timeout(self.read_reg_16(FINAL_RANGE_CONFIG_TIMEOUT_MACROP_HI));

        if enables.pre_range {
            timeouts.final_range_mclks -= timeouts.pre_range_mclks;
        }

        timeouts.final_range_us = self.timeout_mclks_to_microseconds(timeouts.final_range_mclks, timeouts.final_range_vcsel_period_pclks);
    }

    fn decode_vcsel_period(&self, reg_val: u8) -> u8 {
        (reg_val + 1) << 1
    }

    fn encode_vcsel_period(&self, period_pclks: u8) -> u8 {
        (period_pclks >> 1) - 1
    }

    fn get_vcsel_pulse_period(&self, period_type: VcselPeriodType) -> u8 {
        match period_type {
            VcselPeriodType::VcselPeriodPreRange => {
                self.decode_vcsel_period(self.read_reg(PRE_RANGE_CONFIG_VCSEL_PERIOD))
            },
            VcselPeriodType::VcselPeriodFinalRange => {
                self.decode_vcsel_period(self.read_reg(FINAL_RANGE_CONFIG_VCSEL_PERIOD))
            },
        }
    }

    fn set_vcsel_pulse_period(&self, period_type: VcselPeriodType, period_pclks: u8) {
        unimplemented!()
    }

    fn decode_timeout(&self, reg_val: u16) -> u16 {
        // format: "(LSByte * 2^MSByte) + 1"
        ((reg_val & 0x00ff) << (reg_val & 0xff00) as u8) + 1
//   return (uint16_t)((reg_val & 0x00FF) <<
//          (uint16_t)((reg_val & 0xFF00) >> 8)) + 1;
    }
    fn encode_timeout(&self, timeout_mclks: u16) -> u16 {
        // format: "(LSByte * 2^MSByte) + 1"

        let mut ls_byte: u32;
        let mut ms_byte: u16 = 0;

        if timeout_mclks > 0 {
            ls_byte = (timeout_mclks as u32) - 1;

            while (ls_byte & 0xFFFFFF00) > 0 {
                ls_byte >>= 1;
                ms_byte += 1;
            }
            ms_byte << 8 | (ls_byte & 0xFF) as u16
        } else { 
            0
        }
    }

    fn calc_macro_period(&self, vcsel_period_pclks: u8) -> u32 {
        ((2304 * (vcsel_period_pclks as u32) * 1655) + 500) / 1000
        // ((((uint32_t)2304 * (vcsel_period_pclks) * 1655) + 500) / 1000)        
    }

    fn timeout_mclks_to_microseconds(&self, timeout_period_mclks: u16, vcsel_period_pclks: u8) -> u32 {
        let timeout_period_mclks: u32 = timeout_period_mclks as u32;
        let macro_period_ns: u32 = self.calc_macro_period(vcsel_period_pclks);
        ((timeout_period_mclks * macro_period_ns) + (macro_period_ns / 2)) / 1000
    }

    fn timeout_microseconds_to_mclks(&self, timeout_period_us: u32, vcsel_period_pclks: u8) -> u32 {
        let timeout_period_u2: u32 = timeout_period_us as u32;
        let macro_period_ns: u32 = self.calc_macro_period(vcsel_period_pclks);
        ((timeout_period_us * 1000) + (macro_period_ns / 2)) / macro_period_ns
    }    
        
    // based on VL53L0X_perform_single_ref_calibration()
    fn perform_single_ref_calibration(&self, vhv_init_byte: u8) {
        self.write_reg(SYSRANGE_START, 0x01 | vhv_init_byte); // VL53L0X_REG_SYSRANGE_MODE_START_STOP

        while (self.read_reg(RESULT_INTERRUPT_STATUS) & 0x07) == 0 {}

        self.write_reg(SYSTEM_INTERRUPT_CLEAR, 0x01);
        self.write_reg(SYSRANGE_START, 0x00);        
    }
        
    // Returns a range reading in millimeters when continuous mode is active
    // (readRangeSingleMillimeters() also calls this function after starting a
    // single-shot range measurement)
    fn read_range_continuous_millimeters(&self) -> u16 {
        while self.read_reg(RESULT_INTERRUPT_STATUS) & 0x7 == 0 {}
        // assumptions: Linearity Corrective Gain is 1000 (default);
        // fractional ranging is not enabled
        let range: u16 = self.read_reg_16(RESULT_RANGE_STATUS + 10);
        self.write_reg(SYSTEM_INTERRUPT_CLEAR, 0x01);
        return range;
    }

    // Performs a single-shot range measurement and returns the reading in
    // millimeters
    // based on VL53L0X_PerformSingleRangingMeasurement()
    pub fn read_range_single_millimeters(&self) -> u16 {
        self.write_reg(0x80, 0x01);
        self.write_reg(0xFF, 0x01);
        self.write_reg(0x00, 0x00);
        self.write_reg(0x91, self.stop_variable);
        self.write_reg(0x00, 0x01);
        self.write_reg(0xFF, 0x00);
        self.write_reg(0x80, 0x00);

        self.write_reg(SYSRANGE_START, 0x01);

        while self.read_reg(SYSRANGE_START) & 0x01 != 0 {}

        return self.read_range_continuous_millimeters();
    }
    
}

pub enum VcselPeriodType {
    VcselPeriodPreRange, 
    VcselPeriodFinalRange    
}

#[derive(Default, Debug)]
pub struct SequenceStepEnables {
    tcc: bool,
    msrc: bool,
    dss: bool,
    pre_range: bool,
    final_range: bool,
}

#[derive(Default, Debug)]
pub struct SequenceStepTimeouts {
    pre_range_vcsel_period_pclks: u8,
    final_range_vcsel_period_pclks: u8,

    msrc_dss_tcc_mclks: u16,
    pre_range_mclks: u16,
    final_range_mclks: u16,

    msrc_dss_tcc_us: u32,
    pre_range_us: u32,
    final_range_us: u32,
}