use proc_macro2::Span;
use crate::command::OptionMeta;

pub(crate) fn validate_option(meta: &OptionMeta, span: Span) -> Result<(), darling::Error> {
    if meta.description.len() == 0 {
        return Err(syn::Error::new(span, "description is need for option.").into())
    }
    if meta.description.len() > 100 {
        return Err(syn::Error::new(span, format!("description length is longer than limit ({} > 100)", meta.description.len())).into());
    }
    // TODO: name validation
    Ok(())
}
