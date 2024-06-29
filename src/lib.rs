#![feature(lazy_cell, ptr_sub_ptr)]

use engage::{
    pad::Pad, proc::procinst_jump, sequence::gmap_sequence::GmapSequence, util::get_instance,
};

use skyline::patching::Patch;

use unity::prelude::*;

static mut MINUS_PRESSED: bool = false;

// App.GmapSequence$$Tick	7102b3e140	void App.GmapSequence$$Tick(App_GmapSequence_o * __this, MethodInfo * method)	3404
#[unity::hook("App", "GmapSequence", "Tick")]
pub fn gmapsequence_tick(this: &GmapSequence, method_info: OptionalMethod) {
    let pad_instance = get_instance::<Pad>();

    // This tick is for GmapSequence, which means it isn't run during the GmapDisposeSequence
    // Thus, when we're in this tick, we can safely reset the MINUS_PRESSED flag
    unsafe { MINUS_PRESSED = false };
    if pad_instance.npad_state.buttons.minus() {
        if !pad_instance.old_buttons.minus() {
            unsafe {
                println!("Minus pressed");
                MINUS_PRESSED = true;
                procinst_jump(this, 5, None);
            }
        }
    }
    call_original!(this, method_info);
}

// App.GameUserGmapData$$IsCheckDispos	710251ca40	bool App.GameUserGmapData$$IsCheckDispos(App_GameUserGmapData_o * __this, MethodInfo * method)	124
#[unity::hook("App", "GameUserGmapData", "IsCheckDispos")]
pub fn gameusergmapdata_is_check_dispos(this: *const u8, method_info: OptionalMethod) -> bool {
    let result = call_original!(this, method_info);
    match unsafe { MINUS_PRESSED } {
        true => {
            println!("Minus pressed, returning true");
            return true;
        }
        false => {
            println!("IsCheckDispos: {}", result);
            return result;
        }
    }
}

// int32_t App.GmapSpotManager$$CalculateDisposCount(App_GmapSpotManager_o *__this,MethodInfo *method)
#[unity::hook("App", "GmapSpotManager", "CalculateDisposCount")]
pub fn gmapspotmanager_calculate_dispos_count(this: *const u8, method_info: OptionalMethod) -> i32 {
    let result: i32 = call_original!(this, method_info);
    match unsafe { MINUS_PRESSED } {
        true => {
            println!("Minus pressed, returning 5");
            return 5;
        }
        false => {
            println!("CalculateDisposCount: {}", result);
            return result;
        }
    }
}

#[skyline::main(name = "hooks")]
pub fn main() {
    skyline::install_hooks!(
        gmapsequence_tick,
        gameusergmapdata_is_check_dispos,
        gmapspotmanager_calculate_dispos_count
    );
    Patch::in_text(0x02B46990)
        .bytes([0x1F, 0x90, 0x01, 0x71])
        .unwrap(); // Allows training skirmishes to be spawned
    println!("{}", format!("Spawn Skirmishes {}", env!("CARGO_PKG_VERSION")))
}
