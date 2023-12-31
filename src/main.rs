fn main() {
    // list all of the devices pcap tells us are available
    for device in pcap::Device::list().expect("device lookup failed") {
        println!("Found device! {:?}", device);
        
        let Ok(cap) = pcap::Capture::from_device(device) else {
            println!("    Error creating capture");
            continue;
        };
        let Ok(cap) = cap.immediate_mode(true).open() else {
            println!("    Error opening capture");
            continue;
        };

        {
            let l = cap.get_datalink();
            println!("  Main data link: {:?} {:?} {:?}", l, l.get_name(), l.get_description())
        }

        if let Ok(links) = cap.list_datalinks() {
            for l in links {
                println!("    {:?} {:?} {:?}", l, l.get_name(), l.get_description());
            }
        }
    }
}
