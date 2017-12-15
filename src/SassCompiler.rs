// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A Wrapper around the Sass_Compiler object to make it easier to use correctly.
#[derive(Debug, Copy, Clone)]
pub struct SassCompiler(*mut Sass_Compiler);

impl SassCompiler
{
	/// Get Sass_Context object.
	#[inline(always)]
	pub fn context(&self) -> *mut Sass_Context
	{
		self.0.get_context()
	}
	
	/// Get Sass_Options object.
	#[inline(always)]
	pub fn get_options(&self) -> *mut Sass_Options
	{
		self.0.get_options()
	}
	
	/// Get compiler state object.
	#[inline(always)]
	pub fn get_state(&self) -> Sass_Compiler_State
	{
		self.0.get_state()
	}
	
	/// Iterate over imports.
	/// Iterator also provides a specific `last_import()` method for convenience.
	#[inline(always)]
	pub fn iter_imports(&self) -> ImportsSassCompilerIterator
	{
		ImportsSassCompilerIterator::new(self.0)
	}
	
	/// Iterate over callees.
	/// Iterator also provides a specific `last_callee()` method for convenience.
	#[inline(always)]
	pub fn iter_callees(&self) -> CalleesSassCompilerIterator
	{
		CalleesSassCompilerIterator::new(self.0)
	}
	
	/// Helper implementation to make import entry error
	#[inline(always)]
	pub fn make_import_error<P: AsRef<Path>>(imp_path: Option<P>, abs_path: Option<P>, message: &CStr, line: usize, column: usize) -> Sass_Import_Entry
	{
		let entry = Self::make_import_entry(imp_path, abs_path, None, None);
		entry.set_error(message, line, column)
	}
	
	/// Helper implementation to make import entries
	#[inline(always)]
	pub fn make_import_entry<P: AsRef<Path>>(imp_path: Option<P>, abs_path: Option<P>, source: Option<&CStr>, source_map: Option<&CStr>) -> Sass_Import_Entry
	{
		#[cfg(windows)]
		fn path_to_pointer<P: AsRef<Path>>(path: Option<P>) -> (Option<CString>, *const c_char)
		{
			if let Some(ref path) = path
			{
				let c_string = CString::new(sass_input_file_path.as_ref().to_str().unwrap()).unwrap();
				let pointer = c_string.as_ptr();
				(Some(c_string), pointer)
			}
			else
			{
				(None, null())
			}
		}
		
		#[cfg(not(windows))]
		fn path_to_pointer<P: AsRef<Path>>(path: Option<P>) -> (Option<CString>, *const c_char)
		{
			if let Some(ref path) = path
			{
				use ::std::os::unix::ffi::OsStrExt;
				(None, path.as_ref().as_os_str().as_bytes().as_ptr() as *const c_char)
			}
			else
			{
				(None, null())
			}
		}
		
		let (_windows_hack, imp_path) = path_to_pointer(imp_path);
		let (_windows_hack, abs_path) = path_to_pointer(abs_path);
		
		
		unsafe
		{
			sass_make_import
			(
				imp_path,
				abs_path,
				match source
				{
					None => null_mut(),
					Some(source) => sass_copy_c_string(source.as_ptr()),
				},
				match source_map
				{
					None => null_mut(),
					Some(source_map) => sass_copy_c_string(source_map.as_ptr()),
				}
			)
		}
	}
	
}
