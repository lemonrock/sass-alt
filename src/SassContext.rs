// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// Wraps a SASS data or file context and holds a list of function used by that context.
#[derive(Debug)]
pub struct SassContext<S: DataOrFileSassContextMutPointer>(S, Rc<SassFunctionList>, Rc<SassImporterList>, Rc<SassImporterList>);

impl<S: DataOrFileSassContextMutPointer> Drop for SassContext<S>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.delete()
	}
}

impl<S: DataOrFileSassContextMutPointer> SassContext<S>
{
	/// Creates a new SassContext from data and options.
	#[inline(always)]
	pub fn new_with_data<'p, P: AsRef<Path>>(sass_input_data: &str, useful_sass_options: &SassOptions<'p, P>) -> SassContext<*mut Sass_Data_Context>
	{
		let source_string = CString::new(sass_input_data).unwrap();
		let data_context = unsafe { sass_make_data_context(sass_copy_c_string(source_string.as_ptr())) };
		Self::finish_new(data_context, useful_sass_options)
	}
	
	/// Creates a new SassContext from a file and options.
	#[inline(always)]
	pub fn new_with_file<'p, P: AsRef<Path>>(sass_input_file_path: P, useful_sass_options: &SassOptions<'p, P>) -> SassContext<*mut Sass_File_Context>
	{
		#[cfg(windows)]
		{
			let c_string = CString::new(sass_input_file_path.as_ref().to_str().unwrap()).unwrap();
			let pointer = c_string.as_ptr();
			Self::finish_new(unsafe { sass_make_file_context(pointer) }, useful_sass_options)
		}
		
		#[cfg(not(windows))]
		{
			use ::std::os::unix::ffi::OsStrExt;
			let bytes = sass_input_file_path.as_ref().as_os_str().as_bytes();
			
			let pointer = bytes.as_ptr() as *const c_char;
			
			Self::finish_new(unsafe { sass_make_file_context(pointer) }, useful_sass_options)
		}
	}
	
	#[inline(always)]
	fn finish_new<'p, P: AsRef<Path>, S2: DataOrFileSassContextMutPointer>(data_or_file_context: S2, useful_sass_options: &SassOptions<'p, P>) -> SassContext<S2>
	{
		let options = data_or_file_context.get_options();
		
		options.set_output_style(useful_sass_options.output_style);
		
		options.set_source_comments(useful_sass_options.source_comments);
		
		options.set_source_map_embed(useful_sass_options.source_map_embed);
		
		options.set_source_map_contents(useful_sass_options.source_map_contents);
		
		options.set_source_map_file_urls(useful_sass_options.source_map_file_urls);
		
		options.set_omit_source_map_url(useful_sass_options.omit_source_map_url);
		
		if !useful_sass_options.indent.to_bytes().is_empty()
		{
			options.set_indent(&useful_sass_options.indent)
		}
		
		if !useful_sass_options.linefeed.to_bytes().is_empty()
		{
			options.set_linefeed(&useful_sass_options.linefeed)
		}
		
		options.set_precision(useful_sass_options.precision);
		
		if useful_sass_options.input_syntax == InputSyntax::SASS
		{
			options.set_is_indented_syntax_src();
		}
		
		options.set_include_path(useful_sass_options.include_paths);
		
		useful_sass_options.function_list.set_functions_on_options(options);
		
		useful_sass_options.importer_list.set_importers_on_options(options);
		
		useful_sass_options.header_list.set_headers_on_options(options);
		
		SassContext(data_or_file_context, useful_sass_options.function_list.clone(), useful_sass_options.importer_list.clone(), useful_sass_options.header_list.clone())
	}
	
	/// Compiles SASS data.
	pub fn compile(&self) -> Result<String, SassCompileError>
	{
		self.0.compile();
		let context = self.0.get_context();
		
		let error_status = context.get_error_status();
		if error_status == 0
		{
			if let Some(output_string) = context.get_output_string()
			{
				let css_utf8_string = output_string.to_str()?;
				Ok(css_utf8_string.to_owned())
			}
			else
			{
				panic!("No error, but CSS output string is null");
			}
		}
		else
		{
			if let Some(error_message) = context.get_error_message()
			{
				Err(SassCompileError::Known(error_message.to_owned()))
			}
			else
			{
				Err(SassCompileError::Unknown)
			}
		}
	}
}
