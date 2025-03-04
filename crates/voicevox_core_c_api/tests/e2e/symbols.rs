use std::ffi::{c_char, c_int, c_void};

use libloading::{Library, Symbol};
use strum::EnumIter;

/// voicevox\_core\_c\_apiのcdylibのシンボルを集めたもの。
#[allow(dead_code)] // TODO: WIP
pub(crate) struct Symbols<'lib> {
    pub(crate) voicevox_open_jtalk_rc_new: Symbol<
        'lib,
        unsafe extern "C" fn(*const c_char, *mut *mut OpenJtalkRc) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_open_jtalk_rc_use_user_dict: Symbol<
        'lib,
        unsafe extern "C" fn(*mut OpenJtalkRc, *const VoicevoxUserDict) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_open_jtalk_rc_delete: Symbol<'lib, unsafe extern "C" fn(*mut OpenJtalkRc)>,
    pub(crate) voicevox_make_default_initialize_options:
        Symbol<'lib, unsafe extern "C" fn() -> VoicevoxInitializeOptions>,
    pub(crate) voicevox_get_version: Symbol<'lib, unsafe extern "C" fn() -> *const c_char>,
    pub(crate) voicevox_voice_model_new_from_path: Symbol<
        'lib,
        unsafe extern "C" fn(*const c_char, *mut *mut VoicevoxVoiceModel) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_voice_model_id:
        Symbol<'lib, unsafe extern "C" fn(*const VoicevoxVoiceModel) -> VoicevoxVoiceModelId>,
    pub(crate) voicevox_voice_model_get_metas_json:
        Symbol<'lib, unsafe extern "C" fn(*const VoicevoxVoiceModel) -> *const c_char>,
    pub(crate) voicevox_voice_model_delete:
        Symbol<'lib, unsafe extern "C" fn(*mut VoicevoxVoiceModel)>,
    pub(crate) voicevox_synthesizer_new_with_initialize: Symbol<
        'lib,
        unsafe extern "C" fn(
            *const OpenJtalkRc,
            VoicevoxInitializeOptions,
            *mut *mut VoicevoxSynthesizer,
        ) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_synthesizer_delete:
        Symbol<'lib, unsafe extern "C" fn(*mut VoicevoxSynthesizer)>,
    pub(crate) voicevox_synthesizer_load_voice_model: Symbol<
        'lib,
        unsafe extern "C" fn(
            *mut VoicevoxSynthesizer,
            *const VoicevoxVoiceModel,
        ) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_synthesizer_unload_voice_model: Symbol<
        'lib,
        unsafe extern "C" fn(*mut VoicevoxSynthesizer, VoicevoxVoiceModelId) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_synthesizer_is_gpu_mode:
        Symbol<'lib, unsafe extern "C" fn(*const VoicevoxSynthesizer) -> bool>,
    pub(crate) voicevox_synthesizer_is_loaded_voice_model: Symbol<
        'lib,
        unsafe extern "C" fn(*const VoicevoxSynthesizer, VoicevoxVoiceModelId) -> bool,
    >,
    pub(crate) voicevox_synthesizer_create_metas_json:
        Symbol<'lib, unsafe extern "C" fn(*const VoicevoxSynthesizer) -> *mut c_char>,
    pub(crate) voicevox_create_supported_devices_json:
        Symbol<'lib, unsafe extern "C" fn(*mut *mut c_char) -> VoicevoxResultCode>,
    pub(crate) voicevox_synthesizer_create_audio_query_from_kana: Symbol<
        'lib,
        unsafe extern "C" fn(
            *const VoicevoxSynthesizer,
            *const c_char,
            VoicevoxStyleId,
            *mut *mut c_char,
        ) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_synthesizer_create_audio_query: Symbol<
        'lib,
        unsafe extern "C" fn(
            *const VoicevoxSynthesizer,
            *const c_char,
            VoicevoxStyleId,
            *mut *mut c_char,
        ) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_make_default_synthesis_options:
        Symbol<'lib, unsafe extern "C" fn() -> VoicevoxSynthesisOptions>,
    pub(crate) voicevox_synthesizer_synthesis: Symbol<
        'lib,
        unsafe extern "C" fn(
            *const VoicevoxSynthesizer,
            *const c_char,
            VoicevoxStyleId,
            VoicevoxSynthesisOptions,
            *mut usize,
            *mut *mut u8,
        ) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_make_default_tts_options:
        Symbol<'lib, unsafe extern "C" fn() -> VoicevoxTtsOptions>,
    pub(crate) voicevox_synthesizer_tts_from_kana: Symbol<
        'lib,
        unsafe extern "C" fn(
            *const VoicevoxSynthesizer,
            *const c_char,
            VoicevoxStyleId,
            VoicevoxTtsOptions,
            *mut usize,
            *mut *mut u8,
        ) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_synthesizer_tts: Symbol<
        'lib,
        unsafe extern "C" fn(
            *const VoicevoxSynthesizer,
            *const c_char,
            VoicevoxStyleId,
            VoicevoxTtsOptions,
            *mut usize,
            *mut *mut u8,
        ) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_json_free: Symbol<'lib, unsafe extern "C" fn(*mut c_char)>,
    pub(crate) voicevox_wav_free: Symbol<'lib, unsafe extern "C" fn(*mut u8)>,
    pub(crate) voicevox_error_result_to_message:
        Symbol<'lib, unsafe extern "C" fn(VoicevoxResultCode) -> *const c_char>,

    pub(crate) initialize: Symbol<'lib, unsafe extern "C" fn(bool, c_int, bool) -> bool>,
    pub(crate) load_model: Symbol<'lib, unsafe extern "C" fn(i64) -> bool>,
    pub(crate) is_model_loaded: Symbol<'lib, unsafe extern "C" fn(i64) -> bool>,
    pub(crate) finalize: Symbol<'lib, unsafe extern "C" fn()>,
    pub(crate) metas: Symbol<'lib, unsafe extern "C" fn() -> *const c_char>,
    pub(crate) last_error_message: Symbol<'lib, unsafe extern "C" fn() -> *const c_char>,
    pub(crate) supported_devices: Symbol<'lib, unsafe extern "C" fn() -> *const c_char>,
    pub(crate) yukarin_s_forward:
        Symbol<'lib, unsafe extern "C" fn(i64, *mut i64, *mut i64, *mut f32) -> bool>,
    pub(crate) yukarin_sa_forward: Symbol<
        'lib,
        unsafe extern "C" fn(
            i64,
            *mut i64,
            *mut i64,
            *mut i64,
            *mut i64,
            *mut i64,
            *mut i64,
            *mut i64,
            *mut f32,
        ) -> bool,
    >,
    pub(crate) decode_forward: Symbol<
        'lib,
        unsafe extern "C" fn(i64, i64, *mut f32, *mut f32, *mut i64, *mut f32) -> bool,
    >,

    pub(crate) voicevox_user_dict_word_make:
        Symbol<'lib, unsafe extern "C" fn(*const c_char, *const c_char) -> VoicevoxUserDictWord>,
    pub(crate) voicevox_user_dict_new:
        Symbol<'lib, unsafe extern "C" fn() -> *mut VoicevoxUserDict>,
    pub(crate) voicevox_user_dict_load: Symbol<
        'lib,
        unsafe extern "C" fn(*const VoicevoxUserDict, *const c_char) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_user_dict_add_word: Symbol<
        'lib,
        unsafe extern "C" fn(
            *const VoicevoxUserDict,
            *const VoicevoxUserDictWord,
            *mut [u8; 16],
        ) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_user_dict_update_word: Symbol<
        'lib,
        unsafe extern "C" fn(
            *const VoicevoxUserDict,
            *const [u8; 16],
            *const VoicevoxUserDictWord,
        ) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_user_dict_remove_word: Symbol<
        'lib,
        unsafe extern "C" fn(*const VoicevoxUserDict, *const [u8; 16]) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_user_dict_to_json: Symbol<
        'lib,
        unsafe extern "C" fn(*const VoicevoxUserDict, *mut *mut c_char) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_user_dict_import: Symbol<
        'lib,
        unsafe extern "C" fn(
            *const VoicevoxUserDict,
            *const VoicevoxUserDict,
        ) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_user_dict_save: Symbol<
        'lib,
        unsafe extern "C" fn(*const VoicevoxUserDict, *const c_char) -> VoicevoxResultCode,
    >,
    pub(crate) voicevox_user_dict_delete:
        Symbol<'lib, unsafe extern "C" fn(*mut VoicevoxUserDict) -> VoicevoxResultCode>,
}

impl<'lib> Symbols<'lib> {
    pub(crate) unsafe fn new(lib: &'lib Library) -> Result<Self, libloading::Error> {
        macro_rules! new(($($name:ident),* $(,)?) => {
            Self {
                $(
                    $name: lib.get(stringify!($name).as_ref())?,
                )*
            }
        });

        Ok(new!(
            voicevox_open_jtalk_rc_new,
            voicevox_open_jtalk_rc_use_user_dict,
            voicevox_open_jtalk_rc_delete,
            voicevox_make_default_initialize_options,
            voicevox_get_version,
            voicevox_voice_model_new_from_path,
            voicevox_voice_model_id,
            voicevox_voice_model_get_metas_json,
            voicevox_voice_model_delete,
            voicevox_synthesizer_new_with_initialize,
            voicevox_synthesizer_delete,
            voicevox_synthesizer_load_voice_model,
            voicevox_synthesizer_unload_voice_model,
            voicevox_synthesizer_is_gpu_mode,
            voicevox_synthesizer_is_loaded_voice_model,
            voicevox_synthesizer_create_metas_json,
            voicevox_create_supported_devices_json,
            voicevox_synthesizer_create_audio_query_from_kana,
            voicevox_synthesizer_create_audio_query,
            voicevox_make_default_synthesis_options,
            voicevox_synthesizer_synthesis,
            voicevox_make_default_tts_options,
            voicevox_synthesizer_tts_from_kana,
            voicevox_synthesizer_tts,
            voicevox_json_free,
            voicevox_wav_free,
            voicevox_error_result_to_message,
            initialize,
            load_model,
            is_model_loaded,
            finalize,
            metas,
            last_error_message,
            supported_devices,
            yukarin_s_forward,
            yukarin_sa_forward,
            decode_forward,
            voicevox_user_dict_word_make,
            voicevox_user_dict_new,
            voicevox_user_dict_load,
            voicevox_user_dict_add_word,
            voicevox_user_dict_update_word,
            voicevox_user_dict_remove_word,
            voicevox_user_dict_to_json,
            voicevox_user_dict_import,
            voicevox_user_dict_save,
            voicevox_user_dict_delete,
        ))
    }
}

type OpenJtalkRc = c_void;
type VoicevoxVoiceModel = c_void;
type VoicevoxVoiceModelId = *const c_char;
type VoicevoxSynthesizer = c_void;
type VoicevoxStyleId = u32;

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
#[allow(non_camel_case_types)]
pub(crate) enum VoicevoxResultCode {
    VOICEVOX_RESULT_OK = 0,
    VOICEVOX_RESULT_NOT_LOADED_OPENJTALK_DICT_ERROR = 1,
    VOICEVOX_RESULT_GET_SUPPORTED_DEVICES_ERROR = 3,
    VOICEVOX_RESULT_GPU_SUPPORT_ERROR = 4,
    VOICEVOX_RESULT_STYLE_NOT_FOUND_ERROR = 6,
    VOICEVOX_RESULT_MODEL_NOT_FOUND_ERROR = 7,
    VOICEVOX_RESULT_INFERENCE_ERROR = 8,
    VOICEVOX_RESULT_EXTRACT_FULL_CONTEXT_LABEL_ERROR = 11,
    VOICEVOX_RESULT_INVALID_UTF8_INPUT_ERROR = 12,
    VOICEVOX_RESULT_PARSE_KANA_ERROR = 13,
    VOICEVOX_RESULT_INVALID_AUDIO_QUERY_ERROR = 14,
    VOICEVOX_RESULT_INVALID_ACCENT_PHRASE_ERROR = 15,
    VOICEVOX_RESULT_OPEN_ZIP_FILE_ERROR = 16,
    VOICEVOX_RESULT_READ_ZIP_ENTRY_ERROR = 17,
    VOICEVOX_RESULT_MODEL_ALREADY_LOADED_ERROR = 18,
    VOICEVOX_RESULT_STYLE_ALREADY_LOADED_ERROR = 26,
    VOICEVOX_RESULT_INVALID_MODEL_DATA_ERROR = 27,
    VOICEVOX_RESULT_LOAD_USER_DICT_ERROR = 20,
    VOICEVOX_RESULT_SAVE_USER_DICT_ERROR = 21,
    VOICEVOX_RESULT_USER_DICT_WORD_NOT_FOUND_ERROR = 22,
    VOICEVOX_RESULT_USE_USER_DICT_ERROR = 23,
    VOICEVOX_RESULT_INVALID_USER_DICT_WORD_ERROR = 24,
    VOICEVOX_RESULT_INVALID_UUID_ERROR = 25,
}

#[repr(i32)]
#[allow(non_camel_case_types)]
pub(crate) enum VoicevoxAccelerationMode {
    VOICEVOX_ACCELERATION_MODE_CPU = 1,
}

#[repr(C)]
pub(crate) struct VoicevoxInitializeOptions {
    pub(crate) acceleration_mode: VoicevoxAccelerationMode,
    pub(crate) _cpu_num_threads: u16,
}

#[repr(C)]
pub(crate) struct VoicevoxSynthesisOptions {
    _enable_interrogative_upspeak: bool,
}

#[repr(C)]
pub(crate) struct VoicevoxTtsOptions {
    _enable_interrogative_upspeak: bool,
}

#[repr(C)]
pub(crate) struct VoicevoxUserDict {
    _private: [u8; 0],
}

#[repr(C)]
pub(crate) struct VoicevoxUserDictWord {
    pub(crate) surface: *const c_char,
    pub(crate) pronunciation: *const c_char,
    pub(crate) accent_type: usize,
    pub(crate) word_type: VoicevoxUserDictWordType,
    pub(crate) priority: u32,
}

#[repr(i32)]
#[allow(non_camel_case_types)]
pub(crate) enum VoicevoxUserDictWordType {
    VOICEVOX_USER_DICT_WORD_TYPE_PROPER_NOUN = 0,
}
