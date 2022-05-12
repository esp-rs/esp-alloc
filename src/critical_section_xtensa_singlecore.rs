use core::arch::asm;

use xtensa_lx::interrupt::set_mask;
use xtensa_lx_rt::interrupt::CpuInterruptLevel;

static mut LAST_ENABLED_INTERRUPT_MASK: u32 = 0;

pub(crate) struct XtensaSingleCoreCriticalSection;

/// WARNING: THIS IS NOT MULTICORE SAFE
unsafe impl critical_section::Impl for XtensaSingleCoreCriticalSection {
    unsafe fn acquire() -> u8 {
        let ps = disable_interrupts();
        // keep debug
        let mask = set_mask(CpuInterruptLevel::Level6.mask());
        set_ps(ps);

        if mask & !CpuInterruptLevel::Level6.mask() != 0 {
            LAST_ENABLED_INTERRUPT_MASK = mask;
            1
        } else {
            0
        }
    }

    unsafe fn release(token: u8) {
        if token != 0 {
            set_mask(LAST_ENABLED_INTERRUPT_MASK);
        }
    }
}

#[inline(always)]
unsafe fn disable_interrupts() -> u32 {
    let mask;
    asm!("
        rsil {0}, 3
        ",
        out(reg) mask, options(nostack)
    );
    mask
}

#[inline(always)]
unsafe fn set_ps(ps: u32) {
    asm!("
        wsr {0}, PS
        ",
        in(reg) ps, options(nostack)
    );
}
