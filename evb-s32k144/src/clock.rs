use ::chip::scg::{self, SCG};

// 8 MHz Crystal on PTB6 / PTB7
//   into SOSC

//
// Target clock configuration is normal RUN mode
// VCO_CLK = 320MHz, SPLL_CLK = 160MHz
//   CORE_CLK = 80Mhz
//   SYS_CLK = 80MHz
//   BUS_CLK = 40MHz
//   FLASH_CLK = 26.67Mhz

// SCG Configuration
//    SCG_SOSCCFG[RANGE] = 0b11 (High Frequency Range)
//    

// Transitions
//   RESET = FIRC
//   FIRC => SOSC
//   SOSC => SYSPLL

// System Oscillator Clock (SOSC) mode is entered when all the following conditions occur:
// • RUN MODE: 0001 is written to RCCR[SCS].
// HSRUN MODE: 0001 is written to HCCR[SCS].
// • SOSCEN = 1
// • SOSCVLD = 1
// In SOSC mode, SCGCLKOUT and system clocks are derived from the external System Oscillator Clock (SOSC).

// Sys PLL (SPLL) mode is entered when all the following conditions occur:
// • RUN MODE: 0110 is written to RCCR[SCS].
// HSRUN MODE: 0110 is written to HCCR[SCS].
// • SPLLEN = 1
// • SPLLVLD = 1

// In SPLL mode, the SCGCLKOUT and system clocks are derived from the output of PLL which is controlled 
// by the System Oscillator (SOSC) clock. The selected PLL clock frequency locks to a multiplication factor, 
// as specified by its corresponding SCG_SPLLCFG[MULT], times the selected PLL reference frequency. 
// The PLL's programmable reference divider must be configured to produce a valid PLL reference clock. 
// This divide value is defined by the SCG_SPLLCFG[PREDIV] bits.

pub fn init() {
    
    unsafe {

        // Setup SOSC

        // Set SOSC Output Dividers
        SCG.set_soscdiv(scg::Soscdiv(0)
            .set_soscdiv1(0b001)
            .set_soscdiv2(0b001)
        );

        // Set SOSC Configuration
        SCG.set_sosccfg(scg::Sosccfg(0)
            .set_range(0b11) // High Speed Range
            .set_hgo(0) // Low Gain
            .set_erefs(1) // Use Internal Reference Clock
        );

        // Enable SOSC
        SCG.set_sosccsr(scg::Sosccsr(0)
            .set_soscen(1)
        );

        // Wait for SOSC Valid
        while SCG.sosccsr().soscvld() == 0 {}

        // Setup SPLL

        // Disable PLL
        SCG.with_spllcsr(|r| r.set_spllen(0));

        // Set PLL Output Dividers
        SCG.set_splldiv(scg::Splldiv(0)
            .set_splldiv1(0b010) // Divide by 2
            .set_splldiv2(0b011) // Divide by 4
        );

        // Set PLL Configuration
        SCG.set_spllcfg(scg::Spllcfg(0)
            .set_prediv(0b000) // Divide by 1
            .set_mult(0b11000) // Multiply by 40
        );
    
        // Enable PLL
        SCG.with_spllcsr(|r| r.set_spllen(1));

        // Wait for PLL Valid
        while SCG.spllcsr().spllvld() == 0 {}

        // Switch to SPLL and set multipliers
        SCG.set_rccr(scg::Rccr(0)
            .set_scs(0b0110)
            .set_divcore(0b0001)
            .set_divbus(0b0001)
            .set_divslow(0b0010)
        );
        // Wait for clock source to be set
        while SCG.csr().scs() != 0b0110 {}
    }
}