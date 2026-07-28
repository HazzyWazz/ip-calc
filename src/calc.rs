use std::net::{Ipv4Addr};
pub(crate) struct Network {
    pub(crate) network_address: Ipv4Addr,
    pub(crate) mask_length: u8,
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
    /// println!("Network: {} ({})", network_info.0, network_info.7);
    /// println!("Subnet: {}", network_info.1);
    /// println!("Wildcard: {}", network_info.2);
    /// println!("First host:first_host}", network_info.3);
    /// println!("Last host:last_host}", network_info.4);
    /// println!("Broadcast: {}" network_info.5);
    /// println!("Hosts: {}", network_info.6);
    /// ```
    

    pub(crate) fn info(&self) -> (Ipv4Addr, Ipv4Addr, Ipv4Addr, Ipv4Addr, Ipv4Addr, Ipv4Addr, u32, String) {
        (self.network_address, self.subnet(), self.wildcard(), self.first_host(),
         self.last_host(), self.broadcast(), self.available_hosts(), self.class())
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
    let c = -1_i8;
    let ipn = u32::from(ip);

    // Loopback
    if ip.is_loopback() { return 0 };

    // Broadcast
    if ip.is_broadcast() {return 127 };

    // Class A
    let a_s = u32::from(Ipv4Addr::new(1,0,0,0));
    let a_e = u32::from(Ipv4Addr::new(126,255,255,255));
    if a_s <= ipn && ipn <= a_e { return 1 };

    // Class B
    let b_s = u32::from(Ipv4Addr::new(128,0,0,0));
    let b_e = u32::from(Ipv4Addr::new(191,255,255,255));
    if b_s <= ipn && ipn <= b_e { return 2 };

    // Class C
    let c_s = u32::from(Ipv4Addr::new(192,0,0,0));
    let c_e = u32::from(Ipv4Addr::new(223,255,255,255));
    if c_s <= ipn && ipn <= c_e { return 3 };

    // Class D (multicast)
    if ip.is_multicast() { return 4 };

    // Class E
    let e_s = u32::from(Ipv4Addr::new(240,0,0,0));
    let e_e = u32::from(Ipv4Addr::new(255,255,255,254));
    if e_s <= ipn && ipn <= e_e { return 5 };

    c
}

/// Determines the class of the given IPv4 address using range_check()
fn get_class(ip: Ipv4Addr) -> String {
    let mut class = "";

    match range_check(ip) {
        0 => { class = "Loopback addresses" },
        1 => { class = "Class A" },
        2 => { class = "Class B" },
        3 => { class = "Class C" },
        4 => { class = "Class D | Multicast addresses" },
        5 => { class = "Class E | Reserved addresses" },
        127 => { class = "Broadcast Address" },
        _ => {},
    }

    String::from(class)
}

