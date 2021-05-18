use std::{env, fs};
use sys_info;
use whoami;

fn main() {
  println!(r#"{bold}       {white}_..oo8{black}"""Y8b.._
     {white}.88888888o.{black}    "Yb.
   {white}.d888P""Y8888b{black}      "b
   {white}o88888    88888){black}       "b	{brightMagenta}{user}{white}@{brightMagenta}{host}
 {white}d888888b..d8888P{black}         'b	{blue}distro 	{white}::{blue} {distro}
 {white}88888888888888"{black}           8	{green}uptime 	{white}::{green} {uptime}
{white}(88DWB8888888P{black}             8)	{brightRed}wm 	{white}::{brightRed} {wm}
 {white}8888888888P{black}               8	{brightYellow}shell	{white}::{brightYellow} {shell}
 {white}Y88888888P{black}     {white}ee{black}        .P	{brightCyan}ram 	{white}::{brightCyan} {ram}
  {white}Y888888({black}     {white}8888{black}      oP
   {white}"Y88888b{black}     {white}""{black}     oP"
     {white}"Y8888o._{black}    _.oP"
       {white}`""Y888boodP""'"#,
           bold = "\x1b[1m",
           black = "\x1b[30m",
           green = "\x1b[32m",
           blue = "\x1b[34m",
           white = "\x1b[37m",
           brightRed = "\x1b[91m",
           brightYellow = "\x1b[93m",
           brightMagenta = "\x1b[95m",
           brightCyan = "\x1b[96m",
           user = whoami::username(),
           host = whoami::hostname(),
           distro = whoami::distro(),
           uptime = get_uptime(),
           wm = get_wm(),
           shell = get_shell(),
           ram = get_mem_info())
}

///
/// Returns the window manager of the current user, or none if they don't use one.
///
fn get_wm() -> String {
  return env::var("DESKTOP_SESSION")
    .or(env::var("XDG_SESSION_DESKTOP"))
    .or(env::var("XDG_CURRENT_DESKTOP"))
    .or(env::var("GDMSESSION"))
    .unwrap_or(String::from("none"));
}

///
/// Returns the current shell that the user is using.
///
fn get_shell() -> String {
  let mut shell = env::var("SHELL")
    .or(env::var("SESSIONNAME"))
    .unwrap_or("unknown".to_string());

  let os_type = sys_info::os_type();
  if os_type.is_ok() && os_type.unwrap().to_lowercase() == "linux" {
    shell = shell
      .split("/")
      .last()
      .unwrap()
      .to_string();
  }

  return shell;
}

///
/// Returns a string containing the current uptime.
///
fn get_uptime() -> String {
  let s = match fs::read_to_string("/proc/uptime") {
    Ok(s) => s,
    Err(_e) => return "unknown".to_string()
  };

  let seconds = s
    .split_whitespace()
    .next()
    .unwrap_or("");

  let s: f64 = seconds.parse().unwrap();
  let mut uptime = "".to_string();

  let d = (s / 60.0 / 60.0 / 24.0).floor();
  if d != 0.0 { uptime.push_str(format!("{:1}d ", d).as_str()) }

  let h = (s / 60.0 / 60.0 % 24.0).floor();
  if h != 0.0 { uptime.push_str(format!("{:1}h ", h).as_str()) };

  let m = (s / 60.0 % 24.0).floor();
  if m != 0.0 { uptime.push_str(format!("{:1}m", m).as_str()) };

  return uptime.trim().to_string();
}

///
/// Returns a string display the used / total system memory.
///
fn get_mem_info() -> String {
  return match sys_info::mem_info() {
    Ok(info) => {
      let used = (info.total - info.avail) / 1024;
      let total = info.total / 1024;
      format!("{}mb / {}mb", used, total)
    }

    Err(_err) => "unknown".to_string()
  };
}
