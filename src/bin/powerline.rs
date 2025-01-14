extern crate powerline;

use std::env;

#[cfg(feature = "time")]
use powerline::modules::Time;
use powerline::{modules::*, theme::SimpleTheme};

const GIT_ENABLED_FLAG: &str = "git";
const GIT_DISABLED_FLAG: &str = "-git";
const READONLY_ENABLED_FLAG: &str = "readonly";
const READONLY_DISABLED_FLAG: &str = "-readonly";
const CMD_ENABLED_FLAG: &str = "cmd";
const CMD_DISABLED_FLAG: &str = "-cmd";
const EXITCODE_ENABLED_FLAG: &str = "exitcode";
const EXITCODE_DISABLED_FLAG: &str = "-exitcode";
const USER_ENABLED_FLAG: &str = "user";
const USER_DISABLED_FLAG: &str = "-user";
const HOST_ENABLED_FLAG: &str = "host";
const HOST_DISABLED_FLAG: &str = "-host";
const CWD_ENABLED_FLAG: &str = "cwd";
const CWD_DISABLED_FLAG: &str = "-cwd";
const PYVENV_ENABLED_FLAG: &str = "pyvenv";
const PYVENV_DISABLED_FLAG: &str = "-pyvenv";

#[cfg(feature = "time")]
const TIME_ENABLED_FLAG: &str = "time";
#[cfg(feature = "time")]
const TIME_DISABLED_FLAG: &str = "-time";

fn main() {
	let mut prompt = powerline::Powerline::new();

	let mut pyvenv_enabled = true;
	let mut user_enabled = true;
	let mut host_enabled = false;
	let mut cwd_enabled = true;
	let mut git_enabled = true;
	let mut readonly_enabled = true;
	let mut cmd_enabled = true;
	let mut exitcode_enabled = false;

	#[cfg(feature = "time")]
	let mut time_enabled = true;

	if cfg!(feature = "cli-options") {
		for arg in env::args() {
			match arg.as_str() {
				GIT_ENABLED_FLAG => git_enabled = true,
				GIT_DISABLED_FLAG => git_enabled = false,
				READONLY_ENABLED_FLAG => readonly_enabled = true,
				READONLY_DISABLED_FLAG => readonly_enabled = false,
				CMD_ENABLED_FLAG => cmd_enabled = true,
				CMD_DISABLED_FLAG => cmd_enabled = false,
				EXITCODE_ENABLED_FLAG => exitcode_enabled = true,
				EXITCODE_DISABLED_FLAG => exitcode_enabled = false,
				USER_ENABLED_FLAG => user_enabled = true,
				USER_DISABLED_FLAG => user_enabled = false,
				HOST_ENABLED_FLAG => host_enabled = true,
				HOST_DISABLED_FLAG => host_enabled = false,
				CWD_ENABLED_FLAG => cwd_enabled = true,
				CWD_DISABLED_FLAG => cwd_enabled = false,
				PYVENV_ENABLED_FLAG => pyvenv_enabled = true,
				PYVENV_DISABLED_FLAG => pyvenv_enabled = false,
				_ => {},
			}
			#[cfg(feature = "time")]
			{
				match arg.as_str() {
					TIME_ENABLED_FLAG => time_enabled = true,
					TIME_DISABLED_FLAG => time_enabled = false,
					_ => {},
				}
			}
		}
	}

	#[cfg(feature = "time")]
	{
		if time_enabled {
			measure_elapsed("time", || prompt.add_module(Time::<SimpleTheme>::with_time_format("%H:%M:%S")));
		}
	}
	if pyvenv_enabled {
		measure_elapsed("pyvenv", || prompt.add_module(PyVenv::<SimpleTheme>::new()));
	}
	if user_enabled {
		measure_elapsed("user", || prompt.add_module(User::<SimpleTheme>::new()));
	}
	if host_enabled {
		measure_elapsed("host", || prompt.add_module(Host::<SimpleTheme>::new()));
	}
	if cwd_enabled {
		measure_elapsed("cwd", || prompt.add_module(Cwd::<SimpleTheme>::new(45, 4, false)));
	}
	if git_enabled {
		measure_elapsed("git", || prompt.add_module(Git::<SimpleTheme>::new()));
	}
	if readonly_enabled {
		measure_elapsed("readonly", || prompt.add_module(ReadOnly::<SimpleTheme>::new()));
	}
	if cmd_enabled {
		measure_elapsed("cmd", || prompt.add_module(Cmd::<SimpleTheme>::new()));
	}
	if exitcode_enabled {
		measure_elapsed("exitcode", || prompt.add_module(ExitCode::<SimpleTheme>::new()));
	}
	println!("{}", prompt);
}

fn measure_elapsed(label: &str, mut expr: impl FnMut()) {
	let _ = label;
	#[cfg(feature = "print-module-timings")]
	let start = std::time::Instant::now();
	let result = expr();
	#[cfg(feature = "print-module-timings")]
	if env::var("POWERLINE_DEBUG_TIMINGS").unwrap_or("".to_string()) == "1" {
		println!("{} completed in {:#?}", label, std::time::Instant::now().duration_since(start));
	}
	result
}
