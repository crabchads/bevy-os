use bevy::{
	app::Update,
	ecs::entity::Entity,
	prelude::{Component, Query},
};
use log::info;

#[derive(Component)]
pub struct Process {
	pub id: u32,
	pub state: ProcessState,
}

#[derive(Component)]
pub struct ElfProcess {
	pub elf: goblin::elf::Elf<'static>,
}

#[derive(Debug, PartialEq)]
pub enum ProcessState {
	Starting,
	Running,
	Blocked,
	Terminated,
}

pub struct TaskPlugin;

impl bevy::app::Plugin for TaskPlugin {
	fn build(&self, app: &mut bevy::app::App) {
		app.add_systems(Update, setup_tasks);
	}
}

fn setup_tasks(
	mut processes: Query<(Entity, &mut Process)>,
	elf_processes: Query<&ElfProcess>,
) {
	for (entity, mut process) in processes.iter_mut() {
		if process.state == ProcessState::Starting {
			match elf_processes.get(entity) {
				Ok(elf_process) => {
					info!("Starting process {}", process.id);

					process.state = ProcessState::Running;
				}
				_ => {}
			}
		}
	}
}
