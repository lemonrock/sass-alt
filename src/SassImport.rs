// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// Represents the three different kinds of SassImport.
pub enum SassImport<'a, P: AsRef<Path>>
{
	/// Import is a relative path.
	RelativePath(P),
	
	/// Import is an absolute path.
	AbsolutePath(P),
	
	/// Import refers to a buffer of SCSS / SASS source bytes.
	Source(&'a CStr),
}

impl<'a, P: AsRef<Path>> SassImport<'a, P>
{
	/// Create a SassImportEntry.
	#[inline(always)]
	pub fn into_sass_import_entry(self) -> SassImportEntry
	{
		SassImportEntry::new(self, None)
	}
}
