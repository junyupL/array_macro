extern crate proc_macro;
use proc_macro::Delimiter;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

//for everything inside the macro, whenever there are square brackets with semicolons,
//change array type and array literal syntax:
//[...[[expr; sizeN]; sizeN-1];...;size1] to [size1; ...; sizeN-1; sizeN; expr]

fn parse(custom_array: proc_macro::Group) -> TokenStream {
    let stream = custom_array.stream().to_string();
    let arguments: Vec<&str> = stream.split(';').collect();
    let mut left_side = "".to_owned();
    let mut right_side = "".to_owned();
    
    for i in 0..arguments.len() - 1 {
        let index = arguments.len() - 2 - i;
        left_side.insert(0, '[');
        
        right_side.push_str(format!(";{}]", arguments[index]).as_str());
        
    }

    left_side.push_str(arguments[arguments.len()-1]);
    left_side.push_str(right_side.as_str());


    return left_side.parse().unwrap();
}

#[proc_macro]
pub fn convert_to_custom_array(body: TokenStream) -> TokenStream {
    if body.is_empty() {
        return body;
    }
    //some default value
    let mut last_token = proc_macro::Punct::new('*', proc_macro::Spacing::Alone);
    let mut output = TokenStream::new();
    for tt in body.into_iter() {
        
        match &tt {
            TokenTree::Group(x) => {
                let actual_stream = convert_to_custom_array(x.stream());
                let actual_group = proc_macro::Group::new(x.delimiter(), actual_stream);
                let actual_tt = TokenTree::from(actual_group.clone());
                
                let mut semicolon_in = false;
                for token in x.stream() {
                    match &token {
                        TokenTree::Punct(x) => {
                            if x.as_char() == ';' {
                                semicolon_in = true;
                                break;
                            }
                        }
                        _ => {

                        }
                    }
                }
                if last_token.as_char() != '#' && last_token.as_char() != '!' &&
                x.delimiter() == Delimiter::Bracket && semicolon_in{
                    
                    output.extend(parse(actual_group.clone()));
                    
                }
                else {
                    output.extend(TokenStream::from(actual_tt).into_iter());
                }
                
            },
            TokenTree::Punct(x) => {
                last_token = x.clone();
                output.extend(TokenStream::from(tt.clone()).into_iter());
                
            },
            _ => {
                output.extend(TokenStream::from(tt.clone()).into_iter());
            }
            /*TokenTree::Ident(_) => {
            },
            
            TokenTree::Literal(_) => {
            },*/
        }
        
    }
    return output;
}