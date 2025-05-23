use attohttpc::header::{HeaderName, HeaderValue};
use attohttpc::RequestBuilder;
use handlebars::{
    BlockContext, Context, Handlebars, Helper, HelperResult, Output, PathAndJson, RenderContext, RenderError,
    RenderErrorReason, Renderable, ScopedJson,
};
use handlebars_misc_helpers::{assign_helpers, json_helpers, string_helpers};
use serde_json::Value;

use crate::constants::REGEX_HANDLEBARS;

#[macro_export]
macro_rules! render_handlebars {
    ($input:expr, $data:expr) => {
        $crate::utils::render_handlebars($input, $data)
    };
}

fn create_block<'rc>(param: &PathAndJson<'rc>) -> BlockContext<'rc> {
    let mut block = BlockContext::new();

    if let Some(new_path) = param.context_path() {
        block.base_path_mut().clone_from(new_path);
    } else {
        // use clone for now
        block.set_base_value(param.value().clone());
    }

    block
}

fn param_url<'rc>(helper: &'rc Helper<'rc>) -> Result<&'rc str, RenderErrorReason> {
    helper
        .param(0)
        .and_then(|param| param.value().as_str())
        .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("fetch", 0))
}

fn append_headers<T>(helper: &Helper<'_>, mut request_builder: RequestBuilder<T>) -> RequestBuilder<T>
where
    T: attohttpc::body::Body,
{
    if let Some(headers) = helper.hash_get("headers").and_then(|h| h.value().as_array()) {
        for header in headers {
            if let Some(header_str) = header.as_str() {
                if let Some((key, value)) = header_str.split_once(':') {
                    let Ok(key) = key.parse::<HeaderName>() else {
                        continue;
                    };

                    let Ok(value) = value.parse::<HeaderValue>() else {
                        continue;
                    };

                    request_builder = request_builder.header_append(key, value);
                }
            }
        }
    }

    request_builder
}

fn push_response<'reg, 'rc, T>(
    helper: &Helper<'rc>,
    registry: &'reg Handlebars,
    context: &'rc Context,
    render_context: &mut RenderContext<'reg, 'rc>,
    output: &mut dyn Output,
    request_builder: RequestBuilder<T>,
) -> HelperResult
where
    T: attohttpc::body::Body,
{
    if let Ok(text) = request_builder
        .send()
        .map_err(|err| RenderErrorReason::Other(err.to_string()))?
        .text()
    {
        if let Some(template) = helper.template() {
            let value = serde_json::from_str::<Value>(&text).unwrap_or(Value::String(text));
            let block_context = create_block(&PathAndJson::new(None, ScopedJson::from(value)));

            render_context.push_block(block_context);

            template.render(registry, context, render_context, output)?;

            render_context.pop_block();
        }
    }

    Ok(())
}

fn helper_http_get<'reg, 'rc>(
    helper: &Helper<'rc>,
    registry: &'reg Handlebars,
    context: &'rc Context,
    render_context: &mut RenderContext<'reg, 'rc>,
    output: &mut dyn Output,
) -> HelperResult {
    let url = param_url(helper)?;

    let mut request_builder = attohttpc::get(url);

    request_builder = append_headers(helper, request_builder);

    push_response(helper, registry, context, render_context, output, request_builder)
}

fn helper_http_post<'reg, 'rc>(
    helper: &Helper<'rc>,
    registry: &'reg Handlebars,
    context: &'rc Context,
    render_context: &mut RenderContext<'reg, 'rc>,
    output: &mut dyn Output,
) -> HelperResult {
    let url = param_url(helper)?;

    let data = helper
        .hash_get("data")
        .map(|b| b.value())
        .unwrap_or_else(|| &Value::Null);

    let mut request_builder = attohttpc::post(url)
        .json(data)
        .map_err(|err| RenderErrorReason::Other(err.to_string()))?;

    request_builder = append_headers(helper, request_builder);

    push_response(helper, registry, context, render_context, output, request_builder)
}

pub fn render_handlebars(input: &str, data: &Value) -> Result<String, RenderError> {
    if !REGEX_HANDLEBARS.is_match(input) {
        return Ok(input.to_owned());
    }

    let mut registry = Handlebars::new();

    registry.set_prevent_indent(true);

    assign_helpers::register(&mut registry);
    json_helpers::register(&mut registry);
    string_helpers::register(&mut registry);

    registry.register_helper("http_get", Box::new(helper_http_get));
    registry.register_helper("http_post", Box::new(helper_http_post));

    registry.render_template(input, data)
}
