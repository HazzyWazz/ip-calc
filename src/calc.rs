use std::net::{Ipv4Addr};
#[derive(Copy, Clone)]
pub(crate) struct Network {
    pub(crate) network_address: Ipv4Addr,
    pub(crate) mask_length: u8,
}

pub(crate) struct NetworkInfo {
    pub(crate) network_address: Ipv4Addr,
    pub(crate) mask_length: u8,
    pub(crate) subnet_mask: Ipv4Addr,
    pub(crate) wildcard_mask: Ipv4Addr,
    pub(crate) first_host: Ipv4Addr,
    pub(crate) last_host: Ipv4Addr,
    pub(crate) broadcast: Ipv4Addr,
    pub(crate) hosts: u32,
    pub(crate) class: String,
}

impl Network {
    /// Returns all available information on the Network, including, in order:
    /// the network address, the network subnet, the wildcard address, the first host,
    /// the last host, the broadcast address, the number of available hosts and
    /// the class of the Network
    /// ```
    /// let main_network_ip: Ipv4Addr = "172.16.0.0".parse().unwrap();
    /// let main_network_mask_length: u8 = 16;
    ///
    /// let main_network = Network {
    ///     network_address: main_network_ip,
    ///     mask_length: main_network_mask_length,
    /// }
    ///
    /// let network_info = main_network.info()
    /// println!("Network: {} ({})", network_info.network_address, network_info.class);
    /// println!("Subnet: {}", network_info.subnet_mask);
    /// println!("Wildcard: {}", network_info.wildcard_mask);
    /// println!("First host:first_host}", network_info.first_host);
    /// println!("Last host:last_host}", network_info.last_host);
    /// println!("Broadcast: {}" network_info.broadcast);
    /// println!("Hosts: {}", network_info.hosts);
    /// ```
    

    pub(crate) fn info(&self) -> NetworkInfo {
        NetworkInfo {
            network_address: self.network_address,
            mask_length: self.mask_length,
            subnet_mask: self.subnet(),
            wildcard_mask: self.wildcard(),
            first_host: self.first_host(),
            last_host: self.last_host(),
            broadcast: self.broadcast(),
            hosts: self.available_hosts(),
            class: self.class(),
        }
    }

    /// Calculates the subnet address of the [Network]
    pub(crate) fn subnet(&self) -> Ipv4Addr {
        Ipv4Addr::from(u32::MAX << (32 - self.mask_length))
    }

    /// Calculates the wildcard address of the [Network]
    pub(crate) fn wildcard(&self) -> Ipv4Addr {
        Ipv4Addr::from(u32::MAX ^ (u32::MAX << (32 - self.mask_length)))
    }

    /// Calculates the number of available host addresses of the [Network]
    pub(crate) fn available_hosts(&self) -> u32 {
        2_u32.pow(32 - self.mask_length as u32) - 2
    }

    /// Calculates the first host of the [Network]
    pub(crate) fn first_host(&self) -> Ipv4Addr {
        Ipv4Addr::from(self.network_address.to_bits() + 1)
    }

    /// Calculates the last host of the [Network]
    pub(crate) fn last_host(&self) -> Ipv4Addr {
        Ipv4Addr::from(self.network_address.to_bits() + self.wildcard().to_bits() - 1)
    }

    /// Calculates the broadcast address of the [Network]
    pub(crate) fn broadcast(&self) -> Ipv4Addr {
        Ipv4Addr::from(self.last_host().to_bits() + 1)
    }

    /// Determines the class of the [Network].
    /// See [range_check] and [get_class] for more information
    pub(crate) fn class(&self) -> String {
        get_class(self.network_address)
    }

    /// Splits the [Network] and returns a two [Network] tuple with the correct network addresses
    /// and mask lengths.
    pub(crate) fn split(&self) -> (Network, Network) {
        let new_len: u8 = self.mask_length + 1;
        let first_half = Network {
            network_address: self.network_address,
            mask_length: new_len,
        };

        let other_half = Network {
            network_address: Ipv4Addr::from(first_half.broadcast().to_bits() + 1),
            mask_length: new_len,
        };

        (first_half, other_half)
    }
}

/// Helper function for [get_class].
/// Determines the class of the provided IPv4 address.
/// See classful networking for more information.
fn range_check(ip: Ipv4Addr) -> i8 {
    let class = -1_i8;
    let ipn = u32::from(ip);

    // Loopback
    if ip.is_loopback() { return 0 };

    // Broadcast
    if ip.is_broadcast() {return 127 };

    // Class A private
    let class_a_private_start = u32::from(Ipv4Addr::new(10, 0, 0, 0));
    let class_a_private_end = u32::from(Ipv4Addr::new(10, 255, 255, 255));
    if class_a_private_start <= ipn && ipn <= class_a_private_end { return 10 };

    // Class A shared address space
    let class_a_shared_start = u32::from(Ipv4Addr::new(100, 64, 0, 0));
    let class_a_shared_end = u32::from(Ipv4Addr::new(100, 127, 255, 255));
    if class_a_shared_start <= ipn && ipn <= class_a_shared_end { return 11 };

    // Class A
    let class_a_start = u32::from(Ipv4Addr::new(0, 0, 0, 0));
    let class_a_end = u32::from(Ipv4Addr::new(127, 255, 255, 255));
    if class_a_start <= ipn && ipn <= class_a_end { return 1 };

    // Class B Link-local
    let class_b_link_local_start = u32::from(Ipv4Addr::new(169, 254, 0, 0));
    let class_b_link_local_end = u32::from(Ipv4Addr::new(169, 254, 255, 255));
    if class_b_link_local_start <= ipn && ipn <= class_b_link_local_end { return 21 };
    
    // Class B Private
    let class_b_private_start = u32::from(Ipv4Addr::new(172, 16, 0, 0));
    let class_b_private_end = u32::from(Ipv4Addr::new(172, 31, 255, 255));
    if class_b_private_start <= ipn && ipn <= class_b_private_end { return 20 };

    // Class B
    let class_b_start = u32::from(Ipv4Addr::new(128, 0, 0, 0));
    let class_b_end = u32::from(Ipv4Addr::new(191, 255, 255, 255));
    if class_b_start <= ipn && ipn <= class_b_end { return 2 };

    // Class C Private
    let class_c_private_start = u32::from(Ipv4Addr::new(192, 168, 0, 0));
    let class_c_private_end = u32::from(Ipv4Addr::new(192, 168, 255, 255));
    if class_c_private_start <= ipn && ipn <= class_c_private_end { return 30 };
    

    // Class C
    let class_c_start = u32::from(Ipv4Addr::new(192, 0, 0, 0));
    let class_c_end = u32::from(Ipv4Addr::new(223, 255, 255, 255));
    if class_c_start <= ipn && ipn <= class_c_end { return 3 };

    // Class D (multicast)
    if ip.is_multicast() { return 4 };

    // Class E
    let class_e_start = u32::from(Ipv4Addr::new(240, 0, 0, 0));
    let class_e_end = u32::from(Ipv4Addr::new(255, 255, 255, 254));
    if class_e_start <= ipn && ipn <= class_e_end { return 5 };

    class
}

/// Determines the class of the given IPv4 address using range_check()
fn get_class(ip: Ipv4Addr) -> String {
    let mut class = "";

    match range_check(ip) {
        0 => { class = "Class A | Loopback addresses" },
        1 => { class = "Class A" },
        10 => { class = "Class A | Private addresses" },
        11 => { class = "Class A | Shared addresses" },
        2 => { class = "Class B" },
        20 => { class = "Class B | Private addresses" },
        21 => { class = "Class B | Link-local addresses" },
        3 => { class = "Class C" },
        30 => { class = "Class C | Private addresses" },
        4 => { class = "Class D | Multicast addresses" },
        5 => { class = "Class E | Reserved addresses" },
        127 => { class = "Broadcast Address" },
        _ => {},
    }

    String::from(class)
}

pub(crate) fn find_network(ip: Ipv4Addr, subnet: u8) -> Ipv4Addr {
    let subnet_mask = Ipv4Addr::from(u32::MAX << (32 - subnet));
    let network_address = ip & subnet_mask;
    network_address
}

