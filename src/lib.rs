use std::thread;
use std::time::Duration;
//use std::i64;
//use regex::Regex;
#[repr(C)]
/// struct of bit_settings
pub struct bit_settings {
    pub width: cty::c_int,  // width of window
    pub height: cty::c_int, // height of window
    pub audio: bool,        // do we play audio
    pub modded: bool,       // sets game modified, this is set after the function is run
    pub silent: bool,       // dont show modded text durring gameplay
}
//*mut bit_settings settingsTemp;
/// this is the patching function
#[no_mangle]
pub extern "C" fn bittenEnginePatch(settings: *mut bit_settings) {
    // this is unsafe, but it had to be done
    let pointer = format!("{:p}",settings);
	unsafe {
        thread::sleep(Duration::from_millis(10)); // sleep 10 millis
	    (*settings).width=900; // show that it worked
        (*settings).silent=false; // get rid of modded text
        (*settings).modded=false; // set modded to false
        test(pointer);
	}
    let text=format!("Bitten engine patching worked.\nvalue of settings.width {}", unsafe{(*settings).width});
    println!("{}", text);
    return;
}

unsafe fn test(mut settings: String) {
    //(*settings).audio=true;
    println!("hmm did we pass the string to test, lets see {}", pointer);
    println!("if that worked, lets see what else this baby can do");
//    parse(settings);
    //let settings: *mut bit_settings = Ok(<i32>::from_str_radix(pointer.strip_prefix("0x").unwrap(), 16)) as *mut bit_settings;
    //(*settings).audio=true;
}