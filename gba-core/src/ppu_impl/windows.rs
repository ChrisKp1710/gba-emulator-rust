/// PPU Windows - Window clipping system
///
/// GBA has 4 windows for controlling which pixels are visible:
/// - WIN0: Rectangular window 0
/// - WIN1: Rectangular window 1
/// - WINOBJ: Object window (from sprites)
/// - WINOUT: Outside windows area
///
/// Registers:
/// - WIN0H/WIN1H: Horizontal coordinates (right, left)
/// - WIN0V/WIN1V: Vertical coordinates (bottom, top)
/// - WININ: Control for inside WIN0/WIN1
/// - WINOUT: Control for outside windows and OBJ window

/// Window control flags (WININ/WINOUT)
#[derive(Debug, Clone, Copy, Default)]
pub struct WindowControl {
    pub bg0_enable: bool,
    pub bg1_enable: bool,
    pub bg2_enable: bool,
    pub bg3_enable: bool,
    pub obj_enable: bool,
    pub blend_enable: bool,
}

impl WindowControl {
    pub fn from_u8(value: u8) -> Self {
        Self {
            bg0_enable: (value & 0x01) != 0,
            bg1_enable: (value & 0x02) != 0,
            bg2_enable: (value & 0x04) != 0,
            bg3_enable: (value & 0x08) != 0,
            obj_enable: (value & 0x10) != 0,
            blend_enable: (value & 0x20) != 0,
        }
    }

    pub fn to_u8(&self) -> u8 {
        (self.bg0_enable as u8)
            | ((self.bg1_enable as u8) << 1)
            | ((self.bg2_enable as u8) << 2)
            | ((self.bg3_enable as u8) << 3)
            | ((self.obj_enable as u8) << 4)
            | ((self.blend_enable as u8) << 5)
    }
}

/// Window boundaries
#[derive(Debug, Clone, Copy)]
pub struct WindowBounds {
    pub left: u8,
    pub right: u8,
    pub top: u8,
    pub bottom: u8,
}

impl WindowBounds {
    pub fn new() -> Self {
        Self {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        }
    }

    /// Check if a pixel (x, y) is inside this window
    pub fn contains(&self, x: u8, y: u8) -> bool {
        let x_in = if self.right < self.left {
            // Wrapped: x >= left OR x < right
            x >= self.left || x < self.right
        } else {
            // Normal: left <= x < right
            x >= self.left && x < self.right
        };

        let y_in = if self.bottom < self.top {
            // Wrapped: y >= top OR y < bottom
            y >= self.top || y < self.bottom
        } else {
            // Normal: top <= y < bottom
            y >= self.top && y < self.bottom
        };

        x_in && y_in
    }

    pub fn from_horizontal(value: u16) -> (u8, u8) {
        let right = (value & 0xFF) as u8;
        let left = ((value >> 8) & 0xFF) as u8;
        (left, right)
    }

    pub fn from_vertical(value: u16) -> (u8, u8) {
        let bottom = (value & 0xFF) as u8;
        let top = ((value >> 8) & 0xFF) as u8;
        (top, bottom)
    }
}

/// Window system state
pub struct Windows {
    pub win0: WindowBounds,
    pub win1: WindowBounds,
    pub win0_control: WindowControl,
    pub win1_control: WindowControl,
    pub winout_control: WindowControl,
    pub winobj_control: WindowControl,
    pub win0_enabled: bool,
    pub win1_enabled: bool,
    pub winobj_enabled: bool,
}

impl Windows {
    pub fn new() -> Self {
        Self {
            win0: WindowBounds::new(),
            win1: WindowBounds::new(),
            win0_control: WindowControl::default(),
            win1_control: WindowControl::default(),
            winout_control: WindowControl::default(),
            winobj_control: WindowControl::default(),
            win0_enabled: false,
            win1_enabled: false,
            winobj_enabled: false,
        }
    }

    /// Get the window control for a pixel at (x, y)
    /// Priority: WIN0 > WIN1 > WINOBJ > WINOUT
    pub fn get_control(&self, x: u8, y: u8, _in_obj_window: bool) -> WindowControl {
        // WIN0 has highest priority
        if self.win0_enabled && self.win0.contains(x, y) {
            return self.win0_control;
        }

        // WIN1 second priority
        if self.win1_enabled && self.win1.contains(x, y) {
            return self.win1_control;
        }

        // WINOBJ third priority (TODO: check if pixel is in OBJ window)
        // if self.winobj_enabled && in_obj_window {
        //     return self.winobj_control;
        // }

        // Default: WINOUT (outside all windows)
        self.winout_control
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_control_parsing() {
        let ctrl = WindowControl::from_u8(0b00101011);
        assert!(ctrl.bg0_enable);
        assert!(ctrl.bg1_enable);
        assert!(!ctrl.bg2_enable);
        assert!(ctrl.bg3_enable);
        assert!(!ctrl.obj_enable);
        assert!(ctrl.blend_enable);

        assert_eq!(ctrl.to_u8(), 0b00101011);
    }

    #[test]
    fn test_window_bounds_normal() {
        let mut bounds = WindowBounds::new();
        bounds.left = 30;
        bounds.right = 100;
        bounds.top = 20;
        bounds.bottom = 80;

        assert!(bounds.contains(50, 50)); // Inside
        assert!(bounds.contains(30, 20)); // Top-left corner
        assert!(!bounds.contains(100, 50)); // Right edge (exclusive)
        assert!(!bounds.contains(50, 80)); // Bottom edge (exclusive)
        assert!(!bounds.contains(10, 50)); // Left of window
        assert!(!bounds.contains(150, 50)); // Right of window
    }

    #[test]
    fn test_window_bounds_wrapped_horizontal() {
        let mut bounds = WindowBounds::new();
        bounds.left = 200;
        bounds.right = 50; // Wrapped
        bounds.top = 20;
        bounds.bottom = 80;

        assert!(bounds.contains(220, 50)); // Right side of wrap
        assert!(bounds.contains(10, 50)); // Left side of wrap
        assert!(!bounds.contains(100, 50)); // Middle (excluded)
    }

    #[test]
    fn test_window_bounds_wrapped_vertical() {
        let mut bounds = WindowBounds::new();
        bounds.left = 30;
        bounds.right = 100;
        bounds.top = 140;
        bounds.bottom = 20; // Wrapped

        assert!(bounds.contains(50, 150)); // Bottom side of wrap
        assert!(bounds.contains(50, 10)); // Top side of wrap
        assert!(!bounds.contains(50, 80)); // Middle (excluded)
    }

    #[test]
    fn test_window_priority() {
        let mut windows = Windows::new();

        // Setup overlapping windows
        windows.win0_enabled = true;
        windows.win0.left = 0;
        windows.win0.right = 100;
        windows.win0.top = 0;
        windows.win0.bottom = 100;
        windows.win0_control.bg0_enable = true;

        windows.win1_enabled = true;
        windows.win1.left = 50;
        windows.win1.right = 150;
        windows.win1.top = 50;
        windows.win1.bottom = 150;
        windows.win1_control.bg1_enable = true;

        windows.winout_control.bg2_enable = true;

        // Inside WIN0 only
        let ctrl = windows.get_control(25, 25, false);
        assert!(ctrl.bg0_enable);
        assert!(!ctrl.bg1_enable);

        // Inside WIN0 and WIN1 overlap - WIN0 wins
        let ctrl = windows.get_control(75, 75, false);
        assert!(ctrl.bg0_enable);
        assert!(!ctrl.bg1_enable);

        // Inside WIN1 only
        let ctrl = windows.get_control(125, 125, false);
        assert!(!ctrl.bg0_enable);
        assert!(ctrl.bg1_enable);

        // Outside all windows - WINOUT
        let ctrl = windows.get_control(200, 200, false);
        assert!(!ctrl.bg0_enable);
        assert!(!ctrl.bg1_enable);
        assert!(ctrl.bg2_enable);
    }

    #[test]
    fn test_horizontal_vertical_parsing() {
        // WIN0H = 0x5020 means right=0x20, left=0x50
        let (left, right) = WindowBounds::from_horizontal(0x5020);
        assert_eq!(left, 0x50);
        assert_eq!(right, 0x20);

        // WIN0V = 0xA040 means bottom=0x40, top=0xA0
        let (top, bottom) = WindowBounds::from_vertical(0xA040);
        assert_eq!(top, 0xA0);
        assert_eq!(bottom, 0x40);
    }
}
