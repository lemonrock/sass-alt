// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A memory-safe wrapper around a Sass_Import_Entry which ensures it gets dropped as required.
#[derive(Debug)]
pub struct SassImportEntry(Sass_Import_Entry, bool);

impl Drop for SassImportEntry
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.is_owned_by_rust() && !self.0.is_null()
		{
			self.0.delete()
		}
	}
}

impl SassImportEntry
{
	/// Create a new SassImportEntry.
	#[inline(always)]
	fn new<'a, P: AsRef<Path>>(sass_import: SassImport<'a, P>, source_map: Option<&'a CStr>) -> Self
	{
		use self::SassImport::*;
		
		let entry = match sass_import
		{
			RelativePath(relative_path) => Self::make_import_entry(Some(relative_path), None, None, source_map),
			
			AbsolutePath(absolute_path) => Self::make_import_entry(None, Some(absolute_path), None, source_map),
			
			Source(source) => Self::make_import_entry::<&Path>(None, None, Some(source), source_map),
		};
		assert!(!entry.is_null());
		SassImportEntry(entry, true)
	}
	
	/// Is this owned by Rust?
	#[inline(always)]
	fn is_owned_by_rust(&self) -> bool
	{
		self.1
	}
	
	/// Forgets this object and yields up original C pointer.
	/// Does not free this object.
	#[inline(always)]
	fn transfer_ownership_to_c(mut self) -> Sass_Import_Entry
	{
		self.1 = false;
		let pointer = self.0;
		forget(self);
		pointer
	}
	
	// source_map is ignored by libsass currently.
	#[inline(always)]
	fn make_import_entry<P: AsRef<Path>>(imp_path: Option<P>, abs_path: Option<P>, source: Option<&CStr>, source_map: Option<&CStr>) -> Sass_Import_Entry
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
