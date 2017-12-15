// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A type to model SASS importers.
/// See `make_import_entry` and `make_import_error()` on SassCompiler for helpers to create Sass_Import_Entry
/// Include it by using `use ::sass_alt::Sass_Import_Entry_Ext;`.
pub trait SassImporter: Debug
{
	/// The priority of this importer / header. And, weirdly, it really is a double-precision floating point value.
	fn priority(&self) -> f64
	{
		0.0f64
	}
	
	/// The implementation of this SASS importer.
	/// Return Some() with a list of import entries (sources of CSS or SASS data).
	/// Return None to tell libsass to handle the import by itself (as if no custom importer was in use).
	fn callback(&mut self, path: &CStr, compiler: SassCompiler) -> Result<Option<Vec<SassImportEntry>>, SassImporterError>;
}
