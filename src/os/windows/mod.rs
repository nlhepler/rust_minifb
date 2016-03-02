#![cfg(target_os = "windows")]

extern crate user32;
extern crate kernel32;
extern crate winapi;
extern crate gdi32;
extern crate time;

use {Scale, Key, KeyRepeat, MouseButton, MouseMode, WindowOptions};

use key_handler::KeyHandler;
use menu::Menu;

use std::ptr;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use std::mem;
use std::os::raw;
use mouse_handler;

use self::winapi::windef::HWND;
use self::winapi::windef::HDC;
use self::winapi::windef::HMENU;
use self::winapi::wingdi::BITMAPINFOHEADER;
use self::winapi::wingdi::RGBQUAD;
use self::winapi::winuser::WNDCLASSW;
use self::winapi::winuser::ACCEL;
use self::winapi::basetsd::UINT_PTR;

// Wrap this so we can have a proper numbef of bmiColors to write in
#[repr(C)]
struct BitmapInfo {
    pub bmi_header: BITMAPINFOHEADER,
    pub bmi_colors: [RGBQUAD; 3],
}

fn update_key_state(window: &mut Window, wparam: u32, state: bool) {
    match wparam & 0x1ff {
        0x00B => window.key_handler.set_key_state(Key::Key0, state),
        0x002 => window.key_handler.set_key_state(Key::Key1, state),
        0x003 => window.key_handler.set_key_state(Key::Key2, state),
        0x004 => window.key_handler.set_key_state(Key::Key3, state),
        0x005 => window.key_handler.set_key_state(Key::Key4, state),
        0x006 => window.key_handler.set_key_state(Key::Key5, state),
        0x007 => window.key_handler.set_key_state(Key::Key6, state),
        0x008 => window.key_handler.set_key_state(Key::Key7, state),
        0x009 => window.key_handler.set_key_state(Key::Key8, state),
        0x00A => window.key_handler.set_key_state(Key::Key9, state),
        0x01E => window.key_handler.set_key_state(Key::A, state),
        0x030 => window.key_handler.set_key_state(Key::B, state),
        0x02E => window.key_handler.set_key_state(Key::C, state),
        0x020 => window.key_handler.set_key_state(Key::D, state),
        0x012 => window.key_handler.set_key_state(Key::E, state),
        0x021 => window.key_handler.set_key_state(Key::F, state),
        0x022 => window.key_handler.set_key_state(Key::G, state),
        0x023 => window.key_handler.set_key_state(Key::H, state),
        0x017 => window.key_handler.set_key_state(Key::I, state),
        0x024 => window.key_handler.set_key_state(Key::J, state),
        0x025 => window.key_handler.set_key_state(Key::K, state),
        0x026 => window.key_handler.set_key_state(Key::L, state),
        0x032 => window.key_handler.set_key_state(Key::M, state),
        0x031 => window.key_handler.set_key_state(Key::N, state),
        0x018 => window.key_handler.set_key_state(Key::O, state),
        0x019 => window.key_handler.set_key_state(Key::P, state),
        0x010 => window.key_handler.set_key_state(Key::Q, state),
        0x013 => window.key_handler.set_key_state(Key::R, state),
        0x01F => window.key_handler.set_key_state(Key::S, state),
        0x014 => window.key_handler.set_key_state(Key::T, state),
        0x016 => window.key_handler.set_key_state(Key::U, state),
        0x02F => window.key_handler.set_key_state(Key::V, state),
        0x011 => window.key_handler.set_key_state(Key::W, state),
        0x02D => window.key_handler.set_key_state(Key::X, state),
        0x015 => window.key_handler.set_key_state(Key::Y, state),
        0x02C => window.key_handler.set_key_state(Key::Z, state),
        0x03B => window.key_handler.set_key_state(Key::F1, state),
        0x03C => window.key_handler.set_key_state(Key::F2, state),
        0x03D => window.key_handler.set_key_state(Key::F3, state),
        0x03E => window.key_handler.set_key_state(Key::F4, state),
        0x03F => window.key_handler.set_key_state(Key::F5, state),
        0x040 => window.key_handler.set_key_state(Key::F6, state),
        0x041 => window.key_handler.set_key_state(Key::F7, state),
        0x042 => window.key_handler.set_key_state(Key::F8, state),
        0x043 => window.key_handler.set_key_state(Key::F9, state),
        0x044 => window.key_handler.set_key_state(Key::F10, state),
        0x057 => window.key_handler.set_key_state(Key::F11, state),
        0x058 => window.key_handler.set_key_state(Key::F12, state),
        0x150 => window.key_handler.set_key_state(Key::Down, state),
        0x14B => window.key_handler.set_key_state(Key::Left, state),
        0x14D => window.key_handler.set_key_state(Key::Right, state),
        0x148 => window.key_handler.set_key_state(Key::Up, state),
        0x028 => window.key_handler.set_key_state(Key::Apostrophe, state),
        0x029 => window.key_handler.set_key_state(Key::Backquote, state),
        0x02B => window.key_handler.set_key_state(Key::Backslash, state),
        0x033 => window.key_handler.set_key_state(Key::Comma, state),
        0x00D => window.key_handler.set_key_state(Key::Equal, state),
        0x01A => window.key_handler.set_key_state(Key::LeftBracket, state),
        0x00C => window.key_handler.set_key_state(Key::Minus, state),
        0x034 => window.key_handler.set_key_state(Key::Period, state),
        0x01B => window.key_handler.set_key_state(Key::RightBracket, state),
        0x027 => window.key_handler.set_key_state(Key::Semicolon, state),
        0x035 => window.key_handler.set_key_state(Key::Slash, state),
        0x00E => window.key_handler.set_key_state(Key::Backspace, state),
        0x153 => window.key_handler.set_key_state(Key::Delete, state),
        0x14F => window.key_handler.set_key_state(Key::End, state),
        0x01C => window.key_handler.set_key_state(Key::Enter, state),
        0x001 => window.key_handler.set_key_state(Key::Escape, state),
        0x147 => window.key_handler.set_key_state(Key::Home, state),
        0x152 => window.key_handler.set_key_state(Key::Insert, state),
        0x15D => window.key_handler.set_key_state(Key::Menu, state),
        0x151 => window.key_handler.set_key_state(Key::PageDown, state),
        0x149 => window.key_handler.set_key_state(Key::PageUp, state),
        0x045 => window.key_handler.set_key_state(Key::Pause, state),
        0x039 => window.key_handler.set_key_state(Key::Space, state),
        0x00F => window.key_handler.set_key_state(Key::Tab, state),
        0x145 => window.key_handler.set_key_state(Key::NumLock, state),
        0x03A => window.key_handler.set_key_state(Key::CapsLock, state),
        0x046 => window.key_handler.set_key_state(Key::ScrollLock, state),
        0x02A => window.key_handler.set_key_state(Key::LeftShift, state),
        0x036 => window.key_handler.set_key_state(Key::RightShift, state),
        0x01D => window.key_handler.set_key_state(Key::LeftCtrl, state),
        0x11D => window.key_handler.set_key_state(Key::RightCtrl, state),
        0x052 => window.key_handler.set_key_state(Key::NumPad0, state),
        0x04F => window.key_handler.set_key_state(Key::NumPad1, state),
        0x050 => window.key_handler.set_key_state(Key::NumPad2, state),
        0x051 => window.key_handler.set_key_state(Key::NumPad3, state),
        0x04B => window.key_handler.set_key_state(Key::NumPad4, state),
        0x04C => window.key_handler.set_key_state(Key::NumPad5, state),
        0x04D => window.key_handler.set_key_state(Key::NumPad6, state),
        0x047 => window.key_handler.set_key_state(Key::NumPad7, state),
        0x048 => window.key_handler.set_key_state(Key::NumPad8, state),
        0x049 => window.key_handler.set_key_state(Key::NumPad9, state),
        0x053 => window.key_handler.set_key_state(Key::NumPadDot, state),
        0x135 => window.key_handler.set_key_state(Key::NumPadSlash, state),
        0x037 => window.key_handler.set_key_state(Key::NumPadAsterisk, state),
        0x04A => window.key_handler.set_key_state(Key::NumPadMinus, state),
        0x04E => window.key_handler.set_key_state(Key::NumPadPlus, state),
        0x11C => window.key_handler.set_key_state(Key::NumPadEnter, state),
        _ => (),
    }
}


#[cfg(target_arch = "x86_64")]
unsafe fn set_window_long(window: winapi::HWND, data: winapi::LONG_PTR) -> winapi::LONG_PTR {
    user32::SetWindowLongPtrW(window, winapi::winuser::GWLP_USERDATA, data)
}

#[cfg(target_arch = "x86_64")]
unsafe fn get_window_long(window: winapi::HWND) -> winapi::LONG_PTR {
    user32::GetWindowLongPtrW(window, winapi::winuser::GWLP_USERDATA)
}

#[cfg(target_arch = "x86")]
unsafe fn set_window_long(window: winapi::HWND, data: winapi::LONG) -> winapi::LONG {
    user32::SetWindowLongW(window, winapi::winuser::GWLP_USERDATA, data)
}

#[cfg(target_arch = "x86")]
unsafe fn get_window_long(window: winapi::HWND) -> winapi::LONG {
    user32::GetWindowLongW(window, winapi::winuser::GWLP_USERDATA)
}

unsafe extern "system" fn wnd_proc(window: winapi::HWND,
                                   msg: winapi::UINT,
                                   wparam: winapi::WPARAM,
                                   lparam: winapi::LPARAM)
                                   -> winapi::LRESULT {
    // This make sure we actually don't do anything before the user data has been setup for the
    // window

    let user_data = get_window_long(window);

    if user_data == 0 {
        return user32::DefWindowProcW(window, msg, wparam, lparam);
    }

    let mut wnd: &mut Window = mem::transmute(user_data);

    match msg {
        /*
        winapi::winuser::WM_MOUSEMOVE => {
            let mouse_coords = lparam as u32;
            let scale = user_data.scale as f32;
            user_data.mouse.local_x = (((mouse_coords >> 16) & 0xffff) as f32) / scale;
            user_data.mouse.local_y = ((mouse_coords & 0xffff) as f32) / scale;

            return 0;
        }
        */
        winapi::winuser::WM_MOUSEWHEEL => {
            let scroll = ((((wparam as u32) >> 16) & 0xffff) as i16) as f32 * 0.1;
            wnd.mouse.scroll = scroll;
        }

        winapi::winuser::WM_KEYDOWN => {
            update_key_state(wnd, (lparam as u32) >> 16, true);
            return 0;
        }

        winapi::winuser::WM_LBUTTONDOWN => {
            wnd.mouse.state[0] = true
        }

        winapi::winuser::WM_LBUTTONUP => {
            wnd.mouse.state[0] = false
        }

        winapi::winuser::WM_MBUTTONDOWN => {
            wnd.mouse.state[1] = true
        }

        winapi::winuser::WM_MBUTTONUP => {
            wnd.mouse.state[1] = false
        }

        winapi::winuser::WM_RBUTTONDOWN => {
            wnd.mouse.state[2] = true
        }

        winapi::winuser::WM_RBUTTONUP => {
            wnd.mouse.state[2] = false
        }

        winapi::winuser::WM_CLOSE => {
            wnd.is_open = false;
        }

        winapi::winuser::WM_KEYUP => {
            update_key_state(wnd, (lparam as u32) >> 16, false);
            return 0;
        }

        winapi::winuser::WM_PAINT => {

            // if we have nothing to draw here we return the default function
            if wnd.buffer.len() == 0 {
                return user32::DefWindowProcW(window, msg, wparam, lparam);
            }

            let mut bitmap_info: BitmapInfo = mem::zeroed();

            bitmap_info.bmi_header.biSize = mem::size_of::<BITMAPINFOHEADER>() as u32;
            bitmap_info.bmi_header.biPlanes = 1;
            bitmap_info.bmi_header.biBitCount = 32;
            bitmap_info.bmi_header.biCompression = winapi::wingdi::BI_BITFIELDS;
            bitmap_info.bmi_header.biWidth = wnd.width;
            bitmap_info.bmi_header.biHeight = -wnd.height;
            bitmap_info.bmi_colors[0].rgbRed = 0xff;
            bitmap_info.bmi_colors[1].rgbGreen = 0xff;
            bitmap_info.bmi_colors[2].rgbBlue = 0xff;

            gdi32::StretchDIBits(wnd.dc.unwrap(),
                                 0,
                                 0,
                                 wnd.width * wnd.scale_factor,
                                 wnd.height * wnd.scale_factor,
                                 0,
                                 0,
                                 wnd.width,
                                 wnd.height,
                                 mem::transmute(wnd.buffer.as_ptr()),
                                 mem::transmute(&bitmap_info),
                                 winapi::wingdi::DIB_RGB_COLORS,
                                 winapi::wingdi::SRCCOPY);

            user32::ValidateRect(window, ptr::null_mut());

            return 0;
        }

        _ => (),
    }

    return user32::DefWindowProcW(window, msg, wparam, lparam);
}

pub enum MinifbError {
    UnableToCreateWindow,
}

fn to_wstring(str: &str) -> Vec<u16> {
    let mut v: Vec<u16> = OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
    v.push(0u16);
    v
}

#[derive(Default)]
struct MouseData {
    pub x: f32,
    pub y: f32,
    pub state: [bool; 8],
    pub scroll: f32,
}

pub struct Window {
    mouse: MouseData,
    dc: Option<HDC>,
    window: Option<HWND>,
    buffer: Vec<u32>,
    is_open : bool,
    scale_factor: i32,
    width: i32,
    height: i32,
    key_handler: KeyHandler,
    accel_table: Option<Vec<ACCEL>>,
}

impl Window {
    fn open_window(name: &str, width: usize, height: usize, opts: WindowOptions, scale_factor: i32) -> Option<HWND> {
        unsafe {
            let class_name = to_wstring("minifb_window");
            let class = WNDCLASSW {
                style: winapi::CS_HREDRAW | winapi::CS_VREDRAW | winapi::CS_OWNDC,
                lpfnWndProc: Some(wnd_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: kernel32::GetModuleHandleA(ptr::null()),
                hIcon: ptr::null_mut(),
                hCursor: ptr::null_mut(),
                hbrBackground: ptr::null_mut(),
                lpszMenuName: ptr::null(),
                lpszClassName: class_name.as_ptr(),
            };

            if user32::RegisterClassW(&class) == 0 {
                // ignore the "Class already exists" error for multiple windows
                if kernel32::GetLastError() as u32 != 1410 {
                    println!("Unable to register class, error {}", kernel32::GetLastError() as u32);
                    return None;
                }
            }

            let new_width = width * scale_factor as usize;
            let new_height = height * scale_factor as usize;

            let mut rect = winapi::RECT {
                left: 0,
                right: new_width as winapi::LONG,
                top: 0,
                bottom: new_height as winapi::LONG,
            };

            user32::AdjustWindowRect(&mut rect,
                                     winapi::WS_POPUP | winapi::WS_SYSMENU | winapi::WS_CAPTION,
                                     0);

            rect.right -= rect.left;
            rect.bottom -= rect.top;

            let window_name = to_wstring(name);

            let mut flags = 0;

            if opts.title {
                flags |= winapi::WS_OVERLAPPEDWINDOW as u32;
            }

            if opts.resize {
                flags |= winapi::WS_THICKFRAME as u32 | winapi::WS_MAXIMIZEBOX as u32 ;

            } else {
                flags &= !winapi::WS_MAXIMIZEBOX;
                flags &= !winapi::WS_THICKFRAME;
            }

            if opts.borderless {
                flags &= !winapi::WS_THICKFRAME;
            }

            let handle = user32::CreateWindowExW(0,
                                                 class_name.as_ptr(),
                                                 window_name.as_ptr(),
                                                 flags,
                                                 winapi::CW_USEDEFAULT,
                                                 winapi::CW_USEDEFAULT,
                                                 rect.right,
                                                 rect.bottom,
                                                 ptr::null_mut(),
                                                 ptr::null_mut(),
                                                 ptr::null_mut(),
                                                 ptr::null_mut());
            if handle.is_null() {
                println!("Unable to create window, error {}", kernel32::GetLastError() as u32);
                return None;
            }

            user32::ShowWindow(handle, winapi::SW_NORMAL);

            return Some(handle);
        }
    }

    pub fn new(name: &str,
               width: usize,
               height: usize,
               opts: WindowOptions)
               -> Result<Window, &str> {
        unsafe {
            let scale_factor = Self::get_scale_factor(width, height, opts.scale);

            let handle = Self::open_window(name, width, height, opts, scale_factor);

            if handle.is_none() {
                return Err("Unable to create Window");
            }

            let window = Window {
                mouse: MouseData::default(),
                dc: Some(user32::GetDC(handle.unwrap())),
                window: Some(handle.unwrap()),
                buffer: Vec::new(),
                key_handler: KeyHandler::new(),
                is_open: true,
                scale_factor: scale_factor,
                width: width as i32,
                height: height as i32,
                accel_table: None,
            };

            Ok(window)
        }
    }

    #[inline]
    pub fn get_window_handle(&self) -> *mut raw::c_void {
        self.window.unwrap() as *mut raw::c_void
    }

    #[inline]
    pub fn set_position(&mut self, x: isize, y: isize) {
        unsafe {
            user32::SetWindowPos(self.window.unwrap(), ptr::null_mut(), x as i32, y as i32,
                                 0, 0, winapi::SWP_SHOWWINDOW | winapi::SWP_NOSIZE);
        }
    }

    pub fn get_mouse_pos(&self, mode: MouseMode) -> Option<(f32, f32)> {
        let s = self.scale_factor as f32;
        let w = self.width as f32;
        let h = self.height as f32;

        // TODO: Needs to be fixed with resize support
        mouse_handler::get_pos(mode, self.mouse.x, self.mouse.y, s, w * s, h * s)
    }

    pub fn get_mouse_down(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.mouse.state[0],
            MouseButton::Middle => self.mouse.state[1],
            MouseButton::Right => self.mouse.state[2],
        }
    }

    pub fn get_scroll_wheel(&self) -> Option<(f32, f32)> {
        if self.mouse.scroll.abs() > 0.0 {
            Some((0.0, self.mouse.scroll))
        } else {
            None
        }
    }

    #[inline]
    pub fn get_keys(&self) -> Option<Vec<Key>> {
        self.key_handler.get_keys()
    }

    #[inline]
    pub fn get_keys_pressed(&self, repeat: KeyRepeat) -> Option<Vec<Key>> {
        self.key_handler.get_keys_pressed(repeat)
    }

    #[inline]
    pub fn is_key_down(&self, key: Key) -> bool {
        self.key_handler.is_key_down(key)
    }

    #[inline]
    pub fn set_key_repeat_delay(&mut self, delay: f32) {
        self.key_handler.set_key_repeat_delay(delay)
    }

    #[inline]
    pub fn set_key_repeat_rate(&mut self, rate: f32) {
        self.key_handler.set_key_repeat_rate(rate)
    }

    #[inline]
    pub fn is_key_pressed(&self, key: Key, repeat: KeyRepeat) -> bool {
        self.key_handler.is_key_pressed(key, repeat)
    }


    #[inline]
    pub fn is_open(&self) -> bool {
        return self.is_open
    }

    fn generic_update(&mut self, window: HWND) {
        unsafe {

            let mut point: winapi::POINT = mem::uninitialized();
            user32::GetCursorPos(&mut point);
            user32::ScreenToClient(window, &mut point);

            self.mouse.x = point.x as f32;
            self.mouse.y = point.y as f32;
            self.mouse.scroll = 0.0;

            self.key_handler.update();

            set_window_long(window, mem::transmute(self));
        }
    }

    fn message_loop(&mut self, window: HWND) {
        unsafe {
            let mut msg = mem::uninitialized();

            while user32::PeekMessageW(&mut msg, window, 0, 0, winapi::winuser::PM_REMOVE) != 0 {
                user32::TranslateMessage(&mut msg);
                user32::DispatchMessageW(&mut msg);
            }
        }
    }

    pub fn update_with_buffer(&mut self, buffer: &[u32]) {
        let window = self.window.unwrap();

        Self::generic_update(self, window);

        self.buffer = buffer.iter().cloned().collect();
        unsafe {
            user32::InvalidateRect(window, ptr::null_mut(), winapi::TRUE);
        }

        Self::message_loop(self, window);
    }

    pub fn update(&mut self) {
        let window = self.window.unwrap();

        Self::generic_update(self, window);
        Self::message_loop(self, window);
    }

    #[inline]
    pub fn is_active(&mut self) -> bool {
        // TODO: Proper implementation
        true
    }

    unsafe fn get_scale_factor(width: usize, height: usize, scale: Scale) -> i32 {
        let factor: i32 = match scale {
            Scale::X1 => 1,
            Scale::X2 => 2,
            Scale::X4 => 4,
            Scale::X8 => 8,
            Scale::X16 => 16,
            Scale::X32 => 32,
            Scale::FitScreen => {
                let screen_x = user32::GetSystemMetrics(winapi::winuser::SM_CXSCREEN) as i32;
                let screen_y = user32::GetSystemMetrics(winapi::winuser::SM_CYSCREEN) as i32;

                let mut scale = 1i32;

                loop {
                    let w = width as i32 * (scale + 1);
                    let h = height as i32 * (scale + 1);

                    if w > screen_x || h > screen_y {
                        break;
                    }

                    scale *= 2;
                }

                scale
            }
        };

        return factor;
    }

    fn map_key_to_vk_accel(key: Key) -> (raw::c_int, &'static str) {
        match key {
            Key::Key0 => (0x30, "0"),
            Key::Key1 => (0x31, "1"),
            Key::Key2 => (0x32, "2"),
            Key::Key3 => (0x33, "3"),
            Key::Key4 => (0x34, "4"),
            Key::Key5 => (0x35, "5"),
            Key::Key6 => (0x36, "6"),
            Key::Key7 => (0x37, "7"),
            Key::Key8 => (0x38, "8"),
            Key::Key9 => (0x39, "9"),

            Key::A => (0x41, "a"),
            Key::B => (0x42, "b"),
            Key::C => (0x43, "c"),
            Key::D => (0x44, "d"),
            Key::E => (0x45, "e"),
            Key::F => (0x46, "f"),
            Key::G => (0x47, "g"),
            Key::H => (0x48, "h"),
            Key::I => (0x49, "i"),
            Key::J => (0x4a, "j"),
            Key::K => (0x4b, "k"),
            Key::L => (0x4c, "l"),
            Key::M => (0x4d, "m"),
            Key::N => (0x4e, "n"),
            Key::O => (0x4f, "o"),
            Key::P => (0x50, "p"),
            Key::Q => (0x51, "q"),
            Key::R => (0x52, "r"),
            Key::S => (0x53, "s"),
            Key::T => (0x54, "t"),
            Key::U => (0x55, "u"),
            Key::V => (0x56, "v"),
            Key::W => (0x57, "w"),
            Key::X => (0x58, "x"),
            Key::Y => (0x59, "y"),
            Key::Z => (0x5a, "z"),

            Key::F1 => (winapi::winuser::VK_F1, "F1"),
            Key::F2 => (winapi::winuser::VK_F2, "F2"),
            Key::F3 => (winapi::winuser::VK_F3, "F3"),
            Key::F4 => (winapi::winuser::VK_F4, "F4"),
            Key::F5 => (winapi::winuser::VK_F5, "F5"),
            Key::F6 => (winapi::winuser::VK_F6, "F6"),
            Key::F7 => (winapi::winuser::VK_F7, "F7"),
            Key::F8 => (winapi::winuser::VK_F8, "F8"),
            Key::F9 => (winapi::winuser::VK_F9, "F9"),
            Key::F10 => (winapi::winuser::VK_F10, "F10"),
            Key::F11 => (winapi::winuser::VK_F11, "F11"),
            Key::F12 => (winapi::winuser::VK_F12, "F12"),
            Key::F13 => (winapi::winuser::VK_F13, "F14"),
            Key::F14 => (winapi::winuser::VK_F14, "F14"),
            Key::F15 => (winapi::winuser::VK_F15, "F15"),

            Key::Down => (winapi::winuser::VK_DOWN, "Down"),
            Key::Left => (winapi::winuser::VK_LEFT, "Left"),
            Key::Right => (winapi::winuser::VK_RIGHT, "Right"),
            Key::Up => (winapi::winuser::VK_UP, "Up"),

            Key::Backslash => (winapi::winuser::VK_OEM_102, "Backslash"),
            Key::Comma => (winapi::winuser::VK_OEM_COMMA, ","),
            Key::Minus => (winapi::winuser::VK_OEM_MINUS, "-"),
            Key::Period => (winapi::winuser::VK_OEM_PERIOD, "."),

            Key::Backspace => (winapi::winuser::VK_BACK, "Back"),
            Key::Delete => (winapi::winuser::VK_DELETE, "Delete"),
            Key::End => (winapi::winuser::VK_END, "End"),
            Key::Enter => (winapi::winuser::VK_RETURN, "Enter"),

            Key::Escape => (winapi::winuser::VK_ESCAPE, "Esc"),

            Key::Home => (winapi::winuser::VK_HOME, "Home"),
            Key::Insert => (winapi::winuser::VK_INSERT, "Insert"),
            Key::Menu => (winapi::winuser::VK_MENU, "Menu"),

            Key::PageDown => (winapi::winuser::VK_NEXT, "PageDown"),
            Key::PageUp => (winapi::winuser::VK_PRIOR, "PageUp"),

            Key::Pause => (winapi::winuser::VK_PAUSE, "Pause"),
            Key::Space => (winapi::winuser::VK_SPACE, "Space"),
            Key::Tab => (winapi::winuser::VK_TAB, "Tab"),
            Key::NumLock => (winapi::winuser::VK_NUMLOCK, "NumLock"),
            Key::CapsLock => (winapi::winuser::VK_CAPITAL, "CapsLock"),
            Key::ScrollLock => (winapi::winuser::VK_SCROLL, "Scroll"),

            Key::LeftShift => (winapi::winuser::VK_LSHIFT, "LeftShift"),
            Key::RightShift => (winapi::winuser::VK_RSHIFT, "RightShift"),
            Key::LeftCtrl => (winapi::winuser::VK_CONTROL, "Ctrl"),
            Key::RightCtrl => (winapi::winuser::VK_CONTROL, "Ctrl"),

            Key::NumPad0 => (winapi::winuser::VK_NUMPAD0, "NumPad0"),
            Key::NumPad1 => (winapi::winuser::VK_NUMPAD1, "NumPad1"),
            Key::NumPad2 => (winapi::winuser::VK_NUMPAD2, "NumPad2"),
            Key::NumPad3 => (winapi::winuser::VK_NUMPAD3, "NumPad3"),
            Key::NumPad4 => (winapi::winuser::VK_NUMPAD4, "NumPad4"),
            Key::NumPad5 => (winapi::winuser::VK_NUMPAD5, "NumPad5"),
            Key::NumPad6 => (winapi::winuser::VK_NUMPAD6, "NumPad6"),
            Key::NumPad7 => (winapi::winuser::VK_NUMPAD7, "NumPad7"),
            Key::NumPad8 => (winapi::winuser::VK_NUMPAD8, "NumPad8"),
            Key::NumPad9 => (winapi::winuser::VK_NUMPAD9, "NumPad9"),

            Key::LeftAlt => (winapi::winuser::VK_MENU, "Alt"),
            Key::RightAlt => (winapi::winuser::VK_MENU, "Alt"),

            Key::LeftSuper => (winapi::winuser::VK_LWIN, "LeftWin"),
            Key::RightSuper => (winapi::winuser::VK_RWIN, "RightWin"),

            _ => (0, "Unsupported"),
        }
    }


    //
    // When attaching a menu to the window we need to resize it so
    // the current client size is preserved and still show all pixels
    //
    unsafe fn adjust_window_size_for_menu(handle: HWND) {
        let mut rect: winapi::RECT = mem::uninitialized();

        let menu_height = user32::GetSystemMetrics(winapi::winuser::SM_CYMENU);

        user32::GetWindowRect(handle, &mut rect);
        user32::MoveWindow(handle, 
                           rect.left, 
                           rect.top, 
                           rect.right - rect.left, 
                           (rect.bottom - rect.top) + menu_height, 
                           1);
    }

    /*
    fn format_name(menu: &Menu, name: &`static str) -> String {

    }

    fn add_accel(&mut self, key: raw::c_int) {

    }

    fn add_menu_item(parent_menu: HMENU, menu_item: &Menu) {
        let item_name = to_wstring(m.name);
        let vk_accel = map_key_to_vk_accel(menu_item.key);

        match vk_accel.0 {
            0 => {
                user32::AppendMenuW(popup_menu, 0x10, menu_item.id as UINT_PTR, item_name.as_ptr());
            }
            
            _ => {
                fn add_accel(&mut self, key: raw::c_int) {
            }
        }

    }

    */

    unsafe fn recursive_add_menu(parent_menu: HMENU, name: &str, menu: &Vec<Menu>) {
        let menu_name = to_wstring(name);

        let popup_menu = user32::CreatePopupMenu();

        user32::AppendMenuW(parent_menu, 0x10, popup_menu as UINT_PTR, menu_name.as_ptr());

        for m in menu.iter() {
            let item_name = to_wstring(m.name);

            if let Some(ref sub_menu) = m.sub_menu {
                Self::recursive_add_menu(popup_menu, m.name, sub_menu);
            } else {
                if m.id == 0xffffffff {
                    user32::AppendMenuW(popup_menu, 0x800, 0, ptr::null()); // separator
                } else {
                    user32::AppendMenuW(popup_menu, 0x10,
                                        m.id as UINT_PTR,
                                        item_name.as_ptr());
                }
            }
        }
    }

    pub fn add_menu(&mut self, menu_name: &str, menu: &Vec<Menu>) {
        unsafe {
            let window = self.window.unwrap();
            let mut main_menu = user32::GetMenu(window);

            if main_menu == ptr::null_mut() {
                main_menu = user32::CreateMenu();
                user32::SetMenu(window, main_menu);
                Self::adjust_window_size_for_menu(window);
            }

            Self::recursive_add_menu(main_menu, menu_name, menu);

            user32::DrawMenuBar(window);
        }
    }
    pub fn update_menu(&mut self, _menu_name: &str, _menu: &Vec<Menu>) {
        // not implemented yet
    }
    pub fn remove_menu(&mut self, _menu_name: &str) {
        // not implemented yet
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            if self.dc.is_some() {
                user32::ReleaseDC(self.window.unwrap(), self.dc.unwrap());
            }

            if self.window.is_some() {
                user32::CloseWindow(self.window.unwrap());
            }
        }
    }
}
