// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An extension type for the *mut Sass_Import_List class to make its methods Object-Orientated.
#[allow(non_camel_case_types)]
trait Sass_Import_List_Ext
{
	/// Make a new Sass_Import_List with space for `size` imports.
	#[inline(always)]
	fn make(size: usize) -> Self;
	
	/// Set an entry at an index < size.
	#[inline(always)]
	fn set_list_entry(self, index: usize, import_entry: Sass_Import_Entry);
}

impl Sass_Import_List_Ext for Sass_Import_List
{
	/// Make a new Sass_Import_List with space for `size` imports.
	#[inline(always)]
	fn make(size: usize) -> Self
	{
		unsafe { sass_make_import_list(size) }
	}
	
	#[inline(always)]
	fn set_list_entry(self, index: usize, import_entry: Sass_Import_Entry)
	{
		unsafe { sass_import_set_list_entry(self, index, import_entry) }
	}
}
