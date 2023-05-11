use tinysyscall as sal;

fn thread_func(arg: usize) {
    println!("Hello, world from new thread! arg = {}", arg);
}

#[test]
fn create_task() {
    let stack: &mut [u8; 4096] = &mut [0; 4096];
    let tid = sal::task::create(move || { thread_func(2) }, stack);
    assert!(tid > 0);
    sal::time::sleep(1);
}