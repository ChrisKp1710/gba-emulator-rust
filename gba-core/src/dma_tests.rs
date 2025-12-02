use crate::dma::*;

#[test]
fn test_dma_creation() {
    let dma = DMA::new();
    assert!(!dma.is_active());
    assert_eq!(dma.active_channel(), None);
}

#[test]
fn test_dma_control_register() {
    let mut control = DmaControl::default();
    assert!(!control.enabled);
    assert!(!control.irq_enable);
    assert_eq!(control.timing, 0);

    control = DmaControl::from_u16(0xD400); // Enable + IRQ + VBlank + 32bit
    assert!(control.enabled);
    assert!(control.irq_enable);
    assert_eq!(control.timing, 1); // VBlank
    assert!(control.transfer_32bit);
    assert_eq!(control.to_u16(), 0xD400);
}

#[test]
fn test_dma_timing_enum() {
    assert_eq!(DmaTiming::from_u8(0), DmaTiming::Immediate);
    assert_eq!(DmaTiming::from_u8(1), DmaTiming::VBlank);
    assert_eq!(DmaTiming::from_u8(2), DmaTiming::HBlank);
    assert_eq!(DmaTiming::from_u8(3), DmaTiming::Special);
}

#[test]
fn test_dma_register_write_read() {
    let mut dma = DMA::new();
    
    // Write DMA0 registers
    dma.write_register(DMA0SAD, 0x02000000, false); // Source
    dma.write_register(DMA0DAD, 0x06000000, false); // Dest
    dma.write_register(DMA0CNT_L, 0x0100, true);   // Count = 256
    
    assert_eq!(dma.read_register(DMA0SAD), 0x02000000);
    assert_eq!(dma.read_register(DMA0DAD), 0x06000000);
    assert_eq!(dma.read_register(DMA0CNT_L), 0x0100);
}

#[test]
fn test_dma_source_mask() {
    let mut dma = DMA::new();
    
    // DMA0: Can only access internal memory (0x00000000-0x07FFFFFF)
    dma.write_register(DMA0SAD, 0x08001234, false); // Try to set ROM address
    assert_eq!(dma.read_register(DMA0SAD) & 0x08000000, 0); // Should be masked
    
    // DMA3: Can access any memory
    dma.write_register(DMA3SAD, 0x08001234, false);
    assert_eq!(dma.read_register(DMA3SAD), 0x08001234);
}

#[test]
fn test_dma_dest_mask() {
    let mut dma = DMA::new();
    
    // DMA0-2: Can only write to internal memory
    dma.write_register(DMA0DAD, 0x08001234, false);
    assert_eq!(dma.read_register(DMA0DAD) & 0x08000000, 0);
    
    // DMA3: Can write anywhere
    dma.write_register(DMA3DAD, 0x08001234, false);
    assert_eq!(dma.read_register(DMA3DAD), 0x08001234);
}

#[test]
fn test_dma_word_count() {
    let mut dma = DMA::new();
    
    // Write count
    dma.write_register(DMA0CNT_L, 100, true);
    assert_eq!(dma.read_register(DMA0CNT_L), 100);
    
    // Count = 0 should become max (16384 for DMA0-2)
    dma.write_register(DMA0CNT_L, 0, true);
    assert_eq!(dma.read_register(DMA0CNT_L), 0x4000);
}

#[test]
fn test_dma_immediate_trigger() {
    let mut dma = DMA::new();
    let mut transfer_count = 0;
    
    // Setup DMA0 for immediate transfer
    dma.write_register(DMA0SAD, 0x02000000, false);
    dma.write_register(DMA0DAD, 0x06000000, false);
    dma.write_register(DMA0CNT_L, 10, true); // 10 words
    dma.write_register(DMA0CNT_H, 0x8000, true); // Enable, immediate, 16-bit
    
    // Should be active immediately
    assert!(dma.is_active());
    
    // Execute transfers
    dma.step(|_src, _dst, _is_32| {
        transfer_count += 1;
    });
    
    assert_eq!(transfer_count, 10);
    assert!(!dma.is_active()); // Should be done
}

#[test]
fn test_dma_vblank_trigger() {
    let mut dma = DMA::new();
    
    // Setup DMA1 for VBlank transfer
    dma.write_register(DMA1SAD, 0x02000000, false);
    dma.write_register(DMA1DAD, 0x06000000, false);
    dma.write_register(DMA1CNT_L, 5, true);
    dma.write_register(DMA1CNT_H, 0x9000, true); // Enable, VBlank timing
    
    // Should NOT be active yet
    assert!(!dma.is_active());
    
    // Trigger VBlank
    dma.trigger(DmaTiming::VBlank);
    
    // Now should be active
    assert!(dma.is_active());
}

#[test]
fn test_dma_hblank_trigger() {
    let mut dma = DMA::new();
    
    // Setup DMA2 for HBlank transfer
    dma.write_register(DMA2SAD, 0x02000000, false);
    dma.write_register(DMA2DAD, 0x06000000, false);
    dma.write_register(DMA2CNT_L, 3, true);
    dma.write_register(DMA2CNT_H, 0xA000, true); // Enable, HBlank timing
    
    assert!(!dma.is_active());
    
    dma.trigger(DmaTiming::HBlank);
    assert!(dma.is_active());
}

#[test]
fn test_dma_32bit_transfer() {
    let mut dma = DMA::new();
    let mut is_32bit_called = false;
    
    // Setup for 32-bit transfer
    dma.write_register(DMA0SAD, 0x02000000, false);
    dma.write_register(DMA0DAD, 0x06000000, false);
    dma.write_register(DMA0CNT_L, 1, true);
    dma.write_register(DMA0CNT_H, 0x8400, true); // Enable, 32-bit
    
    dma.step(|_src, _dst, is_32| {
        is_32bit_called = is_32;
    });
    
    assert!(is_32bit_called);
}

#[test]
fn test_dma_address_increment() {
    let mut dma = DMA::new();
    let mut addresses = Vec::new();
    
    // Source increment, dest increment (default)
    dma.write_register(DMA0SAD, 0x02000000, false);
    dma.write_register(DMA0DAD, 0x06000000, false);
    dma.write_register(DMA0CNT_L, 3, true);
    dma.write_register(DMA0CNT_H, 0x8000, true); // 16-bit, increment both
    
    dma.step(|src, dst, _| {
        addresses.push((src, dst));
    });
    
    // Check addresses increment by 2 (16-bit)
    assert_eq!(addresses.len(), 3);
    assert_eq!(addresses[0], (0x02000000, 0x06000000));
    assert_eq!(addresses[1], (0x02000002, 0x06000002));
    assert_eq!(addresses[2], (0x02000004, 0x06000004));
}

#[test]
fn test_dma_address_decrement() {
    let mut dma = DMA::new();
    let mut addresses = Vec::new();
    
    // Source decrement, dest decrement
    dma.write_register(DMA0SAD, 0x02000010, false);
    dma.write_register(DMA0DAD, 0x06000010, false);
    dma.write_register(DMA0CNT_L, 3, true);
    dma.write_register(DMA0CNT_H, 0x80A0, true); // Decrement both (dest=01, src=01)
    
    dma.step(|src, dst, _| {
        addresses.push((src, dst));
    });
    
    // Check addresses decrement by 2
    assert_eq!(addresses[0], (0x02000010, 0x06000010));
    assert_eq!(addresses[1], (0x0200000E, 0x0600000E));
    assert_eq!(addresses[2], (0x0200000C, 0x0600000C));
}

#[test]
fn test_dma_address_fixed() {
    let mut dma = DMA::new();
    let mut addresses = Vec::new();
    
    // Source fixed, dest fixed
    dma.write_register(DMA0SAD, 0x02000000, false);
    dma.write_register(DMA0DAD, 0x06000000, false);
    dma.write_register(DMA0CNT_L, 3, true);
    dma.write_register(DMA0CNT_H, 0x8140, true); // Fixed both (bits 5-6, 7-8 = 10)
    
    dma.step(|src, dst, _| {
        addresses.push((src, dst));
    });
    
    // Addresses should stay the same
    assert_eq!(addresses[0], (0x02000000, 0x06000000));
    assert_eq!(addresses[1], (0x02000000, 0x06000000));
    assert_eq!(addresses[2], (0x02000000, 0x06000000));
}

#[test]
fn test_dma_irq_flag() {
    let mut dma = DMA::new();
    
    // Setup with IRQ enabled
    dma.write_register(DMA0SAD, 0x02000000, false);
    dma.write_register(DMA0DAD, 0x06000000, false);
    dma.write_register(DMA0CNT_L, 2, true);
    dma.write_register(DMA0CNT_H, 0xC000, true); // Enable + IRQ
    
    let irq_flags = dma.step(|_, _, _| {});
    
    // Should have IRQ flag for channel 0
    assert_eq!(irq_flags & 1, 1);
}

#[test]
fn test_dma_no_irq_when_disabled() {
    let mut dma = DMA::new();
    
    // Setup without IRQ
    dma.write_register(DMA0SAD, 0x02000000, false);
    dma.write_register(DMA0DAD, 0x06000000, false);
    dma.write_register(DMA0CNT_L, 2, true);
    dma.write_register(DMA0CNT_H, 0x8000, true); // Enable, no IRQ
    
    let irq_flags = dma.step(|_, _, _| {});
    
    // Should have NO IRQ
    assert_eq!(irq_flags, 0);
}

#[test]
fn test_dma_repeat_mode() {
    let mut dma = DMA::new();
    
    // Setup with repeat enabled
    dma.write_register(DMA1SAD, 0x02000000, false);
    dma.write_register(DMA1DAD, 0x06000000, false);
    dma.write_register(DMA1CNT_L, 2, true);
    dma.write_register(DMA1CNT_H, 0x9200, true); // Enable, VBlank, Repeat
    
    // First VBlank trigger
    dma.trigger(DmaTiming::VBlank);
    dma.step(|_, _, _| {});
    
    // Should still be enabled for repeat
    assert_eq!(dma.read_register(DMA1CNT_H) & 0x8000, 0x8000);
    
    // Second VBlank trigger should work
    dma.trigger(DmaTiming::VBlank);
    assert!(dma.is_active());
}

#[test]
fn test_dma_priority() {
    let mut dma = DMA::new();
    
    // Enable DMA0 only (highest priority)
    dma.write_register(DMA0SAD, 0x02000000, false);
    dma.write_register(DMA0DAD, 0x06000000, false);
    dma.write_register(DMA0CNT_L, 1, true);
    dma.write_register(DMA0CNT_H, 0x8000, true);
    
    // DMA0 should be active
    assert_eq!(dma.active_channel(), Some(0));
    
    // Execute
    dma.step(|_, _, _| {});
    
    // Should complete
    assert!(!dma.is_active());
}

#[test]
fn test_dma_reset() {
    let mut dma = DMA::new();
    
    // Setup and activate a channel
    dma.write_register(DMA0SAD, 0x02000000, false);
    dma.write_register(DMA0DAD, 0x06000000, false);
    dma.write_register(DMA0CNT_L, 100, true);
    dma.write_register(DMA0CNT_H, 0x8000, true);
    
    assert!(dma.is_active());
    
    // Reset
    dma.reset();
    
    // Should be inactive and registers cleared
    assert!(!dma.is_active());
    assert_eq!(dma.read_register(DMA0SAD), 0);
    assert_eq!(dma.read_register(DMA0CNT_H) & 0x8000, 0);
}
