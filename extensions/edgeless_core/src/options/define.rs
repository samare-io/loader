use std::path::PathBuf;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
  pub static ref PATH_OPTIONS: PathBuf = PathBuf::from("Config");

  pub static ref PATH_OLD_CUSTOM_DISPLAY_RES_OPTIONS: PathBuf = PathBuf::from("分辨率.txt");
  pub static ref PATH_CUSTOM_DISPLAY_RES: PathBuf = PATH_OPTIONS.join("分辨率.txt");

  pub static ref PATH_CUSTOM_WALLPAPER: PathBuf = PathBuf::from("wp.jpg");
  pub static ref PATH_CUSTOM_WALLPAPER_BAK: PathBuf = PathBuf::from("wp_backup.jpg");
  pub static ref PATH_EXTERNAL_LAUNCHER: PathBuf = PathBuf::from("Launcher.cmd");

  pub static ref EXT_PLUGIN_PATTERN: Regex = Regex::new("7z.*").unwrap();
  pub static ref EXT_PLUGIN_NORMAL: &'static str = "7z";
  pub static ref EXT_PLUGIN_DISABLE: &'static str = "7zf";
  pub static ref EXT_PLUGIN_LOCALBOOST: &'static str = "7zl";
  pub static ref EXT_PLUGIN_DOTNET: &'static str = "7zn";
  pub static ref PATH_PLUGIN_RESOURCES: PathBuf = PathBuf::from("Resource");

  pub static ref EXT_THEME_PACK: &'static str = "eth";
  pub static ref EXT_THEME_ICON: &'static str = "eis";
  pub static ref EXT_THEME_CURSOR_STYLE: &'static str = "ems";
  pub static ref EXT_THEME_SIB_CONFIG: &'static str = "esc";
  pub static ref EXT_THEME_SYS_ICON: &'static str = "ess";
  pub static ref EXT_THEME_LOADSCREEN: &'static str = "els";
  pub static ref PATH_THEME_PACK_WALLPAPAER: PathBuf = PathBuf::from("WallPaper.jpg");
  pub static ref PATH_THEME_PACK_INTRO_FOLDER: PathBuf = PathBuf::from("Intro");
  pub static ref PATH_THEME_PACK_INTRO_TEXT: PathBuf = PathBuf::from("Intro.txt");
  pub static ref PATH_THEME_PACK_INTRO_SCRIPT: PathBuf = PathBuf::from("Intro.wcs");

  pub static ref PATH_THEME_DEFAULT: PathBuf = PathBuf::from("Default");
  pub static ref PATH_THEME_DEFAULT_ICON: PathBuf = PATH_THEME_DEFAULT.join("IconPack.eis");
  pub static ref PATH_THEME_DEFAULT_CURSOR_STYLE: PathBuf = PATH_THEME_DEFAULT.join("MouseStyle.ems");
  pub static ref PATH_THEME_DEFAULT_SIB_CONFIG: PathBuf = PATH_THEME_DEFAULT.join("StartIsBackConfig.esc");
  pub static ref PATH_THEME_DEFAULT_SYS_ICON: PathBuf = PATH_THEME_DEFAULT.join("SystemIconPack.ess");
  pub static ref PATH_THEME_DEFAULT_LOCALSCREEN_FOLDER: PathBuf = PATH_THEME_DEFAULT.join("LocalScreen");
  pub static ref PATH_THEME_DEFAULT_INTRO_FOLDER: PathBuf = PATH_THEME_DEFAULT.join("Intro");
  pub static ref PATH_THEME_DEFAULT_INTRO_TEXT: PathBuf = PathBuf::from("Intro.txt");
  pub static ref PATH_THEME_DEFAULT_INTRO_SCRIPT: PathBuf = PathBuf::from("Intro.wcs");

  pub static ref PATH_OPTION_ALLOW_EXTERNAL_LAUNCHER: PathBuf = PATH_OPTIONS.join("Developer");
  pub static ref PATH_OPTION_IGNORE_OUTDATE: PathBuf = PATH_OPTIONS.join("NoOutDateCheck");
  pub static ref PATH_OPTION_DISABLE_USB_MANAGER: PathBuf = PATH_OPTIONS.join("DisableUSBManager");
  pub static ref PATH_OPTION_DISABLE_SMART_ISO: PathBuf = PATH_OPTIONS.join("DisableSmartISO");
  pub static ref PATH_OPTION_UNFOLD_RIBBON: PathBuf = PATH_OPTIONS.join("UnfoldRibbon");
  pub static ref PATH_OPTION_REBOOT_DEFAULT: PathBuf = PATH_OPTIONS.join("RebootDefault");
  pub static ref PATH_OPTION_DISABLE_RECYCLE_BIN: PathBuf = PATH_OPTIONS.join("DisableRecycleBin");
  pub static ref PATH_OPTION_AUTO_UNATTEND: PathBuf = PATH_OPTIONS.join("AutoUnattend");
  pub static ref PATH_OPTION_DRV_UP_ACT: PathBuf = PATH_OPTIONS.join("UpActDrv");
  pub static ref PATH_OPTION_DRV_WIN_FIRST: PathBuf = PATH_OPTIONS.join("WinFirst");
  pub static ref PATH_OPTION_DRV_ORDER_ANOTHER: PathBuf = PATH_OPTIONS.join("OrderDrvAnotherWay");
  pub static ref PATH_OPTION_DRV_MOUNT_EVERY_PART: PathBuf = PATH_OPTIONS.join("MountEveryPartition");
  pub static ref PATH_OPTION_DISABLE_LOADSCREEN: PathBuf = PATH_OPTIONS.join("DisableLoadScreen");
  pub static ref PATH_OPTION_DISABLE_PIN_BROWSERS: PathBuf = PATH_OPTIONS.join("DisablePinBrowsers");
}

#[derive(Debug, Clone)]
pub enum ProfileOption {
  AllowExternalLauncher = PATH_OPTION_ALLOW_EXTERNAL_LAUNCHER,
  IgnoreOutdate = PATH_OPTION_IGNORE_OUTDATE,
  DisableUSBManager = PATH_OPTION_DISABLE_USB_MANAGER,
  DisableSmartISO = PATH_OPTION_DISABLE_SMART_ISO,
  UnfoldRibbon = PATH_OPTION_UNFOLD_RIBBON,
  RebootDefault = PATH_OPTION_REBOOT_DEFAULT,
  DisableRecycleBin = PATH_OPTION_DISABLE_RECYCLE_BIN,
  AutoUnattend = PATH_OPTION_AUTO_UNATTEND,
  DriveUpActive = PATH_OPTION_DRV_UP_ACT,
  DriveWindowsFirst = PATH_OPTION_DRV_WIN_FIRST,
  DriveOrderAnotherWay = PATH_OPTION_DRV_ORDER_ANOTHER,
  DriveMountEveryPartition = PATH_OPTION_DRV_MOUNT_EVERY_PART,
  DisableLoadScreen = PATH_OPTION_DISABLE_LOADSCREEN,
  DisablePinBrowsers = PATH_OPTION_DISABLE_PIN_BROWSERS,
}

#[derive(Debug, Clone, Copy)]
pub struct ProfileOptions {
  allow_external_laucher: bool,
  ignore_outdate: bool,
  disable_usb_manager: bool,
  disable_smart_iso: bool,
  unfold_ribbon: bool,
  reboot_default: bool,
  disable_recycle_bin: bool,
  auto_unattend: bool,
  drive_up_active: bool,
  drive_windows_first: bool,
  drive_order_another_way: bool,
  drive_mount_every_partition: bool,
  disable_loadscreen: bool,
  disable_pin_browsers: bool,
}

impl Default for ProfileOptions {
  fn default() -> Self {
      Self {
        allow_external_laucher: false,
        ignore_outdate: false,
        disable_usb_manager: false,
        disable_smart_iso: false,
        unfold_ribbon: false,
        reboot_default: false,
        disable_recycle_bin: false,
        auto_unattend: false,
        drive_up_active: false,
        drive_windows_first: false,
        drive_order_another_way: false,
        drive_mount_every_partition: false,
        disable_loadscreen: false,
        disable_pin_browsers: false,
      }
  }
}