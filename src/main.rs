extern crate quote;
extern crate syn;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use quote::quote;
use syn::{parse_file, Fields, Item, ItemStruct};

fn main() {
    // 1. Specify the file path
    let file_path = "/home/sondq/Documents/substrace/test-parse/src/pallet.rs";
    let new_struct = quote! {
        struct MyStruct {
            field1: i32,
            field2: String,
        }
    };

    // 2. Read the content of the file
    let mut content = String::new();
    File::open(file_path)
        .expect("Failed to open the file")
        .read_to_string(&mut content)
        .expect("Failed to read the file");

    // 3. Parse the content using `syn`
    let mut ast = parse_file(&content).expect("Failed to parse the Rust file");

    // 4. Find the last struct item (if any)
    let mut last_struct_idx = None;
    for (idx, item) in ast.items.iter_mut().enumerate().rev() {
        if let Item::Struct(_) = item {
            last_struct_idx = Some(idx);
            break;
        }
        match item {
            // Item::Struct(v) => {
            //     if let Fields::Named(named_fields) = v.fields.clone() {
            //         for field in &named_fields.named {
            //             let field_name = field.ident.as_ref().unwrap();
            //             let field_type: &syn::Type = &field.ty;
            //             let field_type_string = quote! { #field_type }.to_string();
            //             println!("name: {}, type: {}", field_name, field_type_string);
            //         }
            //     }
            // }
            Item::Mod(m) => {
                // println!("mod {:?}", m.attrs.clone())
                // for attr in m.attrs.clone() {
                //     println!("attr ")
                // }
                if m.ident.to_string() != "pallet" {
                    continue;
                }
                let content = m.content.clone();

                if content.is_none() {
                    continue;
                }

                for (idx, content) in m.content.clone().unwrap().1.iter().enumerate() {
                    match content {
                        Item::Struct(v) => {
                            println!("struct name {}", v.ident.to_string());
                            if let Fields::Named(named_fields) = v.fields.clone() {
                                for field in &named_fields.named {
                                    let field_name = field.ident.as_ref().unwrap();
                                    let field_type: &syn::Type = &field.ty;
                                    let field_type_string = quote! { #field_type }.to_string();
                                    println!("name: {}, type: {}", field_name, field_type_string);
                                }
                            }
                            last_struct_idx = Some(idx);
                        }
                        _ => {}
                    }
                }
                let mut v = content.unwrap();
                if let Some(idx) = last_struct_idx {
                    v.1.insert(
                        idx + 1,
                        Item::Verbatim(new_struct.to_string().parse().unwrap()),
                    );
                } else {
                    v.1.insert(
                        v.1.len(),
                        Item::Verbatim(new_struct.to_string().parse().unwrap()),
                    );
                }

                m.content = Some(v);
            }
            _ => {}
        }
    }

    // 5. Insert the new struct after the last struct (if any)
    // if let Some(idx) = last_struct_idx {
    //     ast.items.insert(
    //         idx + 1,
    //         Item::Verbatim(new_struct.to_string().parse().unwrap()),
    //     );
    // } else {
    //     // If there are no existing structs, add the new struct to the end of the file
    //     ast.items
    //         .push(Item::Verbatim(new_struct.to_string().parse().unwrap()));
    // }

    // 6. Generate the updated Rust code
    // let mut updated_content = String::new();
    let updated_content = quote!(#ast).to_string();

    // 7. Write the updated content back to the file
    let file_output = "/home/sondq/Documents/substrace/test-parse/src/pallet_out.rs";
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_output)
        .expect("Failed to open the file for writing");
    file.write_all(updated_content.as_bytes())
        .expect("Failed to write to the file");

    println!("Struct added successfully!");
}
