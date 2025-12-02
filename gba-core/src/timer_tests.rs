use crate::timer::*;

#[test]
fn test_timer_creation() {
    let timer = Timer::new();
    assert_eq!(timer.read_register(TM0CNT_L), 0);
    assert_eq!(timer.read_register(TM0CNT_H), 0);
}

#[test]
fn test_timer_control_register() {
    let mut control = TimerControl::default();
    assert!(!control.enabled);
    assert!(!control.irq_enable);
    assert_eq!(control.prescaler, 0);

    control = TimerControl::from_u16(0x00C7); // Enable=1, IRQ=1, CountUp=1, Prescaler=3
    assert_eq!(control.prescaler, 3);
    assert!(control.count_up);
    assert!(control.irq_enable);
    assert!(control.enabled);
    assert_eq!(control.to_u16(), 0x00C7);
}

#[test]
fn test_timer_reload() {
    let mut timer = Timer::new();

    // Write reload value
    timer.write_register(TM0CNT_L, 0xF000);
    assert_eq!(timer.read_register(TM0CNT_L), 0xF000);
}

#[test]
fn test_timer_counting() {
    let mut timer = Timer::new();

    // Set reload to 0xFFF0, enable with prescaler 1
    timer.write_register(TM0CNT_L, 0xFFF0);
    timer.write_register(TM0CNT_H, 0x0080); // Enable, prescaler 1

    // Step 10 cycles
    timer.step(10);
    assert_eq!(timer.read_register(TM0CNT_L), 0xFFFA);
}

#[test]
fn test_timer_overflow() {
    let mut timer = Timer::new();

    // Set reload to 0xFFFE, enable with prescaler 1
    timer.write_register(TM0CNT_L, 0xFFFE);
    timer.write_register(TM0CNT_H, 0x0080); // Enable, prescaler 1

    // Step past overflow
    let irq = timer.step(5);

    // Should overflow and reload
    assert_eq!(timer.read_register(TM0CNT_L), 0xFFFF); // Reloaded + 1
    assert_eq!(irq, 0); // IRQ not enabled
}

#[test]
fn test_timer_overflow_irq() {
    let mut timer = Timer::new();

    // Set reload to 0xFFFF, enable with IRQ and prescaler 1
    timer.write_register(TM0CNT_L, 0xFFFF);
    timer.write_register(TM0CNT_H, 0x00C0); // Enable + IRQ, prescaler 1

    // Step 1 cycle to overflow
    let irq = timer.step(1);

    // Should set bit 3 (Timer 0 IRQ)
    assert_eq!(irq & (1 << 3), 1 << 3);
}

#[test]
fn test_prescaler_64() {
    let mut timer = Timer::new();

    // Enable with prescaler 64
    timer.write_register(TM0CNT_L, 0);
    timer.write_register(TM0CNT_H, 0x0081); // Enable, prescaler 64

    // Step 63 cycles (should not increment)
    timer.step(63);
    assert_eq!(timer.read_register(TM0CNT_L), 0);

    // Step 1 more cycle (64 total, should increment)
    timer.step(1);
    assert_eq!(timer.read_register(TM0CNT_L), 1);
}

#[test]
fn test_prescaler_256() {
    let mut timer = Timer::new();

    // Enable with prescaler 256
    timer.write_register(TM0CNT_L, 0);
    timer.write_register(TM0CNT_H, 0x0082); // Enable, prescaler 256

    // Step 255 cycles (should not increment)
    timer.step(255);
    assert_eq!(timer.read_register(TM0CNT_L), 0);

    // Step 1 more cycle (256 total)
    timer.step(1);
    assert_eq!(timer.read_register(TM0CNT_L), 1);
}

#[test]
fn test_prescaler_1024() {
    let mut timer = Timer::new();

    // Enable with prescaler 1024
    timer.write_register(TM0CNT_L, 0);
    timer.write_register(TM0CNT_H, 0x0083); // Enable, prescaler 1024

    // Step 1023 cycles (should not increment)
    timer.step(1023);
    assert_eq!(timer.read_register(TM0CNT_L), 0);

    // Step 1 more cycle (1024 total)
    timer.step(1);
    assert_eq!(timer.read_register(TM0CNT_L), 1);
}

#[test]
fn test_cascade_mode() {
    let mut timer = Timer::new();

    // Timer 0: prescaler 1, reload 0xFFFF
    timer.write_register(TM0CNT_L, 0xFFFF);
    timer.write_register(TM0CNT_H, 0x0080); // Enable, prescaler 1

    // Timer 1: cascade mode (count-up)
    timer.write_register(TM1CNT_L, 0);
    timer.write_register(TM1CNT_H, 0x0084); // Enable, count-up mode

    // Step 1 cycle (timer 0 overflows)
    timer.step(1);

    // Timer 0 should reload, Timer 1 should increment
    assert_eq!(timer.read_register(TM0CNT_L), 0xFFFF);
    assert_eq!(timer.read_register(TM1CNT_L), 1);
}

#[test]
fn test_all_timers() {
    let mut timer = Timer::new();

    // Enable all 4 timers with prescaler 1
    for i in 0..4 {
        let cnt_l = TM0CNT_L + (i * 4);
        let cnt_h = TM0CNT_H + (i * 4);
        timer.write_register(cnt_l, 0);
        timer.write_register(cnt_h, 0x0080);
    }

    // Step 100 cycles
    timer.step(100);

    // All should be at 100
    for i in 0..4 {
        let cnt_l = TM0CNT_L + (i * 4);
        assert_eq!(timer.read_register(cnt_l), 100);
    }
}

#[test]
fn test_timer_disabled_no_count() {
    let mut timer = Timer::new();

    // Write reload but don't enable
    timer.write_register(TM0CNT_L, 0);
    timer.write_register(TM0CNT_H, 0x0000); // Disabled

    // Step cycles
    timer.step(1000);

    // Should remain at 0
    assert_eq!(timer.read_register(TM0CNT_L), 0);
}

#[test]
fn test_timer_enable_reloads() {
    let mut timer = Timer::new();

    // Set reload while disabled
    timer.write_register(TM0CNT_L, 0x1234);
    timer.write_register(TM0CNT_H, 0x0000);

    // Counter should be at reload value
    assert_eq!(timer.read_register(TM0CNT_L), 0x1234);

    // Enable timer
    timer.write_register(TM0CNT_H, 0x0080);

    // Counter should still be at reload
    assert_eq!(timer.read_register(TM0CNT_L), 0x1234);

    // Step 1 cycle
    timer.step(1);

    // Now should increment
    assert_eq!(timer.read_register(TM0CNT_L), 0x1235);
}
