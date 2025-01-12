use attohttpc::header::{HeaderName, HeaderValue};
use attohttpc::RequestBuilder;
use handlebars::*;
use handlebars_misc_helpers::{assign_helpers, json_helpers};
use regex::Captures;
use serde_json::{Map, Value};

use mango3_core::constants::REGEX_HANDLEBARS;

use crate::constants::REGEX_HANDLEBARS_DECLARE;

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

fn append_headers<'rc, T>(helper: &Helper<'rc>, mut request_builder: RequestBuilder<T>) -> RequestBuilder<T>
where
    T: attohttpc::body::Body,
{
    if let Some(headers) = helper.hash_get("headers").and_then(|h| h.value().as_array().clone()) {
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
    if let Ok(response) = request_builder
        .send()
        .map_err(|err| RenderErrorReason::Other(err.to_string()))?
        .json::<Value>()
    {
        if let Some(template) = helper.template() {
            let block_context = create_block(&PathAndJson::new(None, ScopedJson::from(response)));

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

pub fn render_handlebars(input: &str) -> Result<String, RenderError> {
    if !REGEX_HANDLEBARS.is_match(input) {
        return Ok(input.to_owned());
    }

    let mut registry = Handlebars::new();
    let mut data = Map::new();

    registry.set_prevent_indent(true);

    let input = REGEX_HANDLEBARS_DECLARE.replace_all(input, |captures: &Captures| {
        let key = captures.name("key").expect("Could not get match").as_str().to_owned();

        let value = if let Some(value) = captures.name("bool") {
            Value::Bool(value.as_str() == "true")
        } else if let Some(value) = captures.name("number") {
            Value::Number(serde_json::from_str(value.as_str()).unwrap_or_else(|_| 0.into()))
        } else if let Some(value) = captures.name("string") {
            Value::String(value.as_str().to_owned())
        } else if let Some(value) = captures.name("array") {
            Value::Array(serde_json::from_str(value.as_str()).unwrap_or_default())
        } else if let Some(value) = captures.name("object") {
            Value::Object(serde_json::from_str(value.as_str()).unwrap_or_default())
        } else {
            Value::Null
        };

        data.insert(key, value);

        ""
    });

    assign_helpers::register(&mut registry);
    json_helpers::register(&mut registry);

    registry.register_helper("http_get", Box::new(helper_http_get));
    registry.register_helper("http_post", Box::new(helper_http_post));

    registry.render_template(&input, &data)
}
