// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A set of options for creating SASS contexts.
#[derive(Debug, Clone)]
pub struct SassOptions<'p, P: 'p + AsRef<Path>>
{
	/// Output style for generated CSS.
	pub output_style: OutputStyle,
	
	/// Should generated CSS have source comments?
	pub source_comments: bool,
	
	/// Embed source map?
	pub source_map_embed: bool,
	
	/// Embed source as comments?
	pub source_map_contents: bool,
	
	/// Embed source as url?
	pub source_map_file_urls: bool,
	
	/// Omit?
	pub omit_source_map_url: bool,
	
	/// Indent (should be "")
	pub indent: CString,
	
	/// Linefeed (should be "\n")
	pub linefeed: CString,
	
	/// Precision for floating-point numbers in generated CSS.
	pub precision: u8,
	
	/// Is input to compilation in SASS or SCSS format?
	pub input_syntax: InputSyntax,
	
	/// List of include paths.
	pub include_paths: &'p [P],
	
	/// List of SASS functions.
	pub function_list: Rc<SassFunctionList>,
	
	/// List of SASS importers.
	pub importer_list: Rc<SassImporterList>,
	
	/// List of SASS headers (type is same as importers).
	pub header_list: Rc<SassImporterList>,
}

impl<'p, P: AsRef<Path>> SassOptions<'p, P>
{
	/// Creates a SassOptions which minifies CSS output.
	pub fn minified_css(input_syntax: InputSyntax, include_paths: &'p [P], function_list: &Rc<SassFunctionList>, importer_list: &Rc<SassImporterList>, header_list: &Rc<SassImporterList>) -> Self
	{
		SassOptions
		{
			output_style: OutputStyle::Compressed,
			source_comments: false,
			source_map_embed: false,
			source_map_contents: false,
			source_map_file_urls: false,
			omit_source_map_url: true,
			indent: CString::new("").unwrap(),
			linefeed: CString::new("\n").unwrap(),
			precision: 1,
			input_syntax,
			include_paths,
			function_list: function_list.clone(),
			importer_list: importer_list.clone(),
			header_list: header_list.clone(),
		}
	}
	
	/// An entry point to compile SASS data.
	#[inline(always)]
	pub fn compile_data(&self, sass_input_data: &str) -> Result<String, SassCompileError>
	{
		let sass_context = SassContext::<*mut Sass_Data_Context>::new_with_data(sass_input_data, self);
		sass_context.compile()
	}
	
	/// An entry point to compile SASS files.
	#[inline(always)]
	pub fn compile_file(&self, sass_input_file: P) -> Result<String, SassCompileError>
	{
		let sass_context = SassContext::<*mut Sass_File_Context>::new_with_file(sass_input_file, self);
		sass_context.compile()
	}
}
