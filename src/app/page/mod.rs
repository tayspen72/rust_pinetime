//==============================================================================
// Notes
//==============================================================================
// app::page::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
pub mod home;
pub mod log;
pub mod notifications;
pub mod settings;
pub mod startup;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
pub enum AppPage {
	Home,
	Notifications,
	Log,
	Settings,
	Startup
}

//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Public Functions
//==============================================================================


//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
#[allow(dead_code)]
pub fn task_handler(d: &mut crate::app::info::DeviceInfo) {
	if d.change_flags.app_page{
		match d.app_page {
			AppPage::Home => home::print_page(),
			AppPage::Notifications => notifications::print_page(),
			AppPage::Log => log::print_page(),
			AppPage::Settings => settings::print_page(),
			AppPage::Startup => startup::print_page(),
		}
	}
	match d.app_page {
		AppPage::Home => home::task_handler(),
		AppPage::Notifications => notifications::task_handler(),
		AppPage::Log => log::task_handler(),
		AppPage::Settings => settings::task_handler(),
		AppPage::Startup => startup::task_handler(),
	}

}