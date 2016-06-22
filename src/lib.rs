// automatically generated by rust-bindgen

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

pub enum wkhtmltopdf_global_settings { }
pub enum wkhtmltopdf_object_settings { }
pub enum wkhtmltopdf_converter { }
pub type wkhtmltopdf_str_callback =
    ::std::option::Option<unsafe extern "C" fn(converter: *mut wkhtmltopdf_converter,
                                                 str: *const ::std::os::raw::c_char)>;
pub type wkhtmltopdf_int_callback =
    ::std::option::Option<unsafe extern "C" fn(converter: *mut wkhtmltopdf_converter,
                                                 val: ::std::os::raw::c_int)>;
pub type wkhtmltopdf_void_callback =
    ::std::option::Option<unsafe extern "C" fn(converter: *mut wkhtmltopdf_converter)>;


#[link(name = "libwkhtmltox", kind = "dylib")]
extern "C" {
    pub fn wkhtmltopdf_init(use_graphics: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn wkhtmltopdf_deinit() -> ::std::os::raw::c_int;
    pub fn wkhtmltopdf_extended_qt() -> ::std::os::raw::c_int;
    pub fn wkhtmltopdf_version() -> *const ::std::os::raw::c_char;
    pub fn wkhtmltopdf_create_global_settings() -> *mut wkhtmltopdf_global_settings;
    pub fn wkhtmltopdf_destroy_global_settings(arg1: *mut wkhtmltopdf_global_settings);
    pub fn wkhtmltopdf_create_object_settings() -> *mut wkhtmltopdf_object_settings;
    pub fn wkhtmltopdf_destroy_object_settings(arg1: *mut wkhtmltopdf_object_settings);
    pub fn wkhtmltopdf_set_global_setting(settings: *mut wkhtmltopdf_global_settings,
                                          name: *const ::std::os::raw::c_char,
                                          value: *const ::std::os::raw::c_char)
                                          -> ::std::os::raw::c_int;
    pub fn wkhtmltopdf_get_global_setting(settings: *mut wkhtmltopdf_global_settings,
                                          name: *const ::std::os::raw::c_char,
                                          value: *mut ::std::os::raw::c_char,
                                          vs: ::std::os::raw::c_int)
                                          -> ::std::os::raw::c_int;
    pub fn wkhtmltopdf_set_object_setting(settings: *mut wkhtmltopdf_object_settings,
                                          name: *const ::std::os::raw::c_char,
                                          value: *const ::std::os::raw::c_char)
                                          -> ::std::os::raw::c_int;
    pub fn wkhtmltopdf_get_object_setting(settings: *mut wkhtmltopdf_object_settings,
                                          name: *const ::std::os::raw::c_char,
                                          value: *mut ::std::os::raw::c_char,
                                          vs: ::std::os::raw::c_int)
                                          -> ::std::os::raw::c_int;
    pub fn wkhtmltopdf_create_converter(settings: *mut wkhtmltopdf_global_settings)
                                        -> *mut wkhtmltopdf_converter;
    pub fn wkhtmltopdf_destroy_converter(converter: *mut wkhtmltopdf_converter);
    pub fn wkhtmltopdf_set_warning_callback(converter: *mut wkhtmltopdf_converter,
                                            cb: wkhtmltopdf_str_callback);
    pub fn wkhtmltopdf_set_error_callback(converter: *mut wkhtmltopdf_converter,
                                          cb: wkhtmltopdf_str_callback);
    pub fn wkhtmltopdf_set_phase_changed_callback(converter: *mut wkhtmltopdf_converter,
                                                  cb: wkhtmltopdf_void_callback);
    pub fn wkhtmltopdf_set_progress_changed_callback(converter: *mut wkhtmltopdf_converter,
                                                     cb: wkhtmltopdf_int_callback);
    pub fn wkhtmltopdf_set_finished_callback(converter: *mut wkhtmltopdf_converter,
                                             cb: wkhtmltopdf_int_callback);
    pub fn wkhtmltopdf_convert(converter: *mut wkhtmltopdf_converter) -> ::std::os::raw::c_int;
    pub fn wkhtmltopdf_add_object(converter: *mut wkhtmltopdf_converter,
                                  setting: *mut wkhtmltopdf_object_settings,
                                  data: *const ::std::os::raw::c_char);
    pub fn wkhtmltopdf_current_phase(converter: *mut wkhtmltopdf_converter) -> ::std::os::raw::c_int;
    pub fn wkhtmltopdf_phase_count(converter: *mut wkhtmltopdf_converter) -> ::std::os::raw::c_int;
    pub fn wkhtmltopdf_phase_description(converter: *mut wkhtmltopdf_converter,
                                         phase: ::std::os::raw::c_int)
                                         -> *const ::std::os::raw::c_char;
    pub fn wkhtmltopdf_progress_string(converter: *mut wkhtmltopdf_converter)
                                       -> *const ::std::os::raw::c_char;
    pub fn wkhtmltopdf_http_error_code(converter: *mut wkhtmltopdf_converter)
                                       -> ::std::os::raw::c_int;
    pub fn wkhtmltopdf_get_output(converter: *mut wkhtmltopdf_converter,
                                  arg1: *mut *const ::std::os::raw::c_uchar)
                                  -> ::std::os::raw::c_long;
}