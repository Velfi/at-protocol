use crate::lexicon_doc::LexiconDoc;
use rust_code_writer::{Crate, Module, Parent, Visibility};
use std::borrow::Cow;
use std::fmt::Write;
use tracing::debug;

pub const INPUT_MODULE: Module = Module {
    name: Cow::Borrowed("input"),
    documentation: Cow::Borrowed("XRPC inputs"),
    parent: Parent::Lib,
    dependencies: vec![],
    visibility: Visibility::Public,
};

pub const OUTPUT_MODULE: Module = Module {
    name: Cow::Borrowed("output"),
    documentation: Cow::Borrowed("XRPC outputs"),
    parent: Parent::Lib,
    dependencies: vec![],
    visibility: Visibility::Public,
};

pub fn write_to_module(c: &mut Crate, doc: &LexiconDoc) -> anyhow::Result<()> {
    write_input_struct(c, doc)?;
    write_input_struct_builder(c, doc)?;
    write_output_struct(c, doc)?;
    write_output_struct_builder(c, doc)?;

    Ok(())
}

// TODO consider using fancy macros to do this instead
fn write_input_struct(c: &mut Crate, doc: &LexiconDoc) -> anyhow::Result<()> {
    debug!("writing input struct for {}", doc.id());

    let mut writer = c.writer(INPUT_MODULE);
    let struct_name = doc.id().as_struct_name();

    // Write docs for the struct
    writeln!(&mut writer, "/// Input for the {struct_name} operation")?;
    if let Some(input_description) = doc.input().and_then(|i| i.description.as_deref()) {
        writeln!(
            &mut writer,
            "/// \\n
             /// {input_description}",
        )?;
    }

    // Write the input struct declaration
    writeln!(&mut writer, "pub struct {struct_name}Input {{")?;

    // If an input struct is defined, write out its fields
    // otherwise, write a comment noting that it's an empty struct
    if let Some(input) = doc.input() {
        for (k, prop) in input.properties().iter() {
            if let Some(description) = prop.description.as_deref() {
                writeln!(&mut writer, "    /// {description}",)?;
            }

            let field_type = if prop.required.unwrap_or_default() {
                format!("Option<{}>", &prop.r#type)
            } else {
                prop.r#type.to_string()
            };
            writeln!(&mut writer, "    pub {k}: {field_type},")?;
        }
    } else {
        writeln!(&mut writer, "    // this input has no fields")?;
    }

    // Write the end of the input struct declaration
    writeln!(&mut writer, "}}\n")?;

    Ok(())
}

fn write_input_struct_builder(_c: &mut Crate, doc: &LexiconDoc) -> anyhow::Result<()> {
    debug!("writing input struct builder for {}", doc.id());
    Ok(())
}

fn write_output_struct(c: &mut Crate, doc: &LexiconDoc) -> anyhow::Result<()> {
    debug!("writing output struct for {}", doc.id());
    let mut writer = c.writer(OUTPUT_MODULE);
    let struct_name = doc.id().as_struct_name();

    // Write docs for the struct
    writeln!(&mut writer, "/// Output for the {struct_name} operation")?;
    if let Some(output_description) = doc.output().and_then(|i| i.description.as_deref()) {
        writeln!(
            &mut writer,
            "/// \\n
             /// {output_description}",
        )?;
    }

    // Write the output struct declaration
    writeln!(&mut writer, "pub struct {struct_name}Output {{")?;

    // If an output struct is defined, write out its fields
    // otherwise, write a comment noting that it's an empty struct
    if let Some(output) = doc.output() {
        for (k, prop) in output.properties().iter() {
            if let Some(description) = prop.description.as_deref() {
                writeln!(&mut writer, "    /// {description}",)?;
            }

            let field_type = if prop.required.unwrap_or_default() {
                format!("Option<{}>", &prop.r#type)
            } else {
                prop.r#type.to_string()
            };
            writeln!(&mut writer, "    pub {k}: {field_type},")?;
        }
    } else {
        writeln!(&mut writer, "    // this output has no fields")?;
    }

    // Write the end of the output struct declaration
    writeln!(&mut writer, "}}\n")?;

    Ok(())
}

fn write_output_struct_builder(_c: &mut Crate, doc: &LexiconDoc) -> anyhow::Result<()> {
    debug!("writing output struct builder for {}", doc.id());
    Ok(())
}
