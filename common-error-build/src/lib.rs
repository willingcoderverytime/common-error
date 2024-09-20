use std::error::Error;
use std::fs;
use std::path::Path;

use quote::__private::TokenStream;
use quote::quote;
use regex::Regex;
use syn::__private::Span;
use syn::Ident;

use crate::read_csv::{ErrorInfo, read_csv_with_serde};

pub fn error_build() {
    let dest_path = Path::new("src").join("_common_error.rs");

    if dest_path.exists() {
        fs::remove_file(dest_path.clone()).unwrap();
    }


    let vec = match read_csv_with_serde("common-error.csv") {
        Ok(re) => re,
        Err(e) => {
            eprintln!("red file error {}", e);
            return;
        }
    };

    let mut file_content = String::from("extern crate common_error; \n \n");
    let temp_reg = Regex::new(r"(\{[^{}]*\})").unwrap();
    let arg_reg = Regex::new(r"([a-z_]+-)+[a-z_]+").unwrap();

    file_content.push_str("\n");
    for x in vec {
        let string = expanded_error(x, &temp_reg, &arg_reg).to_string();
        file_content.push_str(string.as_str());
        file_content.push_str("\n")
    }
    fs::write(&dest_path, file_content).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let re = Regex::new(r"(\{[^{}]*\})").unwrap();
        ;
        let result = Regex::new(r"([a-z_]+-)+[a-z_]+").unwrap();
        let template = "{}{}";
        let args = "a-b";
        assert!(result.is_match(args));
        let xxx: Vec<&str> = args.split("-").collect();
        assert!(re.split(template).into_iter().count() - 1 == xxx.len())
    }
}


fn expanded_error(recode: ErrorInfo, temp_reg: &Regex, args_reg: &Regex) -> TokenStream {
    let ErrorInfo { code, template, args, desc } = recode;
    if !args_reg.is_match(args.as_str()) {
        eprintln!("error code [{}]  args wrong: {}", code, args);
    }
    let args: Vec<&str> = args.split("-").collect();
    let i = temp_reg.find_iter(template.as_str()).count();
    if i != args.len() {
        eprintln!("error code [{}]  args num no match template", code);
    }

    let templates: Vec<&str> = temp_reg.split(template.as_str()).collect();
    expand_args_template(code, desc, templates, args)
}


fn expand_args_template(code: String, desc: String, templates: Vec<&str>, args: Vec<&str>) -> TokenStream {
    let code_str = code.to_string();
    let code_ident = Ident::new(code.as_str(), Span::call_site());

    if args.len() == 1 {
        let temp0 = *templates.get(0).unwrap();
        let temp1 = *templates.get(1).unwrap();
        let arg0 = Ident::new(*args.get(0).unwrap(), Span::call_site());
        return quote!(
             #[doc = #desc]
            pub struct #code_ident;
            impl  #code_ident {
                pub fn throw(#arg0 :&str)->(common_error::CommonError,String){
                     let msg = #temp0.to_string() + #arg0 + #temp1;
                     let common_error = common_error::CommonError{code:#code_str.to_string(),message:msg.clone()};
                     (common_error,msg)
                }
            }

        );
    }

    if args.len() == 2 {
        let temp0 = *templates.get(0).unwrap();
        let temp1 = *templates.get(1).unwrap();
        let temp2 = *templates.get(2).unwrap();
        let arg0 = Ident::new(*args.get(0).unwrap(), Span::call_site());
        let arg1 = Ident::new(*args.get(1).unwrap(), Span::call_site());
        return quote!(
             #[doc = #desc]
            pub struct #code_ident;
            impl  #code_ident {
                pub fn throw(#arg0 :&str,#arg1 :&str)->(common_error::CommonError,String){
                     let msg = #temp0.to_string() + #arg0 + #temp1 + #arg1 + #temp2;
                     let common_error = common_error::CommonError{code:#code_str.to_string(),message:msg.clone()};
                     (common_error,msg)
                }
            }

        );
    }


    if args.len() == 3 {
        let temp0 = *templates.get(0).unwrap();
        let temp1 = *templates.get(1).unwrap();
        let temp2 = *templates.get(2).unwrap();
        let temp3 = *templates.get(3).unwrap();
        let arg0 = Ident::new(*args.get(0).unwrap(), Span::call_site());
        let arg1 = Ident::new(*args.get(1).unwrap(), Span::call_site());
        let arg2 = Ident::new(*args.get(2).unwrap(), Span::call_site());
        return quote!(
             #[doc = #desc]
            pub struct #code_ident;
            impl  #code_ident {
                pub fn throw(#arg0 :&str,#arg1 :&str ,#arg2 :&str)->(common_error::CommonError,String){
                     let msg = #temp0.to_string() + #arg0 + #temp1 + #arg1 + #temp2 + #arg2 + #temp3;
                     let common_error = common_error::CommonError{code:#code_str.to_string(),message:msg.clone()};
                     (common_error,msg)
                }
            }

        );
    }


    if args.len() == 4 {
        let temp0 = *templates.get(0).unwrap();
        let temp1 = *templates.get(1).unwrap();
        let temp2 = *templates.get(2).unwrap();
        let temp3 = *templates.get(3).unwrap();
        let temp4 = *templates.get(4).unwrap();
        let arg0 = Ident::new(*args.get(0).unwrap(), Span::call_site());
        let arg1 = Ident::new(*args.get(1).unwrap(), Span::call_site());
        let arg2 = Ident::new(*args.get(2).unwrap(), Span::call_site());
        let arg3 = Ident::new(*args.get(3).unwrap(), Span::call_site());
        return quote!(
              #[doc = #desc]
            pub struct #code_ident;
            impl  #code_ident {
                pub fn throw(#arg0 :&str,#arg1 :&str ,#arg2 :&str,#arg3 :&str)->(common_error::CommonError,String){
                     let msg = #temp0.to_string() + #arg0 + #temp1 + #arg1 + #temp2 + #arg2 + #temp3 + #arg3 + #temp4;
                     let common_error = common_error::CommonError{code:#code_str.to_string(),message:msg.clone()};
                     (common_error,msg)
                }
            }

        );
    }


    if args.len() == 5 {
        let temp0 = *templates.get(0).unwrap();
        let temp1 = *templates.get(1).unwrap();
        let temp2 = *templates.get(2).unwrap();
        let temp3 = *templates.get(3).unwrap();
        let temp4 = *templates.get(4).unwrap();
        let temp5 = *templates.get(5).unwrap();
        let arg0 = Ident::new(*args.get(0).unwrap(), Span::call_site());
        let arg1 = Ident::new(*args.get(1).unwrap(), Span::call_site());
        let arg2 = Ident::new(*args.get(2).unwrap(), Span::call_site());
        let arg3 = Ident::new(*args.get(3).unwrap(), Span::call_site());
        let arg4 = Ident::new(*args.get(4).unwrap(), Span::call_site());
        return quote!(
          #[doc = #desc]
            pub struct #code_ident;
            impl  #code_ident {
                pub fn throw(#arg0 :&str,#arg1 :&str ,#arg2 :&str,#arg3 :&str,#arg4 :&str)->(common_error::CommonError,String){
                     let msg = #temp0.to_string() + #arg0 + #temp1 + #arg1 + #temp2 + #arg2 + #temp3 + #arg3 + #temp4+ #arg4 + #temp5;
                     let common_error = common_error::CommonError{code:#code_str.to_string(),message:msg.clone()};
                     (common_error,msg)
                }
            }

        );
    }

    let temp0 = *templates.get(0).unwrap();
    // == 0 的情况
    quote!(
        #[doc = #desc]
        pub struct #code_ident;
        impl  #code_ident {
            pub fn throw()->(common_error::CommonError,String){
                 let common_error = common_error::CommonError{code:#code_str.to_string(),message:#temp0};
                 (common_error,#temp0)
            }
         }

     )
}

mod read_csv {
    use std::error::Error;
    use std::fs::File;

    use csv::Reader;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub(crate) struct ErrorInfo {
        pub code: String,
        pub desc: String,
        pub template: String,
        pub args: String,
    }

    pub(crate) fn read_csv_with_serde(file_path: &str) -> Result<Vec<ErrorInfo>, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let mut rdr = Reader::from_reader(file);
        let mut vec = Vec::new();
        for result in rdr.deserialize() {
            let record: ErrorInfo = result?;
            vec.push(record);
        }
        Ok(vec)
    }
}

