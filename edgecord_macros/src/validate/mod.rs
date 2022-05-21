use crate::command::OptionMeta;
use proc_macro2::Span;

pub(crate) fn validate_option(meta: &OptionMeta, span: Span) -> Result<(), darling::Error> {
    if meta.description.is_empty() {
        return Err(syn::Error::new(span, "description is need for option.").into());
    }
    if meta.description.len() > 100 {
        return Err(syn::Error::new(
            span,
            format!(
                "description length is longer than limit ({} > 100)",
                meta.description.len()
            ),
        )
        .into());
    }
    // TODO: name validation
    Ok(())
}
