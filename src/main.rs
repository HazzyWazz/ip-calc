#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::net::Ipv4Addr;
use std::ops::Index;
use eframe::egui;
use crate::calc::{Network, NetworkInfo};

mod calc;

fn main() -> eframe::Result {

	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 480.0]),
		..Default::default()
	};

	let mut ip_box = "network address".to_owned();
	let mut subnet_length_box = "mask length".to_owned();

	let mut network: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
	let mut class: String = String::new();
	
	let mut subnet: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
	let mut wildcard_mask: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
	let mut first_host: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
	let mut last_host: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
	let mut broadcast_address: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
	let mut hosts: u32 = 0;

	let mut network_list: Vec<calc::Network> = Vec::new();


	eframe::run_ui_native("IP Calc", options, move |ui, _frame| {
		egui::CentralPanel::default().show_inside(ui, |ui| {
			ui.heading("IP/Subnetting calculator");

			ui.horizontal(|ui| {
				let ip = ui.label("IP address: ");
				ui.text_edit_singleline(&mut ip_box).labelled_by(ip.id);
				let subnet = ui.label("Subnet mask: ");
				ui.text_edit_singleline(&mut subnet_length_box).labelled_by(subnet.id);
			});

			ui.horizontal(|ui| {
				
				if ui.button("Calc").clicked() {
	
					network_list = Vec::new();
					
					let ip_b_ip: Ipv4Addr = ip_box.parse().unwrap();
					let sn_l_b_l: u8 = subnet_length_box.parse().unwrap();
	
	
					let network_obj = calc::Network {
						network_address: ip_b_ip,
						mask_length: sn_l_b_l,
					};
	
					let network_info = network_obj.info();
					network = network_info.network_address;
					subnet = network_info.subnet_mask;
					wildcard_mask = network_info.wildcard_mask;
					first_host = network_info.first_host;
					last_host = network_info.last_host;
					broadcast_address = network_info.broadcast;
					hosts = network_info.hosts;
					class = network_info.class;
	
					network_list.push(network_obj);
	
					// These are tests to see if the grid worked as I expected
					/*let class_a_local_space = Network {
						network_address: Ipv4Addr::from_octets([10,0,0,0]),
						mask_length: 8,
					};
					network_list.push(class_a_local_space);
	
					let class_b_local_space = Network {
						network_address: Ipv4Addr::from_octets([172,16,0,0]),
						mask_length: 12,
					};
					network_list.push(class_b_local_space);
	
					let class_c_local_space = Network {
						network_address: Ipv4Addr::from_octets([192,168,0,0]),
						mask_length: 16,
					};
					network_list.push(class_c_local_space);*/
	
				}
			
				if ui.button("Find network").clicked() {
					let ip_query: Ipv4Addr = ip_box.parse().unwrap();
					let subnet_query: u8 = subnet_length_box.parse().unwrap();
					
					let network_ip = calc::find_network(ip_query, subnet_query);
					ip_box = network_ip.to_string();
				}
			});
			
			ui.label(format!("Network: {network} ({class})"));
			ui.label(format!("Subnet: {subnet}"));
			ui.label(format!("Wildcard: {wildcard_mask}"));
			ui.label(format!("First host: {first_host}"));
			ui.label(format!("Last host: {last_host}"));
			ui.label(format!("Broadcast: {broadcast_address}"));
			ui.label(format!("Hosts: {hosts}"));

			ui.separator();

			egui::Grid::new("network_tables").striped(true).show(ui, |ui| {

					ui.label("Network");
					ui.label("Subnet mask");
					ui.label("Wildcard");
					ui.label("Host range");
					ui.label("Broadcast address");
					ui.label("Hosts");
					ui.label("Split");
					// ui.label("Merge");
					ui.end_row();

					for network in network_list.clone() {
						let info = network.info();

						ui.label(info.network_address.to_string());
						ui.label(info.subnet_mask.to_string());
						ui.label(info.wildcard_mask.to_string());
						ui.label(format!("{0} - {1}", info.first_host, info.last_host));
						ui.label(info.broadcast.to_string());
						ui.label(info.hosts.to_string());
						if ui.button("Split").clicked() {
							let split_network_index = network_list.iter()
								.position(|x| x.network_address == network.network_address)
								.unwrap();
							let split_network = network_list[split_network_index].clone().split();
							network_list.remove(split_network_index);
							network_list.push(split_network.0);
							network_list.push(split_network.1);
						}
						// ui.label("Merge");
						ui.end_row();
					}

				// ui.horizontal(|ui| { ui.label("Same"); ui.label("cell"); });
				// ui.label("Third row, second column");
				// ui.end_row();
			});
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