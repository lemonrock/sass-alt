// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An extension type for the *mut Sass_Import_Entry class to make its methods Object-Orientated.
#[allow(non_camel_case_types)]
pub trait Sass_Import_Entry_Ext
{
	/// Delete (drop) a Sass_import_Entry
	#[inline(always)]
	fn delete(self);
	
	/// Get the absolute path of an import.
	/// Can be null.
	#[inline(always)]
	fn get_abs_path<'a>(self) -> Option<&'a CStr>;
	
	/// Get the path of an import.
	/// Can be null.
	#[inline(always)]
	fn get_imp_path<'a>(self) -> Option<&'a CStr>;
	
	/// Get source.
	/// Can be null.
	#[inline(always)]
	fn get_source<'a>(self) -> Option<&'a CStr>;
	
	/// Get source map.
	/// Can be null.
	#[inline(always)]
	fn get_source_map<'a>(self) -> Option<&'a CStr>;
	
	/// Get error line.
	#[inline(always)]
	fn get_error_line(self) -> usize;
	
	/// Get error column.
	#[inline(always)]
	fn get_error_column(self) -> usize;
	
	/// Get error message.
	/// Can be null.
	#[inline(always)]
	fn get_error_message<'a>(self) -> Option<&'a CStr>;
	
	/// Set an error.
	/// The returned value Self, is the same as that passed, self, ie it is for convenience.
	#[inline(always)]
	fn set_error(self, message: &CStr, line: usize, column: usize) -> Self;
}

impl Sass_Import_Entry_Ext for Sass_Import_Entry
{
	#[inline(always)]
	fn delete(self)
	{
		if !self.is_null()
		{
			unsafe { sass_delete_import(self) }
		}
	}
	
	#[inline(always)]
	fn get_abs_path<'a>(self) -> Option<&'a CStr>
	{
		let pointer = unsafe { sass_import_get_abs_path(self) };
		if pointer.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { CStr::from_ptr(pointer) })
		}
	}
	
	#[inline(always)]
	fn get_imp_path<'a>(self) -> Option<&'a CStr>
	{
		let pointer = unsafe { sass_import_get_imp_path(self) };
		if pointer.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { CStr::from_ptr(pointer) })
		}
	}
	
	#[inline(always)]
	fn get_source<'a>(self) -> Option<&'a CStr>
	{
		let pointer = unsafe { sass_import_get_source(self) };
		if pointer.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { CStr::from_ptr(pointer) })
		}
	}
	
	#[inline(always)]
	fn get_source_map<'a>(self) -> Option<&'a CStr>
	{
		let pointer = unsafe { sass_import_get_srcmap(self) };
		if pointer.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { CStr::from_ptr(pointer) })
		}
	}
	
	#[inline(always)]
	fn get_error_line(self) -> usize
	{
		unsafe { sass_import_get_error_line(self) }
	}
	
	#[inline(always)]
	fn get_error_column(self) -> usize
	{
		unsafe { sass_import_get_error_column(self) }
	}
	
	#[inline(always)]
	fn get_error_message<'a>(self) -> Option<&'a CStr>
	{
		let pointer = unsafe { sass_import_get_error_message(self) };
		if pointer.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { CStr::from_ptr(pointer) })
		}
	}
	
	#[inline(always)]
	fn set_error(self, message: &CStr, line: usize, column: usize) -> Self
	{
		unsafe { sass_import_set_error(self, message.as_ptr(), line, column) }
	}
}
