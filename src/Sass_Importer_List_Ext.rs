// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An extension type for the *mut Sass_Importer_List class to make its methods Object-Orientated.
#[allow(non_camel_case_types)]
trait Sass_Importer_List_Ext
{
	/// Make a new Sass_Import_List with space for `size` importers.
	#[inline(always)]
	fn make(size: usize) -> Self;
	
	/// Set an entry at an index < size.
	#[inline(always)]
	fn set_list_entry(self, index: usize, importer_entry: Sass_Importer_Entry);
	
	/// Delete (drop) this object. Guards that this object is not null.
	#[inline(always)]
	fn delete(self);
}

impl Sass_Importer_List_Ext for Sass_Importer_List
{
	#[inline(always)]
	fn make(size: usize) -> Self
	{
		unsafe { sass_make_importer_list(size) }
	}
	
	#[inline(always)]
	fn set_list_entry(self, index: usize, importer_entry: Sass_Importer_Entry)
	{
		unsafe { sass_importer_set_list_entry(self, index, importer_entry) }
	}
	
	#[inline(always)]
	fn delete(self)
	{
		if !self.is_null()
		{
			unsafe { sass_delete_importer_list(self) }
		}
	}
}
