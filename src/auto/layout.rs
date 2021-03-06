// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Alignment;
use AttrList;
use Context;
use EllipsizeMode;
use FontDescription;
use LayoutIter;
use LayoutLine;
use Rectangle;
use TabArray;
use WrapMode;
use ffi;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;

glib_wrapper! {
    pub struct Layout(Object<ffi::PangoLayout, ffi::PangoLayoutClass, LayoutClass>);

    match fn {
        get_type => || ffi::pango_layout_get_type(),
    }
}

impl Layout {
    pub fn new<P: IsA<Context>>(context: &P) -> Layout {
        unsafe {
            from_glib_full(ffi::pango_layout_new(context.as_ref().to_glib_none().0))
        }
    }
}

pub const NONE_LAYOUT: Option<&Layout> = None;

pub trait LayoutExt: 'static {
    fn context_changed(&self);

    fn copy(&self) -> Option<Layout>;

    fn get_alignment(&self) -> Alignment;

    fn get_attributes(&self) -> Option<AttrList>;

    fn get_auto_dir(&self) -> bool;

    fn get_baseline(&self) -> i32;

    fn get_character_count(&self) -> i32;

    fn get_context(&self) -> Option<Context>;

    fn get_cursor_pos(&self, index_: i32) -> (Rectangle, Rectangle);

    fn get_ellipsize(&self) -> EllipsizeMode;

    fn get_extents(&self) -> (Rectangle, Rectangle);

    fn get_font_description(&self) -> Option<FontDescription>;

    fn get_height(&self) -> i32;

    fn get_indent(&self) -> i32;

    fn get_iter(&self) -> Option<LayoutIter>;

    fn get_justify(&self) -> bool;

    fn get_line(&self, line: i32) -> Option<LayoutLine>;

    fn get_line_count(&self) -> i32;

    fn get_line_readonly(&self, line: i32) -> Option<LayoutLine>;

    fn get_lines(&self) -> Vec<LayoutLine>;

    fn get_lines_readonly(&self) -> Vec<LayoutLine>;

    //fn get_log_attrs(&self, attrs: /*Ignored*/Vec<LogAttr>) -> i32;

    //fn get_log_attrs_readonly(&self) -> /*Ignored*/Vec<LogAttr>;

    fn get_pixel_extents(&self) -> (Rectangle, Rectangle);

    fn get_pixel_size(&self) -> (i32, i32);

    fn get_serial(&self) -> u32;

    fn get_single_paragraph_mode(&self) -> bool;

    fn get_size(&self) -> (i32, i32);

    fn get_spacing(&self) -> i32;

    fn get_tabs(&self) -> Option<TabArray>;

    fn get_text(&self) -> Option<GString>;

    fn get_unknown_glyphs_count(&self) -> i32;

    fn get_width(&self) -> i32;

    fn get_wrap(&self) -> WrapMode;

    fn index_to_line_x(&self, index_: i32, trailing: bool) -> (i32, i32);

    fn index_to_pos(&self, index_: i32) -> Rectangle;

    fn is_ellipsized(&self) -> bool;

    fn is_wrapped(&self) -> bool;

    fn move_cursor_visually(&self, strong: bool, old_index: i32, old_trailing: i32, direction: i32) -> (i32, i32);

    fn set_alignment(&self, alignment: Alignment);

    fn set_attributes<'a, P: Into<Option<&'a AttrList>>>(&self, attrs: P);

    fn set_auto_dir(&self, auto_dir: bool);

    fn set_ellipsize(&self, ellipsize: EllipsizeMode);

    fn set_font_description<'a, P: Into<Option<&'a FontDescription>>>(&self, desc: P);

    fn set_height(&self, height: i32);

    fn set_indent(&self, indent: i32);

    fn set_justify(&self, justify: bool);

    fn set_markup(&self, markup: &str);

    fn set_markup_with_accel(&self, markup: &str, accel_marker: char) -> char;

    fn set_single_paragraph_mode(&self, setting: bool);

    fn set_spacing(&self, spacing: i32);

    fn set_tabs<'a, P: Into<Option<&'a TabArray>>>(&self, tabs: P);

    fn set_text(&self, text: &str);

    fn set_width(&self, width: i32);

    fn set_wrap(&self, wrap: WrapMode);

    fn xy_to_index(&self, x: i32, y: i32) -> (bool, i32, i32);
}

impl<O: IsA<Layout>> LayoutExt for O {
    fn context_changed(&self) {
        unsafe {
            ffi::pango_layout_context_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn copy(&self) -> Option<Layout> {
        unsafe {
            from_glib_full(ffi::pango_layout_copy(self.as_ref().to_glib_none().0))
        }
    }

    fn get_alignment(&self) -> Alignment {
        unsafe {
            from_glib(ffi::pango_layout_get_alignment(self.as_ref().to_glib_none().0))
        }
    }

    fn get_attributes(&self) -> Option<AttrList> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_attributes(self.as_ref().to_glib_none().0))
        }
    }

    fn get_auto_dir(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_get_auto_dir(self.as_ref().to_glib_none().0))
        }
    }

    fn get_baseline(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_baseline(self.as_ref().to_glib_none().0)
        }
    }

    fn get_character_count(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_character_count(self.as_ref().to_glib_none().0)
        }
    }

    fn get_context(&self) -> Option<Context> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_context(self.as_ref().to_glib_none().0))
        }
    }

    fn get_cursor_pos(&self, index_: i32) -> (Rectangle, Rectangle) {
        unsafe {
            let mut strong_pos = Rectangle::uninitialized();
            let mut weak_pos = Rectangle::uninitialized();
            ffi::pango_layout_get_cursor_pos(self.as_ref().to_glib_none().0, index_, strong_pos.to_glib_none_mut().0, weak_pos.to_glib_none_mut().0);
            (strong_pos, weak_pos)
        }
    }

    fn get_ellipsize(&self) -> EllipsizeMode {
        unsafe {
            from_glib(ffi::pango_layout_get_ellipsize(self.as_ref().to_glib_none().0))
        }
    }

    fn get_extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_get_extents(self.as_ref().to_glib_none().0, ink_rect.to_glib_none_mut().0, logical_rect.to_glib_none_mut().0);
            (ink_rect, logical_rect)
        }
    }

    fn get_font_description(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_font_description(self.as_ref().to_glib_none().0))
        }
    }

    fn get_height(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_height(self.as_ref().to_glib_none().0)
        }
    }

    fn get_indent(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_indent(self.as_ref().to_glib_none().0)
        }
    }

    fn get_iter(&self) -> Option<LayoutIter> {
        unsafe {
            from_glib_full(ffi::pango_layout_get_iter(self.as_ref().to_glib_none().0))
        }
    }

    fn get_justify(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_get_justify(self.as_ref().to_glib_none().0))
        }
    }

    fn get_line(&self, line: i32) -> Option<LayoutLine> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_line(self.as_ref().to_glib_none().0, line))
        }
    }

    fn get_line_count(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_line_count(self.as_ref().to_glib_none().0)
        }
    }

    fn get_line_readonly(&self, line: i32) -> Option<LayoutLine> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_line_readonly(self.as_ref().to_glib_none().0, line))
        }
    }

    fn get_lines(&self) -> Vec<LayoutLine> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::pango_layout_get_lines(self.as_ref().to_glib_none().0))
        }
    }

    fn get_lines_readonly(&self) -> Vec<LayoutLine> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::pango_layout_get_lines_readonly(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_log_attrs(&self, attrs: /*Ignored*/Vec<LogAttr>) -> i32 {
    //    unsafe { TODO: call ffi::pango_layout_get_log_attrs() }
    //}

    //fn get_log_attrs_readonly(&self) -> /*Ignored*/Vec<LogAttr> {
    //    unsafe { TODO: call ffi::pango_layout_get_log_attrs_readonly() }
    //}

    fn get_pixel_extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_get_pixel_extents(self.as_ref().to_glib_none().0, ink_rect.to_glib_none_mut().0, logical_rect.to_glib_none_mut().0);
            (ink_rect, logical_rect)
        }
    }

    fn get_pixel_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::pango_layout_get_pixel_size(self.as_ref().to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn get_serial(&self) -> u32 {
        unsafe {
            ffi::pango_layout_get_serial(self.as_ref().to_glib_none().0)
        }
    }

    fn get_single_paragraph_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_get_single_paragraph_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn get_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::pango_layout_get_size(self.as_ref().to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_spacing(self.as_ref().to_glib_none().0)
        }
    }

    fn get_tabs(&self) -> Option<TabArray> {
        unsafe {
            from_glib_full(ffi::pango_layout_get_tabs(self.as_ref().to_glib_none().0))
        }
    }

    fn get_text(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_text(self.as_ref().to_glib_none().0))
        }
    }

    fn get_unknown_glyphs_count(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_unknown_glyphs_count(self.as_ref().to_glib_none().0)
        }
    }

    fn get_width(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_width(self.as_ref().to_glib_none().0)
        }
    }

    fn get_wrap(&self) -> WrapMode {
        unsafe {
            from_glib(ffi::pango_layout_get_wrap(self.as_ref().to_glib_none().0))
        }
    }

    fn index_to_line_x(&self, index_: i32, trailing: bool) -> (i32, i32) {
        unsafe {
            let mut line = mem::uninitialized();
            let mut x_pos = mem::uninitialized();
            ffi::pango_layout_index_to_line_x(self.as_ref().to_glib_none().0, index_, trailing.to_glib(), &mut line, &mut x_pos);
            (line, x_pos)
        }
    }

    fn index_to_pos(&self, index_: i32) -> Rectangle {
        unsafe {
            let mut pos = Rectangle::uninitialized();
            ffi::pango_layout_index_to_pos(self.as_ref().to_glib_none().0, index_, pos.to_glib_none_mut().0);
            pos
        }
    }

    fn is_ellipsized(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_is_ellipsized(self.as_ref().to_glib_none().0))
        }
    }

    fn is_wrapped(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_is_wrapped(self.as_ref().to_glib_none().0))
        }
    }

    fn move_cursor_visually(&self, strong: bool, old_index: i32, old_trailing: i32, direction: i32) -> (i32, i32) {
        unsafe {
            let mut new_index = mem::uninitialized();
            let mut new_trailing = mem::uninitialized();
            ffi::pango_layout_move_cursor_visually(self.as_ref().to_glib_none().0, strong.to_glib(), old_index, old_trailing, direction, &mut new_index, &mut new_trailing);
            (new_index, new_trailing)
        }
    }

    fn set_alignment(&self, alignment: Alignment) {
        unsafe {
            ffi::pango_layout_set_alignment(self.as_ref().to_glib_none().0, alignment.to_glib());
        }
    }

    fn set_attributes<'a, P: Into<Option<&'a AttrList>>>(&self, attrs: P) {
        let attrs = attrs.into();
        unsafe {
            ffi::pango_layout_set_attributes(self.as_ref().to_glib_none().0, attrs.to_glib_none().0);
        }
    }

    fn set_auto_dir(&self, auto_dir: bool) {
        unsafe {
            ffi::pango_layout_set_auto_dir(self.as_ref().to_glib_none().0, auto_dir.to_glib());
        }
    }

    fn set_ellipsize(&self, ellipsize: EllipsizeMode) {
        unsafe {
            ffi::pango_layout_set_ellipsize(self.as_ref().to_glib_none().0, ellipsize.to_glib());
        }
    }

    fn set_font_description<'a, P: Into<Option<&'a FontDescription>>>(&self, desc: P) {
        let desc = desc.into();
        unsafe {
            ffi::pango_layout_set_font_description(self.as_ref().to_glib_none().0, desc.to_glib_none().0);
        }
    }

    fn set_height(&self, height: i32) {
        unsafe {
            ffi::pango_layout_set_height(self.as_ref().to_glib_none().0, height);
        }
    }

    fn set_indent(&self, indent: i32) {
        unsafe {
            ffi::pango_layout_set_indent(self.as_ref().to_glib_none().0, indent);
        }
    }

    fn set_justify(&self, justify: bool) {
        unsafe {
            ffi::pango_layout_set_justify(self.as_ref().to_glib_none().0, justify.to_glib());
        }
    }

    fn set_markup(&self, markup: &str) {
        let length = markup.len() as i32;
        unsafe {
            ffi::pango_layout_set_markup(self.as_ref().to_glib_none().0, markup.to_glib_none().0, length);
        }
    }

    fn set_markup_with_accel(&self, markup: &str, accel_marker: char) -> char {
        let length = markup.len() as i32;
        unsafe {
            let mut accel_char = mem::uninitialized();
            ffi::pango_layout_set_markup_with_accel(self.as_ref().to_glib_none().0, markup.to_glib_none().0, length, accel_marker.to_glib(), &mut accel_char);
            from_glib(accel_char)
        }
    }

    fn set_single_paragraph_mode(&self, setting: bool) {
        unsafe {
            ffi::pango_layout_set_single_paragraph_mode(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::pango_layout_set_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn set_tabs<'a, P: Into<Option<&'a TabArray>>>(&self, tabs: P) {
        let tabs = tabs.into();
        unsafe {
            ffi::pango_layout_set_tabs(self.as_ref().to_glib_none().0, mut_override(tabs.to_glib_none().0));
        }
    }

    fn set_text(&self, text: &str) {
        let length = text.len() as i32;
        unsafe {
            ffi::pango_layout_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0, length);
        }
    }

    fn set_width(&self, width: i32) {
        unsafe {
            ffi::pango_layout_set_width(self.as_ref().to_glib_none().0, width);
        }
    }

    fn set_wrap(&self, wrap: WrapMode) {
        unsafe {
            ffi::pango_layout_set_wrap(self.as_ref().to_glib_none().0, wrap.to_glib());
        }
    }

    fn xy_to_index(&self, x: i32, y: i32) -> (bool, i32, i32) {
        unsafe {
            let mut index_ = mem::uninitialized();
            let mut trailing = mem::uninitialized();
            let ret = from_glib(ffi::pango_layout_xy_to_index(self.as_ref().to_glib_none().0, x, y, &mut index_, &mut trailing));
            (ret, index_, trailing)
        }
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Layout")
    }
}
