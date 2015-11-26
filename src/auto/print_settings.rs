// This file was generated by gir (e8e8262) from gir-files (11e0e6d)
// DO NOT EDIT

use NumberUpLayout;
use PageOrientation;
use PageSet;
use PrintDuplex;
use PrintPages;
use PrintQuality;
use Unit;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct PrintSettings(Object<ffi::GtkPrintSettings>);

    match fn {
        get_type => || ffi::gtk_print_settings_get_type(),
    }
}

impl PrintSettings {
    pub fn new() -> PrintSettings {
        unsafe {
            from_glib_full(ffi::gtk_print_settings_new())
        }
    }

    //pub fn new_from_file(file_name: &str, error: /*Ignored*/Option<glib::Error>) -> PrintSettings {
    //    unsafe { TODO: call ffi::gtk_print_settings_new_from_file() }
    //}

    //pub fn new_from_key_file(key_file: /*Ignored*/&glib::KeyFile, group_name: Option<&str>, error: /*Ignored*/Option<glib::Error>) -> PrintSettings {
    //    unsafe { TODO: call ffi::gtk_print_settings_new_from_key_file() }
    //}

    pub fn copy(&self) -> Option<PrintSettings> {
        unsafe {
            from_glib_full(ffi::gtk_print_settings_copy(self.to_glib_none().0))
        }
    }

    //pub fn foreach(&self, func: /*Unknown conversion*/Unknown rust type: "PrintSettingsFunc", user_data: Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::gtk_print_settings_foreach() }
    //}

    pub fn get(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn get_bool(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_bool(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn get_collate(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_collate(self.to_glib_none().0))
        }
    }

    pub fn get_default_source(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_default_source(self.to_glib_none().0))
        }
    }

    pub fn get_dither(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_dither(self.to_glib_none().0))
        }
    }

    pub fn get_double(&self, key: &str) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_double(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    pub fn get_double_with_default(&self, key: &str, def: f64) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_double_with_default(self.to_glib_none().0, key.to_glib_none().0, def)
        }
    }

    pub fn get_duplex(&self) -> PrintDuplex {
        unsafe {
            ffi::gtk_print_settings_get_duplex(self.to_glib_none().0)
        }
    }

    pub fn get_finishings(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_finishings(self.to_glib_none().0))
        }
    }

    pub fn get_int(&self, key: &str) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_int(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    pub fn get_int_with_default(&self, key: &str, def: i32) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_int_with_default(self.to_glib_none().0, key.to_glib_none().0, def)
        }
    }

    pub fn get_length(&self, key: &str, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_length(self.to_glib_none().0, key.to_glib_none().0, unit)
        }
    }

    pub fn get_media_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_media_type(self.to_glib_none().0))
        }
    }

    pub fn get_n_copies(&self) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_n_copies(self.to_glib_none().0)
        }
    }

    pub fn get_number_up(&self) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_number_up(self.to_glib_none().0)
        }
    }

    pub fn get_number_up_layout(&self) -> NumberUpLayout {
        unsafe {
            ffi::gtk_print_settings_get_number_up_layout(self.to_glib_none().0)
        }
    }

    pub fn get_orientation(&self) -> PageOrientation {
        unsafe {
            ffi::gtk_print_settings_get_orientation(self.to_glib_none().0)
        }
    }

    pub fn get_output_bin(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_output_bin(self.to_glib_none().0))
        }
    }

    //pub fn get_page_ranges(&self, num_ranges: &mut i32) -> /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 1, id: 613 }" {
    //    unsafe { TODO: call ffi::gtk_print_settings_get_page_ranges() }
    //}

    pub fn get_page_set(&self) -> PageSet {
        unsafe {
            ffi::gtk_print_settings_get_page_set(self.to_glib_none().0)
        }
    }

    pub fn get_paper_height(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_paper_height(self.to_glib_none().0, unit)
        }
    }

    //pub fn get_paper_size(&self) -> /*Ignored*/PaperSize {
    //    unsafe { TODO: call ffi::gtk_print_settings_get_paper_size() }
    //}

    pub fn get_paper_width(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_paper_width(self.to_glib_none().0, unit)
        }
    }

    pub fn get_print_pages(&self) -> PrintPages {
        unsafe {
            ffi::gtk_print_settings_get_print_pages(self.to_glib_none().0)
        }
    }

    pub fn get_printer(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_printer(self.to_glib_none().0))
        }
    }

    pub fn get_printer_lpi(&self) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_printer_lpi(self.to_glib_none().0)
        }
    }

    pub fn get_quality(&self) -> PrintQuality {
        unsafe {
            ffi::gtk_print_settings_get_quality(self.to_glib_none().0)
        }
    }

    pub fn get_resolution(&self) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_resolution(self.to_glib_none().0)
        }
    }

    pub fn get_resolution_x(&self) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_resolution_x(self.to_glib_none().0)
        }
    }

    pub fn get_resolution_y(&self) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_resolution_y(self.to_glib_none().0)
        }
    }

    pub fn get_reverse(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_reverse(self.to_glib_none().0))
        }
    }

    pub fn get_scale(&self) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_scale(self.to_glib_none().0)
        }
    }

    pub fn get_use_color(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_use_color(self.to_glib_none().0))
        }
    }

    pub fn has_key(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_settings_has_key(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    //pub fn load_file(&self, file_name: &str, error: /*Ignored*/Option<glib::Error>) -> bool {
    //    unsafe { TODO: call ffi::gtk_print_settings_load_file() }
    //}

    //pub fn load_key_file(&self, key_file: /*Ignored*/&glib::KeyFile, group_name: Option<&str>, error: /*Ignored*/Option<glib::Error>) -> bool {
    //    unsafe { TODO: call ffi::gtk_print_settings_load_key_file() }
    //}

    pub fn set(&self, key: &str, value: Option<&str>) {
        unsafe {
            ffi::gtk_print_settings_set(self.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_bool(&self, key: &str, value: bool) {
        unsafe {
            ffi::gtk_print_settings_set_bool(self.to_glib_none().0, key.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_collate(&self, collate: bool) {
        unsafe {
            ffi::gtk_print_settings_set_collate(self.to_glib_none().0, collate.to_glib());
        }
    }

    pub fn set_default_source(&self, default_source: &str) {
        unsafe {
            ffi::gtk_print_settings_set_default_source(self.to_glib_none().0, default_source.to_glib_none().0);
        }
    }

    pub fn set_dither(&self, dither: &str) {
        unsafe {
            ffi::gtk_print_settings_set_dither(self.to_glib_none().0, dither.to_glib_none().0);
        }
    }

    pub fn set_double(&self, key: &str, value: f64) {
        unsafe {
            ffi::gtk_print_settings_set_double(self.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    pub fn set_duplex(&self, duplex: PrintDuplex) {
        unsafe {
            ffi::gtk_print_settings_set_duplex(self.to_glib_none().0, duplex);
        }
    }

    pub fn set_finishings(&self, finishings: &str) {
        unsafe {
            ffi::gtk_print_settings_set_finishings(self.to_glib_none().0, finishings.to_glib_none().0);
        }
    }

    pub fn set_int(&self, key: &str, value: i32) {
        unsafe {
            ffi::gtk_print_settings_set_int(self.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    pub fn set_length(&self, key: &str, value: f64, unit: Unit) {
        unsafe {
            ffi::gtk_print_settings_set_length(self.to_glib_none().0, key.to_glib_none().0, value, unit);
        }
    }

    pub fn set_media_type(&self, media_type: &str) {
        unsafe {
            ffi::gtk_print_settings_set_media_type(self.to_glib_none().0, media_type.to_glib_none().0);
        }
    }

    pub fn set_n_copies(&self, num_copies: i32) {
        unsafe {
            ffi::gtk_print_settings_set_n_copies(self.to_glib_none().0, num_copies);
        }
    }

    pub fn set_number_up(&self, number_up: i32) {
        unsafe {
            ffi::gtk_print_settings_set_number_up(self.to_glib_none().0, number_up);
        }
    }

    pub fn set_number_up_layout(&self, number_up_layout: NumberUpLayout) {
        unsafe {
            ffi::gtk_print_settings_set_number_up_layout(self.to_glib_none().0, number_up_layout);
        }
    }

    pub fn set_orientation(&self, orientation: PageOrientation) {
        unsafe {
            ffi::gtk_print_settings_set_orientation(self.to_glib_none().0, orientation);
        }
    }

    pub fn set_output_bin(&self, output_bin: &str) {
        unsafe {
            ffi::gtk_print_settings_set_output_bin(self.to_glib_none().0, output_bin.to_glib_none().0);
        }
    }

    //pub fn set_page_ranges(&self, page_ranges: /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 1, id: 613 }", num_ranges: i32) {
    //    unsafe { TODO: call ffi::gtk_print_settings_set_page_ranges() }
    //}

    pub fn set_page_set(&self, page_set: PageSet) {
        unsafe {
            ffi::gtk_print_settings_set_page_set(self.to_glib_none().0, page_set);
        }
    }

    pub fn set_paper_height(&self, height: f64, unit: Unit) {
        unsafe {
            ffi::gtk_print_settings_set_paper_height(self.to_glib_none().0, height, unit);
        }
    }

    //pub fn set_paper_size(&self, paper_size: /*Ignored*/&PaperSize) {
    //    unsafe { TODO: call ffi::gtk_print_settings_set_paper_size() }
    //}

    pub fn set_paper_width(&self, width: f64, unit: Unit) {
        unsafe {
            ffi::gtk_print_settings_set_paper_width(self.to_glib_none().0, width, unit);
        }
    }

    pub fn set_print_pages(&self, pages: PrintPages) {
        unsafe {
            ffi::gtk_print_settings_set_print_pages(self.to_glib_none().0, pages);
        }
    }

    pub fn set_printer(&self, printer: &str) {
        unsafe {
            ffi::gtk_print_settings_set_printer(self.to_glib_none().0, printer.to_glib_none().0);
        }
    }

    pub fn set_printer_lpi(&self, lpi: f64) {
        unsafe {
            ffi::gtk_print_settings_set_printer_lpi(self.to_glib_none().0, lpi);
        }
    }

    pub fn set_quality(&self, quality: PrintQuality) {
        unsafe {
            ffi::gtk_print_settings_set_quality(self.to_glib_none().0, quality);
        }
    }

    pub fn set_resolution(&self, resolution: i32) {
        unsafe {
            ffi::gtk_print_settings_set_resolution(self.to_glib_none().0, resolution);
        }
    }

    pub fn set_resolution_xy(&self, resolution_x: i32, resolution_y: i32) {
        unsafe {
            ffi::gtk_print_settings_set_resolution_xy(self.to_glib_none().0, resolution_x, resolution_y);
        }
    }

    pub fn set_reverse(&self, reverse: bool) {
        unsafe {
            ffi::gtk_print_settings_set_reverse(self.to_glib_none().0, reverse.to_glib());
        }
    }

    pub fn set_scale(&self, scale: f64) {
        unsafe {
            ffi::gtk_print_settings_set_scale(self.to_glib_none().0, scale);
        }
    }

    pub fn set_use_color(&self, use_color: bool) {
        unsafe {
            ffi::gtk_print_settings_set_use_color(self.to_glib_none().0, use_color.to_glib());
        }
    }

    //pub fn to_file(&self, file_name: &str, error: /*Ignored*/Option<glib::Error>) -> bool {
    //    unsafe { TODO: call ffi::gtk_print_settings_to_file() }
    //}

    //pub fn to_key_file(&self, key_file: /*Ignored*/&glib::KeyFile, group_name: &str) {
    //    unsafe { TODO: call ffi::gtk_print_settings_to_key_file() }
    //}

    pub fn unset(&self, key: &str) {
        unsafe {
            ffi::gtk_print_settings_unset(self.to_glib_none().0, key.to_glib_none().0);
        }
    }

}
