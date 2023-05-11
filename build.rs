fn main() {
    let _logo = r#"

         _______ _                 ____                     _ _                        
        |__   __(_)              / ____|                   | | |
           | |   _ _ __  _   _  | (___  _   _ ___  ___ __ _| | |
           | |  | | '_ \| | | |  \___ \| | | / __|/ __/ _` | | |
           | |  | | | | | |_| |  ____) | |_| \__ \ (_| (_| | | |
           |_|  |_|_| |_|\__, | |_____/ \__, |___/\___\__,_|_|_|
                          __/ |          __/ |                  
                         |___/          |___/                   
                                                                
        More details: https://github.com/Sunrisepeak/tinysyscall
                                                                
    
    "#;

    println!("cargo:rerun-if-changed=build.rs");
}