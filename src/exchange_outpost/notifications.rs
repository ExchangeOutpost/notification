use extism_pdk::host_fn;


#[host_fn]
extern "ExtismHost" {
    fn add_notification(notification: String); 
}

pub fn send_notification(notification: String) {
    unsafe{
        let res = add_notification(notification);
        if res.is_err() {
            eprintln!("Failed to send notification");
        }
    }
}