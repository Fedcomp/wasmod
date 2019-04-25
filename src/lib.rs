#![cfg_attr(feature = "strict", deny(warnings))]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

use cstr_macro::cstr;
use hlsdk_sys::{edict_t, BOOL, DLL_FUNCTIONS, TRUE};
use metamod_bindgen::{enginefuncs_t, gamedll_funcs_t, globalvars_t};
use metamod_sys::{
    meta_globals_t, plugin_info_t, GETENTITYAPI_FN_INTERFACE_VERSION, META_FUNCTIONS,
    META_INTERFACE_VERSION,
    META_RES::*,
    PLUG_LOADTIME::{self, PT_CHANGELEVEL},
};

const PLUGIN_INFO: plugin_info_t = plugin_info_t {
    ifvers: META_INTERFACE_VERSION,
    name: cstr!("MetaRust"),
    version: cstr!("0.0.1"),
    date: cstr!("23.04.2019"),
    author: cstr!("Fedcomp"),
    url: cstr!("http://amx-x.ru"),
    logtag: cstr!("METARUST"),
    loadable: PT_CHANGELEVEL,
    unloadable: PT_CHANGELEVEL,
};

const gMetaFunctionTable: META_FUNCTIONS = META_FUNCTIONS {
    pfnGetEntityAPI: None,
    pfnGetEntityAPI_Post: None,
    pfnGetEntityAPI2: Some(get_entity_api2),
    pfnGetEntityAPI2_Post: Some(get_entity_api2_post),
    pfnGetNewDLLFunctions: None,
    pfnGetNewDLLFunctions_Post: None,
    pfnGetEngineFunctions: None,
    pfnGetEngineFunctions_Post: None,
};

const gFunctionTable: DLL_FUNCTIONS = DLL_FUNCTIONS {
    pfnGameInit: None,
    pfnSpawn: None,
    pfnThink: None,
    pfnUse: None,
    pfnTouch: None,
    pfnBlocked: None,
    pfnKeyValue: None,
    pfnSave: None,
    pfnRestore: None,
    pfnSetAbsBox: None,
    pfnSaveWriteFields: None,
    pfnSaveReadFields: None,
    pfnSaveGlobalState: None,
    pfnRestoreGlobalState: None,
    pfnResetGlobalState: None,
    pfnClientConnect: None,
    pfnClientDisconnect: None,
    pfnClientKill: None,
    pfnClientPutInServer: None,
    pfnClientCommand: None,
    pfnClientUserInfoChanged: None,
    pfnServerActivate: None,
    pfnServerDeactivate: None,
    pfnPlayerPreThink: None,
    pfnPlayerPostThink: None,
    pfnStartFrame: None,
    pfnParmsNewLevel: None,
    pfnParmsChangeLevel: None,
    pfnGetGameDescription: None,
    pfnPlayerCustomization: None,
    pfnSpectatorConnect: None,
    pfnSpectatorDisconnect: None,
    pfnSpectatorThink: None,
    pfnSys_Error: None,
    pfnPM_Move: None,
    pfnPM_Init: None,
    pfnPM_FindTextureType: None,
    pfnSetupVisibility: None,
    pfnUpdateClientData: None,
    pfnAddToFullPack: None,
    pfnCreateBaseline: None,
    pfnRegisterEncoders: None,
    pfnGetWeaponData: None,
    pfnCmdStart: None,
    pfnCmdEnd: None,
    pfnConnectionlessPacket: None,
    pfnGetHullBounds: None,
    pfnCreateInstancedBaselines: None,
    pfnInconsistentFile: None,
    pfnAllowLagCompensation: None,
};

const gFunctionTable_Post: DLL_FUNCTIONS = DLL_FUNCTIONS {
    pfnGameInit: None,
    pfnSpawn: None,
    pfnThink: None,
    pfnUse: None,
    pfnTouch: None,
    pfnBlocked: None,
    pfnKeyValue: None,
    pfnSave: None,
    pfnRestore: None,
    pfnSetAbsBox: None,
    pfnSaveWriteFields: None,
    pfnSaveReadFields: None,
    pfnSaveGlobalState: None,
    pfnRestoreGlobalState: None,
    pfnResetGlobalState: None,
    pfnClientConnect: None,
    pfnClientDisconnect: None,
    pfnClientKill: None,
    pfnClientPutInServer: None,
    pfnClientCommand: None,
    pfnClientUserInfoChanged: None,
    pfnServerActivate: Some(server_activate_post),
    pfnServerDeactivate: None,
    pfnPlayerPreThink: None,
    pfnPlayerPostThink: None,
    pfnStartFrame: None,
    pfnParmsNewLevel: None,
    pfnParmsChangeLevel: None,
    pfnGetGameDescription: None,
    pfnPlayerCustomization: None,
    pfnSpectatorConnect: None,
    pfnSpectatorDisconnect: None,
    pfnSpectatorThink: None,
    pfnSys_Error: None,
    pfnPM_Move: None,
    pfnPM_Init: None,
    pfnPM_FindTextureType: None,
    pfnSetupVisibility: None,
    pfnUpdateClientData: None,
    pfnAddToFullPack: None,
    pfnCreateBaseline: None,
    pfnRegisterEncoders: None,
    pfnGetWeaponData: None,
    pfnCmdStart: None,
    pfnCmdEnd: None,
    pfnConnectionlessPacket: None,
    pfnGetHullBounds: None,
    pfnCreateInstancedBaselines: None,
    pfnInconsistentFile: None,
    pfnAllowLagCompensation: None,
};

static mut gpGlobals: Option<&globalvars_t> = None;
static mut gpMetaGlobals: Option<&mut meta_globals_t> = None;

/* Initialization pointer/hook processing functions */

#[no_mangle]
pub unsafe extern "C" fn Meta_Attach(
    _plug_loadtime: PLUG_LOADTIME,
    pFunctionTable: *mut META_FUNCTIONS,
    pMGlobals: *mut meta_globals_t,
    _pGamedllFuncs: *const gamedll_funcs_t,
) -> BOOL {
    *pFunctionTable = gMetaFunctionTable;
    gpMetaGlobals = Some(&mut *pMGlobals);

    TRUE
}

#[no_mangle]
pub extern "C" fn Meta_Detach() -> BOOL {
    TRUE
}

#[no_mangle]
pub unsafe extern "C" fn Meta_Query(
    ifvers: *const c_char,
    pinfo: *mut *const plugin_info_t,
    _mutil_funcs: c_char,
) -> BOOL {
    let _interface_version = CStr::from_ptr(ifvers);
    *pinfo = &PLUGIN_INFO;

    TRUE
}

#[no_mangle]
pub unsafe extern "C" fn GiveFnptrsToDll(
    _pengfuncsFromEngine: *const enginefuncs_t,
    pGlobals: *const globalvars_t,
) {
    gpGlobals = Some(&*pGlobals);
}

pub unsafe extern "C" fn get_entity_api2(
    pFunctionTable: *mut DLL_FUNCTIONS,
    interfaceVersion: *const c_int,
) -> BOOL {
    // TODO: Make fail handling as in metamod plugin example
    if *interfaceVersion != GETENTITYAPI_FN_INTERFACE_VERSION {
        panic!(
            "Inconsistent GETENTITYAPI_FN_INTERFACE_VERSION, theirs: {}, ours: {}",
            *interfaceVersion, GETENTITYAPI_FN_INTERFACE_VERSION
        )
    }

    // Return our hook list to engine
    *pFunctionTable = gFunctionTable;

    TRUE
}

pub unsafe extern "C" fn get_entity_api2_post(
    pFunctionTable: *mut DLL_FUNCTIONS,
    interfaceVersion: *const c_int,
) -> BOOL {
    // TODO: Make fail handling as in metamod plugin example
    if *interfaceVersion != GETENTITYAPI_FN_INTERFACE_VERSION {
        panic!(
            "Inconsistent GETENTITYAPI_FN_INTERFACE_VERSION, theirs: {}, ours: {}",
            *interfaceVersion, GETENTITYAPI_FN_INTERFACE_VERSION
        )
    }

    // Return our hook list to engine
    *pFunctionTable = gFunctionTable_Post;

    TRUE
}

/* Library defined hooks */

// amxmodx's plugin_init
pub unsafe extern "C" fn server_activate_post(
    _pEdictList: *const edict_t,
    _edictCount: i32,
    _clientMax: i32,
) {
    println!("plugin_init()");

    let meta_globals = gpMetaGlobals
        .as_mut()
        .expect("Meta globals should be already initialized to this moment");

    meta_globals.mres = MRES_IGNORED;
}
