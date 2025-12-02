/// BIOS Tests - Separated test module
use crate::bios::*;

#[test]
fn test_bios_creation() {
    let bios = Bios::new();
    assert!(!bios.is_halted());
    assert!(!bios.is_waiting());
}

#[test]
fn test_bios_reset() {
    let mut bios = Bios::new();
    bios.halted = true;
    bios.waiting_for_interrupt = true;

    bios.reset();
    assert!(!bios.is_halted());
    assert!(!bios.is_waiting());
}

#[test]
fn test_bios_halt() {
    let mut bios = Bios::new();
    let (should_halt, should_wait) = bios.handle_swi(SWI_HALT);

    assert!(should_halt);
    assert!(!should_wait);
    assert!(bios.is_halted());
}

#[test]
fn test_bios_stop() {
    let mut bios = Bios::new();
    let (should_halt, should_wait) = bios.handle_swi(SWI_STOP);

    assert!(should_halt);
    assert!(!should_wait);
    assert!(bios.is_halted());
}

#[test]
fn test_bios_vblank_wait() {
    let mut bios = Bios::new();
    let (should_halt, should_wait) = bios.handle_swi(SWI_VBLANK_INTR_WAIT);

    assert!(!should_halt);
    assert!(should_wait);
    assert!(bios.is_waiting());
}

#[test]
fn test_bios_intr_wait() {
    let mut bios = Bios::new();
    let (should_halt, should_wait) = bios.handle_swi(SWI_INTR_WAIT);

    assert!(!should_halt);
    assert!(should_wait);
    assert!(bios.is_waiting());
}

#[test]
fn test_bios_clear_halt() {
    let mut bios = Bios::new();
    bios.halted = true;

    bios.clear_halt();
    assert!(!bios.is_halted());
}

#[test]
fn test_bios_clear_wait() {
    let mut bios = Bios::new();
    bios.waiting_for_interrupt = true;

    bios.clear_wait();
    assert!(!bios.is_waiting());
}

#[test]
fn test_div_normal() {
    let result = div(10, 3);
    assert_eq!(result.quotient, 3);
    assert_eq!(result.remainder, 1);
    assert_eq!(result.abs_quotient, 3);
}

#[test]
fn test_div_negative() {
    let result = div(-10, 3);
    assert_eq!(result.quotient, -3);
    assert_eq!(result.remainder, -1);
    assert_eq!(result.abs_quotient, 3);
}

#[test]
fn test_div_by_zero() {
    let result = div(10, 0);
    assert_eq!(result.quotient, i32::MAX);
    assert_eq!(result.remainder, 10);

    let result_neg = div(-10, 0);
    assert_eq!(result_neg.quotient, i32::MIN);
    assert_eq!(result_neg.remainder, -10);
}

#[test]
fn test_sqrt_perfect() {
    let result = sqrt(16);
    assert_eq!(result.result, 4);

    let result = sqrt(64);
    assert_eq!(result.result, 8);
}

#[test]
fn test_sqrt_imperfect() {
    let result = sqrt(10);
    assert_eq!(result.result, 3);

    let result = sqrt(50);
    assert_eq!(result.result, 7);
}

#[test]
fn test_arctan_zero() {
    let result = arctan(0);
    assert_eq!(result, 0);
}

#[test]
fn test_arctan_positive() {
    let result = arctan(8192); // 0.5 in fixed-point
    assert!(result > 0);
    assert!(result < 8192);
}

#[test]
fn test_arctan2_quadrants() {
    // arctan2 returns 0-65535 range (full circle)
    // Just verify it doesn't crash with various inputs
    let _r1 = arctan2(100, 100);
    let _r2 = arctan2(-100, 100);
    let _r3 = arctan2(-100, -100);
    let _r4 = arctan2(100, -100);
    // If we get here, the function works
}

#[test]
fn test_arctan2_zero() {
    let result = arctan2(0, 0);
    assert_eq!(result, 0);
}

#[test]
fn test_swi_constants() {
    assert_eq!(SWI_SOFT_RESET, 0x00);
    assert_eq!(SWI_HALT, 0x02);
    assert_eq!(SWI_VBLANK_INTR_WAIT, 0x05);
    assert_eq!(SWI_DIV, 0x06);
    assert_eq!(SWI_SQRT, 0x08);
    assert_eq!(SWI_ARCTAN, 0x09);
    assert_eq!(SWI_ARCTAN2, 0x0A);
    assert_eq!(SWI_CPU_SET, 0x0B);
    assert_eq!(SWI_LZ77_UNCOMP_WRAM, 0x11);
    assert_eq!(SWI_RL_UNCOMP_WRAM, 0x14);
}

#[test]
fn test_cpuset_flags() {
    assert_eq!(CPUSET_FILL, 1 << 24);
    assert_eq!(CPUSET_32BIT, 1 << 26);
}

#[test]
fn test_soft_reset_no_panic() {
    // Just verify it doesn't panic
    soft_reset();
}

#[test]
fn test_bios_unknown_swi() {
    let mut bios = Bios::new();
    let (should_halt, should_wait) = bios.handle_swi(0xFF);

    assert!(!should_halt);
    assert!(!should_wait);
}
