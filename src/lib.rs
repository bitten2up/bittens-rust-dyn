#[repr(C)]
pub struct bit_settings {
    pub width: cty::c_int,
    pub height: cty::c_int,
    pub audio: bool,
    pub modded: bool,
    pub silent: bool, // dont show modded text durring gameplay
}
#[no_mangle]
pub extern "C" fn bittenEnginePatch(settings: *mut bit_settings) -> cty::c_int {
	unsafe {
	(*settings).width=900;
	}
	println!("Bitten engine patching worked");
	return 1;
}
