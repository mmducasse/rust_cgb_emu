use crate::{
    cart::cart::Cart,
    cpu::{exec::execute_next_instr, interrupt::try_handle_interrupts, regs::CpuRegs},
    debug::{self, debug_state},
    mem::{io_regs::IoReg, mem::Mem},
    other::{emu::Emu, joypad::handle_joypad_inputs},
    ppu::ppu::{print_ppu, update_ppu, Ppu},
    time::{
        clock::Clock,
        timers::{
            update_timer_regs, CPU_PERIOD_MCYCLES, DIV_PERIOD_MCYCLES, TAC_CLK_0_PERIOD_MCYCLES,
        },
    },
};

use super::{
    init::init,
    options::Options,
    speed::{update_speed_ctrl, SpeedControl},
};

/// Represents the state of the emulated Game Boy system.
pub struct Sys {
    pub options: Options,
    pub emu: Emu,
    pub speed_ctrl: SpeedControl,

    pub mem: Mem,
    pub ppu: Ppu,
    pub regs: CpuRegs,

    pub cpu_clock: Clock,
    pub div_timer_clock: Clock,
    pub tima_timer_clock: Clock,

    pub cpu_delay_ticks: u32,

    pub cpu_enable: bool,
    pub lcd_enable: bool,
    pub interrupt_master_enable: bool,

    pub hard_lock: bool,
    pub is_render_pending: bool,
}

impl Sys {
    pub fn new(options: Options, cart: Cart) -> Self {
        let mut sys = Self {
            options,
            emu: Emu::default(),
            speed_ctrl: SpeedControl::new(),

            mem: Mem::new(cart),
            ppu: Ppu::new(),
            regs: CpuRegs::new(),

            cpu_clock: Clock::new("CPU", CPU_PERIOD_MCYCLES),
            div_timer_clock: Clock::new("DIV", DIV_PERIOD_MCYCLES),
            tima_timer_clock: Clock::new("TIMA", TAC_CLK_0_PERIOD_MCYCLES),

            cpu_delay_ticks: 0,

            cpu_enable: true,
            lcd_enable: true,
            interrupt_master_enable: false,

            hard_lock: false,
            is_render_pending: false,
        };

        init(&mut sys);

        return sys;
    }

    pub fn is_cgb_mode(&self) -> bool {
        self.mem.cart.header().compatibility_mode().is_cgb_only()
    }

    pub fn run_one_m_cycle(&mut self) {
        update_speed_ctrl(self);

        if !self.speed_ctrl.is_stop_active() && self.cpu_clock.update_and_check() {
            self.cpu_delay_ticks = u32::saturating_sub(self.cpu_delay_ticks, 1);
            if self.cpu_delay_ticks == 0 {
                try_handle_interrupts(self);
                if self.cpu_enable {
                    self.cpu_delay_ticks = execute_next_instr(self);
                }
            }
        }

        update_ppu(self);
        update_timer_regs(self);
        handle_joypad_inputs(self);

        ///////// DEBUG //////////////////////////////////////////////
        if let Some(kill_after_nop_count) = debug_state().config.kill_after_nop_count {
            if debug_state().nop_count >= kill_after_nop_count {
                debug::fail("Debug max NOP count exceeded.");
            }
        }

        if let Some(kill_after_ticks) = debug_state().config.kill_after_cpu_ticks {
            if self.cpu_clock.debug_total_ticks >= kill_after_ticks {
                debug::fail("Debug kill time elapsed.");
            }
        }

        if let Some(failure) = debug::get_failure() {
            println!("FAILURE: {}", failure);
            //debug::print_system_state(&self);
            self.hard_lock = true;
            return;
        }

        //////////////////////////////////////////////////////////////

        return;
    }

    pub fn print(&self) {
        self.regs.print();
        println!("IME={}", self.interrupt_master_enable);
        println!("IE={:0>8b}", self.mem.io_regs.get(IoReg::Ie));
        println!("IF={:0>8b}", self.mem.io_regs.get(IoReg::If));

        print_ppu(self);

        self.cpu_clock.print();
        self.div_timer_clock.print();
        self.tima_timer_clock.print();
    }
}
