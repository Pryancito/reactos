//! Aplicación de prueba 32-bit para WOW64
//! 
//! Prueba la compatibilidad de aplicaciones 32-bit en ReactOS Rust

use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::*,
};

/// Función principal de la aplicación 32-bit
fn main() -> Result<()> {
    unsafe {
        let hinstance = GetModuleHandleA(None)?;
        
        // Crear ventana de prueba
        let class_name = s!("Test32Window");
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
            s!("ReactOS Rust - Test32 (32-bit)"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            400,
            300,
            None,
            None,
            hinstance,
            None,
        );
        
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
        
        // Crear botón de prueba
        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            s!("BUTTON"),
            s!("Test 32-bit"),
            WS_VISIBLE | WS_CHILD | WINDOW_STYLE(BS_PUSHBUTTON as u32),
            50, 50, 100, 30,
            hwnd,
            HMENU(1),
            hinstance,
            None,
        );
        
        // Crear etiqueta de información
        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            s!("STATIC"),
            s!("Aplicación 32-bit ejecutándose en WOW64"),
            WS_VISIBLE | WS_CHILD,
            50, 100, 300, 20,
            hwnd,
            HMENU(2),
            hinstance,
            None,
        );
        
        // Crear etiqueta de arquitectura
        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            s!("STATIC"),
            s!("Arquitectura: x86 (32-bit)"),
            WS_VISIBLE | WS_CHILD,
            50, 130, 300, 20,
            hwnd,
            HMENU(3),
            hinstance,
            None,
        );
        
        // Crear etiqueta de WOW64
        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            s!("STATIC"),
            s!("WOW64: Activado"),
            WS_VISIBLE | WS_CHILD,
            50, 160, 300, 20,
            hwnd,
            HMENU(4),
            hinstance,
            None,
        );
        
        // Loop de mensajes
        let mut msg = MSG::default();
        while GetMessageA(&mut msg, None, 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
        
        Ok(())
    }
}

/// Procedimiento de ventana
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            LRESULT(0)
        }
        WM_COMMAND => {
            let button_id = (wparam.0 & 0xFFFF) as i32;
            match button_id {
                1 => {
                    // Botón de prueba presionado
                    MessageBoxA(
                        hwnd,
                        s!("¡Funciona! Aplicación 32-bit ejecutándose en WOW64"),
                        s!("Test32 - WOW64"),
                        MB_OK | MB_ICONINFORMATION,
                    );
                }
                _ => {}
            }
            LRESULT(0)
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            
            // Dibujar texto de prueba
            TextOutA(hdc, 10, 10, s!("ReactOS Rust - Test32 (32-bit)"));
            TextOutA(hdc, 10, 30, s!("WOW64 Compatibility Layer"));
            
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
