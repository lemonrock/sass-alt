// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A wrapper type that holds a list of SASS importers and their Rust representations so that Rust importer lifetimes exceed references to SASS importer held by the libsass C object Sass_Importer_List.
#[derive(Debug)]
pub struct SassImporterList(Sass_Importer_List, Vec<Box<SassImporterTraitObject>>);

impl Drop for SassImporterList
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.delete()
	}
}

impl Default for SassImporterList
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new(vec![])
	}
}

impl SassImporterList
{
	/// Create a new list of SASS importers. Called by value as we need to take ownership of the SASS importers to manage their lifetimes.
	#[inline(always)]
	pub fn new(sass_importers: Vec<SassImporterTraitObject>) -> Self
	{
		let list = Sass_Importer_List::make(sass_importers.len());
		let mut drop_sass_importers_when_importer_list_drops = Vec::with_capacity(sass_importers.len());
		let mut index = 0;
		for sass_importer in sass_importers
		{
			let priority = sass_importer.priority();
			
			let mut cookie_holder: Box<SassImporterTraitObject> = Box::new(sass_importer);
			let cookie = cookie_holder.as_mut() as *mut _ as *mut ::std::os::raw::c_void;
			
			let importer_entry = unsafe { sass_make_importer(Some(Self::call), priority, cookie) };
			
			drop_sass_importers_when_importer_list_drops.push(cookie_holder);
			list.set_list_entry(index, importer_entry);
			index += 1;
		}
		SassImporterList(list, drop_sass_importers_when_importer_list_drops)
	}
	
	extern "C" fn call(path: *const c_char, cb: Sass_Importer_Entry, comp: *mut Sass_Compiler) -> Sass_Import_List
	{
		let cookie = cb.get_cookie();
		let raw_this = cookie as *mut SassImporterTraitObject;
		let this = unsafe { &mut *raw_this };
		let c_str_path = unsafe { CStr::from_ptr(path) };
		
		match this.callback(c_str_path, SassCompiler(comp))
		{
			Ok(import_entries) => if let Some(import_entries) = import_entries
			{
				let list = Sass_Import_List::make(import_entries.len());
				let mut index = 0;
				for import_entry in import_entries
				{
					list.set_list_entry(index, import_entry.transfer_ownership_to_c());
					index += 1;
				}
				list
			}
			else
			{
				// Probably the same as null, but libsass documentation insists on zero.
				0 as *mut _
			},
			
			Err(error) => error.into_sass_import_list()
		}
	}
	
	#[inline(always)]
	fn set_importers_on_options(&self, options: *mut Sass_Options)
	{
		if !self.1.is_empty()
		{
			options.set_c_importers(self.0);
		}
	}
	
	#[inline(always)]
	fn set_headers_on_options(&self, options: *mut Sass_Options)
	{
		if !self.1.is_empty()
		{
			options.set_c_headers(self.0);
		}
	}
}
