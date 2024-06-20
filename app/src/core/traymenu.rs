use tray_icon::menu::{Menu, MenuEvent, MenuId, MenuItemBuilder};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent};
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};

use super::config;

const ICONBYTES: &[u8] = include_bytes!(concat!(
  env!("CARGO_MANIFEST_DIR"),
  "/../extension/icons/icon128.png"
));

pub fn run_tray() -> Result<(), EventLoopError> {
  #[cfg(target_os = "linux")]
  // create tray icon and init gtk on linux
  std::thread::spawn(|| {
    let icon = load_icon(ICONBYTES);

    gtk::init().unwrap();

    let menu = Box::new(Menu::new());

    let folder_btn = MenuItemBuilder::new()
      .text("Open Program Folder")
      .id(MenuId("folder".to_string()))
      .enabled(true)
      .build();

    let quit_btn = MenuItemBuilder::new()
      .text("Quit")
      .id(MenuId("quit".to_string()))
      .enabled(true)
      .build();

    menu
      .append_items(&[&folder_btn, &quit_btn])
      .expect("Unable to create tray menu");

    let _tray_icon = TrayIconBuilder::new()
      .with_menu(menu)
      .with_tooltip("OrpheusDL")
      .with_icon(load_icon(ICONBYTES))
      .with_title("OrpheusDL")
      .build()
      .unwrap();

    gtk::main();
  });

  // setup event loop
  let event_loop = EventLoop::builder().build().unwrap();

  // setup channels for interaction events
  let menu_channel = MenuEvent::receiver();
  let tray_channel = TrayIconEvent::receiver();

  #[cfg(not(target_os = "linux"))]
  // create tray icon on other os
  let mut tray_icon: Option<TrayIcon> = None;

  // TODO: convert this to run_app and create an App impl for this.
  event_loop.run(move |event, event_loop| {
    event_loop.set_control_flow(ControlFlow::WaitUntil(
      std::time::Instant::now() + std::time::Duration::from_millis(16),
    ));

    #[cfg(not(target_os = "linux"))]
    if let winit::event::Event::NewEvents(winit::event::StartCause::Init) = event {
      let menu = Box::new(Menu::new());

      let folder_btn = MenuItemBuilder::new()
        .text("Open Program Folder")
        .id(MenuId("folder".to_string()))
        .enabled(true)
        .build();

      let quit_btn = MenuItemBuilder::new()
        .text("Quit")
        .id(MenuId("quit".to_string()))
        .enabled(true)
        .build();

      menu
        .append_items(&[&folder_btn, &quit_btn])
        .expect("Unable to create tray menu");

      tray_icon = Some(
        TrayIconBuilder::new()
          .with_menu(menu)
          .with_tooltip("OrpheusDL")
          .with_icon(load_icon(ICONBYTES))
          .with_title("OrpheusDL")
          .build()
          .unwrap(),
      );
      #[cfg(target_os = "macos")]
      unsafe {
        use core_foundation::runloop::{CFRunLoopGetMain, CFRunLoopWakeUp};

        let rl = CFRunLoopGetMain();
        CFRunLoopWakeUp(rl);
      }
    }

    if let Ok(event) = menu_channel.try_recv() {
      if event.id.0 == "folder".to_string() {
        open::that_detached(config::Config::get_folder()).expect("Failed to open program folder");
      } else if event.id.0 == "quit".to_string() {
        event_loop.exit();
      } else {
        debug!("Menu Event: {:?}", event)
      }
    }
  })
}

fn load_icon(bytes: &[u8]) -> Icon {
  let (icon_rgba, icon_width, icon_height) = {
    let image = image::load_from_memory(&bytes)
      .expect("Failed to open icon path")
      .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  };
  Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
