use bevy::{
	app::Update,
	ecs::entity::Entity,
	prelude::{Component, Query},
};

use crate::println;

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
	processes: Query<(Entity, &Process)>,
	elf_processes: Query<&ElfProcess>,
) {
	for (entity, process) in processes.iter() {
		if process.state == ProcessState::Starting {
			match elf_processes.get(entity) {
				Ok(elf_process) => {
					println!("Starting process {}", process.id);
				}
				_ => {}
			}
		}
	}
}
