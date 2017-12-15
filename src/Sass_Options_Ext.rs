// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An extension type for the *mut Sass_Options class to make its methods Object-Orientated.
#[allow(non_camel_case_types)]
trait Sass_Options_Ext
{
	/// Set the output style of generated CSS.
	#[inline(always)]
	fn set_output_style(self, style: OutputStyle);
	
	/// Set whether generated CSS should have comments.
	#[inline(always)]
	fn set_source_comments(self, on: bool);
	
	/// Set whether generated CSS should have source map contents.
	#[inline(always)]
	fn set_source_map_contents(self, on: bool);
	
	/// Set whether generated CSS should have source map embedded.
	#[inline(always)]
	fn set_source_map_embed(self, on: bool);
	
	/// Set whether generated CSS should have source map URL.
	#[inline(always)]
	fn set_source_map_file_urls(self, on: bool);
	
	/// Set whether generated CSS should have source map URL omitted
	#[inline(always)]
	fn set_omit_source_map_url(self, on: bool);
	
	/// Set the precision of generated CSS.
	#[inline(always)]
	fn set_precision(self, precision: u8);
	
	/// Set the indent of generated CSS.
	#[inline(always)]
	fn set_indent(self, indent: &CStr);
	
	/// Set the line feed of generated CSS.
	#[inline(always)]
	fn set_linefeed(self, linefeed: &CStr);
	
	/// Set whether the source data is in SCSS format.
	#[inline(always)]
	fn set_is_indented_syntax_src(self);
	
	/// Set a colon (Unix) or semicolon (Windows) delimited set of include paths.
	#[inline(always)]
	fn set_include_path<P: AsRef<Path>>(self, paths: &[P]);
	
	/// Set the list of functions.
	#[inline(always)]
	fn set_c_functions(self, function_list: Sass_Function_List);
	
	/// Set the list of importers.
	#[inline(always)]
	fn set_c_importers(self, importer_list: Sass_Importer_List);
	
	/// Set the list of header importers.
	#[inline(always)]
	fn set_c_headers(self, importer_list: Sass_Importer_List);
}

impl Sass_Options_Ext for *mut Sass_Options
{
	#[inline(always)]
	fn set_output_style(self, output_style: OutputStyle)
	{
		use self::OutputStyle::*;
		use self::Sass_Output_Style::*;
		
		let style = match output_style
		{
			Nested => SASS_STYLE_NESTED,
			Expanded => SASS_STYLE_EXPANDED,
			Compact => SASS_STYLE_COMPACT,
			Compressed => SASS_STYLE_COMPRESSED,
		};
		
		unsafe { sass_option_set_output_style(self, style) }
	}
	
	#[inline(always)]
	fn set_source_comments(self, on: bool)
	{
		unsafe { sass_option_set_source_comments(self, on) }
	}
	
	#[inline(always)]
	fn set_source_map_contents(self, on: bool)
	{
		unsafe { sass_option_set_source_map_embed(self, on) }
	}
	
	#[inline(always)]
	fn set_source_map_embed(self, on: bool)
	{
		unsafe { sass_option_set_source_map_embed(self, on) }
	}
	
	#[inline(always)]
	fn set_source_map_file_urls(self, on: bool)
	{
		unsafe { sass_option_set_source_map_file_urls(self, on) }
	}
	
	#[inline(always)]
	fn set_omit_source_map_url(self, on: bool)
	{
		unsafe { sass_option_set_omit_source_map_url(self, on) }
	}
	
	#[inline(always)]
	fn set_precision(self, precision: u8)
	{
		unsafe { sass_option_set_precision(self, precision as i32) }
	}
	
	#[inline(always)]
	fn set_indent(self, indent: &CStr)
	{
		unsafe { sass_option_set_indent(self, indent.as_ptr()) }
	}
	
	#[inline(always)]
	fn set_linefeed(self, linefeed: &CStr)
	{
		unsafe { sass_option_set_linefeed(self, linefeed.as_ptr()) }
	}
	
	#[inline(always)]
	fn set_is_indented_syntax_src(self)
	{
		unsafe { sass_option_set_is_indented_syntax_src(self, true) }
	}
	
	#[inline(always)]
	fn set_include_path<P: AsRef<Path>>(self, paths: &[P])
	{
		if paths.is_empty()
		{
			return;
		}
		
		#[cfg(windows)] const SEPARATOR: &'static str = ";";
		#[cfg(not(windows))] const SEPARATOR: &'static str = ",";
		
		let mut after_first = false;
		let mut joined_paths = OsString::new();
		for path in paths.iter()
		{
			if after_first
			{
				joined_paths.push(SEPARATOR);
			}
			else
			{
				after_first = true;
			}
			
			joined_paths.push(path.as_ref().as_os_str());
		}
		
		#[cfg(windows)]
		{
			let string = joined_paths.into_string().unwrap();
			let c_string = CString::new(string).unwrap();
			
			unsafe { sass_option_set_include_path(self, c_string.as_ptr()) };
		}
		
		#[cfg(not(windows))]
		{
			use ::std::os::unix::ffi::OsStrExt;
			let bytes = joined_paths.as_bytes();
			
			unsafe { sass_option_set_include_path(self, bytes.as_ptr() as *const _) }
		}
	}
	
	#[inline(always)]
	fn set_c_functions(self, function_list: Sass_Function_List)
	{
		unsafe { sass_option_set_c_functions(self, function_list) }
	}
	
	#[inline(always)]
	fn set_c_importers(self, importer_list: Sass_Importer_List)
	{
		unsafe { sass_option_set_c_importers(self, importer_list) }
	}
	
	#[inline(always)]
	fn set_c_headers(self, importer_list: Sass_Importer_List)
	{
		unsafe { sass_option_set_c_headers(self, importer_list) }
	}
}
