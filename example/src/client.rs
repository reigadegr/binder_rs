use crate::IRemoteService::IRemoteService;
use binder::Strong;

pub fn run() -> anyhow::Result<()> {
    let my_service: Strong<dyn IRemoteService> = binder::get_interface("myservice").unwrap();
    println!("Do getPid()");
    let pid = my_service.getPid().expect("Failed to get pid");
    println!("Got pid: {}", pid);
    println!("Do basicTypes()");
    my_service
        .basicTypes(1_i32, 2_i64, false, 1.1_f32, 2.2_f64, "fuckyou!")
        .expect("Failed to call basicTypes");
    println!("Done!");
    Ok(())
}
