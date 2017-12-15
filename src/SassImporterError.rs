// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.



/// An error returned from an importer.
#[derive(Debug)]
pub struct SassImporterError(SassImportEntry);

impl SassImporterError
{
	/// Create a new SassImporterError.
	#[inline(always)]
	pub fn new<P: AsRef<Path>>(imp_path: Option<P>, abs_path: Option<P>, message: &CStr, line: usize, column: usize) -> Self
	{
		let entry = SassImportEntry::make_import_entry(imp_path, abs_path, None, None);
		assert!(!entry.is_null());
		entry.set_error(message, line, column);
		SassImporterError(SassImportEntry(entry, true))
	}
	
	#[inline(always)]
	fn into_sass_import_list(self) -> Sass_Import_List
	{
		let list = Sass_Import_List::make(1);
		list.set_list_entry(0, self.0.transfer_ownership_to_c());
		list
	}
}
