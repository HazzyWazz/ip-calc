#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::net::Ipv4Addr;

use eframe::egui;

// mod old_calc;
mod calc;

fn main() -> eframe::Result {

	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 240.0]),
		..Default::default()
	};

	let mut ip_box = "address".to_owned();
	let mut subnet_length_box = "mask or length".to_owned();

	let mut network: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
	let mut class: String = String::new();
	
	let mut subnet: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
	let mut wildcard_bits: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
	let mut first_host: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
	let mut last_host: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
	let mut broadcast_address: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
	let mut hosts: u32 = 0;



	eframe::run_ui_native("IP Calc", options, move |ui, _frame| {
		egui::CentralPanel::default().show_inside(ui, |ui| {
			ui.heading("IP/Subnetting calculator");

			ui.horizontal(|ui| {
				let ip = ui.label("IP address: ");
				ui.text_edit_singleline(&mut ip_box).labelled_by(ip.id);
				let subnet = ui.label("Subnet mask: ");
				ui.text_edit_singleline(&mut subnet_length_box).labelled_by(subnet.id);
			});

			
			if ui.button("Calc").clicked() {
				
				// let network_info = calc::get_network(&ip_box, &subnet_length_box);

				let ip_b_ip: Ipv4Addr = ip_box.parse().unwrap();
				let sn_l_b_l: u8 = subnet_length_box.parse().unwrap();


				let network_obj = calc::Network {
					network_address: ip_b_ip,
					mask_length: sn_l_b_l,
				};

				let network_info = network_obj.info();
				network = network_info.0;
				subnet = network_info.1;
				wildcard_bits = network_info.2;
				first_host = network_info.3;
				last_host = network_info.4;
				broadcast_address = network_info.5;
				hosts = network_info.6;
				class = network_info.7;

				
			}
			
			ui.label(format!("Network: {network} ({class})"));
			ui.label(format!("Subnet: {subnet}"));
			ui.label(format!("Wildcard: {wildcard_bits}"));
			ui.label(format!("First host: {first_host}"));
			ui.label(format!("Last host: {last_host}"));
			ui.label(format!("Broadcast: {broadcast_address}"));
			ui.label(format!("Hosts: {hosts}"));
			
			// ui.horizontal(|ui| {
			//     let name_label = ui.label("Your name: ");
			//     ui.text_edit_singleline(&mut name)
			//         .labelled_by(name_label.id);
			// });
			// ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
			// if ui.button("Increment").clicked() {
			//     age += 1;
			// }
			// ui.label(format!("Hello '{name}', age {age}"));
		});
	})
}

#[cfg(test)]
mod tests {
	use crate::calc::Network;
	use super::*;

	#[test]
	fn test_splitting() {
		let main_ip: Ipv4Addr = "192.168.1.0".parse().unwrap();
		let expected_second_network: Ipv4Addr = "192.168.1.128".parse().unwrap();

		let main = Network {
			network_address: main_ip,
			mask_length: 24
		};

		let splits = main.split();

		assert_eq!(splits.0.network_address, main_ip);
		assert_eq!(splits.0.mask_length, 25);

		assert_eq!(splits.1.network_address, expected_second_network);
		assert_eq!(splits.1.mask_length, 25);

	}

	#[test]
	fn test_multi_splitting() {
		let main_ip: Ipv4Addr = "192.168.1.0".parse().unwrap();
		let expected_second_network: Ipv4Addr = "192.168.1.128".parse().unwrap();
		let expected_third_network: Ipv4Addr = "192.168.1.192".parse().unwrap();

		let main = Network {
			network_address: main_ip,
			mask_length: 24
		};

		let splits = main.split();
		let second_splits = splits.1.split();

		splits.0.info();
		splits.1.info();

		second_splits.0.info();
		second_splits.1.info();

		assert_eq!(splits.0.network_address, main_ip);
		assert_eq!(splits.0.mask_length, 25);

		assert_eq!(splits.1.network_address, expected_second_network);
		assert_eq!(splits.1.mask_length, 25);

		assert_eq!(second_splits.0.network_address, expected_second_network);
		assert_eq!(second_splits.0.mask_length, 26);

		assert_eq!(second_splits.1.network_address, expected_third_network);
		assert_eq!(second_splits.1.mask_length, 26);

	}
}