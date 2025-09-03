//! ReactOS Calc64 - Calculadora simple en Rust con Windows API
//! Port de la calculadora de ReactOS usando el crate windows

use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::*,
};

static mut CALC_VALUE: f64 = 0.0;
static mut OPERATION: i32 = 0; // 0=none, 1=+, 2=-, 3=*, 4=/
static mut STORED_VALUE: f64 = 0.0;
static mut DISPLAY_BUFFER: [u8; 32] = [0; 32];

unsafe fn update_display() {
    let value_str = format!("{:.6}", CALC_VALUE);
    let bytes = value_str.as_bytes();
    let len = bytes.len().min(31);
    DISPLAY_BUFFER[..len].copy_from_slice(&bytes[..len]);
    DISPLAY_BUFFER[len] = 0;
}

unsafe fn perform_calculation() {
    match OPERATION {
        1 => CALC_VALUE = STORED_VALUE + CALC_VALUE,
        2 => CALC_VALUE = STORED_VALUE - CALC_VALUE,
        3 => CALC_VALUE = STORED_VALUE * CALC_VALUE,
        4 => if CALC_VALUE != 0.0 { CALC_VALUE = STORED_VALUE / CALC_VALUE },
        _ => {}
    }
    OPERATION = 0;
    update_display();
}

unsafe fn button_click(button_id: i32) {
    match button_id {
        0..=9 => {
            if OPERATION != 0 {
                CALC_VALUE = 0.0;
                OPERATION = 0;
            }
            CALC_VALUE = CALC_VALUE * 10.0 + button_id as f64;
            update_display();
        }
        10 => { // Clear
            CALC_VALUE = 0.0;
            OPERATION = 0;
            STORED_VALUE = 0.0;
            update_display();
        }
        11 => { // Plus
            STORED_VALUE = CALC_VALUE;
            OPERATION = 1;
        }
        12 => { // Minus
            STORED_VALUE = CALC_VALUE;
            OPERATION = 2;
        }
        13 => { // Multiply
            STORED_VALUE = CALC_VALUE;
            OPERATION = 3;
        }
        14 => { // Divide
            STORED_VALUE = CALC_VALUE;
            OPERATION = 4;
        }
        15 => { // Equals
            perform_calculation();
        }
        _ => {}
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            // Crear botones de la calculadora
            let buttons = [
                ("7", 0, 50, 50, 30, 7),
                ("8", 0, 100, 50, 30, 8),
                ("9", 0, 150, 50, 30, 9),
                ("/", 0, 200, 50, 30, 14),
                ("4", 0, 50, 100, 30, 4),
                ("5", 0, 100, 100, 30, 5),
                ("6", 0, 150, 100, 30, 6),
                ("*", 0, 200, 100, 30, 13),
                ("1", 0, 50, 150, 30, 1),
                ("2", 0, 100, 150, 30, 2),
                ("3", 0, 150, 150, 30, 3),
                ("-", 0, 200, 150, 30, 12),
                ("0", 0, 50, 200, 30, 0),
                ("C", 0, 100, 200, 30, 10),
                ("=", 0, 150, 200, 30, 15),
                ("+", 0, 200, 200, 30, 11),
            ];
            
            for (text, x, y, w, h, id) in &buttons {
                CreateWindowExA(
                    WINDOW_EX_STYLE::default(),
                    s!("BUTTON"),
                    PCSTR(text.as_ptr()),
                    WS_VISIBLE | WS_CHILD | WINDOW_STYLE(BS_PUSHBUTTON as u32),
                    *x, *y, *w, *h,
                    hwnd,
                    HMENU(*id as isize),
                    GetModuleHandleA(None).unwrap(),
                    None,
                );
            }
            
            update_display();
            LRESULT(0)
        }
        WM_COMMAND => {
            let button_id = (wparam.0 & 0xFFFF) as i32;
            button_click(button_id);
            LRESULT(0)
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            
            // Dibujar display
            let display_text = std::str::from_utf8_unchecked(&DISPLAY_BUFFER);
            TextOutA(hdc, 10, 10, display_text.as_bytes());
            
            EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}

fn main() -> Result<()> {
    unsafe {
        let hinstance = GetModuleHandleA(None)?;
        
        let class_name = s!("ReactOSCalc64");
        let wc = WNDCLASSA {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance.into(),
            hIcon: LoadIconA(hinstance, PCSTR::null()).unwrap_or_default(),
            hCursor: LoadCursorA(hinstance, PCSTR::null()).unwrap_or_default(),
            hbrBackground: HBRUSH(GetStockObject(WHITE_BRUSH).0),
            lpszMenuName: PCSTR::null(),
            lpszClassName: class_name,
        };
        
        RegisterClassA(&wc);
        
        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            class_name,
            s!("ReactOS Calc64 - Rust"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            300,
            300,
            None,
            None,
            hinstance,
            None,
        );
        
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
        
        let mut msg = MSG::default();
        while GetMessageA(&mut msg, None, 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
        
        Ok(())
    }
}
