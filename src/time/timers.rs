use crate::{
    cpu::interrupt::{request_interrupt, InterruptType},
    mem::io_regs::IoReg,
    sys::Sys,
    util::bits::Bits,
};

pub const CPU_PERIOD_MCYCLES: u32 = 1;
pub const DIV_PERIOD_MCYCLES: u32 = 64;
pub const TAC_CLK_0_PERIOD_MCYCLES: u32 = 256;
pub const TAC_CLK_1_PERIOD_MCYCLES: u32 = 4;
pub const TAC_CLK_2_PERIOD_MCYCLES: u32 = 16;
pub const TAC_CLK_3_PERIOD_MCYCLES: u32 = 64;

pub fn update_timer_regs(sys: &mut Sys) {
    // DIV: incs every 16384 Hz; Writing any sets to 0x00; reset on STOP; doesnt tick until stop mode ends)
    // TIMA: incs at freq specified in TAC; when overflows, it is reset to value in TMA and an interrupt is reqd
    // TMA: determines TIMA reset value after overflow
    // TAC: .2: enable; .1-0: clock select;

    if !sys.speed_ctrl.is_stop_active() {
        let div_ticked = sys.div_timer_clock.update_and_check();

        if div_ticked {
            sys.mem.io_regs.mut_(IoReg::Div, |div| {
                let div_ = u8::wrapping_add(*div, 1);
                if div_ == 0 {
                    // DIV overflow
                }
                *div = div_;
            });
        }
    }

    // Update TIMA
    let tac = sys.mem.io_regs.get(IoReg::Tac);
    let enable = tac.bit(2) == 1; // todo unused
    let clock_sel = tac.bits(1, 0);
    let tima_clk_period = match clock_sel {
        0 => TAC_CLK_0_PERIOD_MCYCLES,
        1 => TAC_CLK_1_PERIOD_MCYCLES,
        2 => TAC_CLK_2_PERIOD_MCYCLES,
        3 => TAC_CLK_3_PERIOD_MCYCLES,
        _ => unreachable!(),
    };

    sys.tima_timer_clock.set_period(tima_clk_period);

    if enable {
        let tima_ticked = sys.tima_timer_clock.update_and_check();

        if tima_ticked {
            let tima = sys.mem.io_regs.get(IoReg::Tima);
            let tima_ = u8::wrapping_add(tima, 1);
            if tima_ == 0 {
                // TIMA overflow
                let tma = sys.mem.io_regs.get(IoReg::Tma);
                sys.mem.io_regs.set(IoReg::Tima, tma);
                request_interrupt(sys, InterruptType::Timer);
            }
            sys.mem.io_regs.set(IoReg::Tima, tima_);
        }
    }
}
