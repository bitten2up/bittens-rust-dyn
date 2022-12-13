use std::thread;
use std::time::Duration;

#[repr(C)]
/// struct of bit_settings
pub struct bit_settings {
    pub width: cty::c_int,  // width of window
    pub height: cty::c_int, // height of window
    pub audio: bool,        // do we play audio
    pub modded: bool,       // sets game modified, this is set after the function is run
    pub silent: bool,       // dont show modded text durring gameplay
}
/// this is the patching function
#[no_mangle]
pub extern "C" fn bittenEnginePatch(settings: *mut bit_settings) {
	unsafe {
        thread::sleep(Duration::from_millis(10)); // sleep 10 millis
	    (*settings).width=900; // show that it worked
        (*settings).silent=false; // get rid of modded text
        (*settings).modded=false; // set modded to false
	}
    let text=format!("Bitten engine patching worked.\nvalue of settings.width {}", unsafe{(*settings).width});
    println!("{}", text);
    return;
}