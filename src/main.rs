use raw_window_handle::{HasRawWindowHandle, HasWindowHandle, RawWindowHandle};
use vizia::prelude::*;
use winapi::shared::windef::HWND;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("src/style.css"))
            .expect("Failed to load stylesheet");

        AppData { is_blurred: false }.build(cx);

        HStack::new(cx, |cx| {
            Label::new(cx, "Hello, world!").background_color(Color::transparent());
        })
        .background_color(Color::transparent());

        cx.emit(AppEvent::ApplyBlur);
    })
    .transparent(true)
    .vsync(false)
    .on_create(|_| {
        // can't work
        println!("Created");
    })
    .run()
}

#[derive(Lens)]
struct AppData {
    is_blurred: bool,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event: &AppEvent, _| match app_event {
            AppEvent::ApplyBlur => {
                if let Some(window) = cx.window() {
                    if !self.is_blurred {
                        println!("Applying blur");
                        window.set_title("Blurred Window");

                        // 在 Windows 平台上，修改窗口样式：去除标题栏，但保留边框以便调整大小
                        #[cfg(target_os = "windows")]
                        {
                            use winapi::um::winuser::{
                                GetWindowLongW, SetWindowLongW, SetWindowPos, GWL_STYLE,
                                SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, WS_CAPTION,
                            };

                            let handle = window.window_handle().unwrap();
                            let hwnd: HWND = match handle.as_raw() {
                                RawWindowHandle::Win32(win32_handle) => {
                                    win32_handle.hwnd.get() as HWND
                                }
                                _ => panic!("Not a Win32 window"),
                            };

                            unsafe {
                                let style = GetWindowLongW(hwnd, GWL_STYLE);
                                // 去除标题栏 (WS_CAPTION)，保留其他样式（如 WS_THICKFRAME 用于调整大小）
                                SetWindowLongW(hwnd, GWL_STYLE, style & !WS_CAPTION as i32);
                                // 刷新窗口的非客户区，使样式改变生效
                                SetWindowPos(
                                    hwnd,
                                    std::ptr::null_mut(),
                                    0,
                                    0,
                                    0,
                                    0,
                                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
                                );
                            }
                        }

                        window_vibrancy::apply_acrylic(window, Some((0, 0, 0, 0))).unwrap();
                        // window_vibrancy::apply_mica(window, Some(false)).unwrap();
                        self.is_blurred = true;
                    }
                }
            }
        });
    }
}

pub enum AppEvent {
    ApplyBlur,
}
