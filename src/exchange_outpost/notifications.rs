use extism_pdk::host_fn;


#[host_fn]
extern "ExtismHost" {
    fn add_notification(); 
}

pub fn send_notification() {
    unsafe{
        let res = add_notification();
        if res.is_err() {
            eprintln!("Failed to send notification");
        }
    }
}