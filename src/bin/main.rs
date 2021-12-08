use std::fs;
use std::time::Instant;
use wt_datamine_extractor_lib::lang::extract_csv;
use wt_datamine_extractor_lib::missile::extract_missiles::KnownMissiles;
use wt_datamine_extractor_lib::missile::missile::Missile;
use wt_datamine_extractor_lib::shell::demarre::{DemarreMod, shell_to_demarre};

use wt_datamine_extractor_lib::shell::known_shells::KnownShells;
use wt_datamine_extractor_lib::shell::shells::{Shell};
use wt_datamine_extractor_lib::thermal::extract_thermals::KnownThermals;
use wt_datamine_extractor_lib::thermal::thermals::Thermal;


fn main() {
	let start = Instant::now();

	if fs::read_dir("resources/cache").is_ok() {
		fs::write("meta_index/version.txt", &fs::read_to_string("resources/cache/aces.vromfs.bin_u/version").unwrap()).unwrap();


		extract_csv();

		let known_missiles = KnownMissiles::generate_index().write_index().copy_index_to_folder();
		let known_thermals = KnownThermals::generate_index().write_index().copy_index_to_folder();
		let known_shells = KnownShells::generate_index().write_index().copy_index_to_folder();

		let missiles = Missile::generate_from_index(&known_missiles);
		let thermals = Thermal::generate_from_index(&known_thermals);
		let shells = Shell::generate_from_index(&known_shells);

		Missile::write_all(missiles);
		Thermal::write_all(thermals);
		Shell::write_all(shells);
	} else {
		panic!("Local mined cache is invalid or could not be read");
	}

	println!("{}", shell_to_demarre(1000.0, 88.0, 10.16, {
		DemarreMod {
			penetration_k: 1.0,
			speed_pow: 1.43,
			mass_pow: 0.71,
			caliber_pow: 1.07
		}
	}));

	println!("{:?}", start.elapsed());
}