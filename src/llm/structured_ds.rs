use anyhow::Ok;
use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs, ResponseFormat,
};
use schemars::schema_for;

use crate::models::action_plan::ActionPlan;

pub async fn chat_complete_structured_ds(
    model: &str,
    // system: Option<&str>,
    prompt: &str,
) -> anyhow::Result<ActionPlan> {
    let client = async_openai::Client::new();
    let mut messages = vec![];

    messages.push(
        ChatCompletionRequestSystemMessageArgs::default()
            .content(build_system_prompt())
            .build()?
            .into(),
    );

    messages.push(
        ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()?
            .into(),
    );

    // let schema = schemars::schema_for!(ActionPlan);
    // let schema_json = schema.as_value().clone();
    let format_setting = ResponseFormat::JsonObject;
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages)
        .response_format(format_setting)
        .max_tokens(2048u32)
        .build()?;
    //  tracing::info!("requesting");
    let response = client.chat().create(request).await?;

    tracing::info!("Response {:#?}", response);

    let plan: ActionPlan = response
        .choices
        .into_iter()
        .next()
        .and_then(|c| c.message.content)
        .ok_or_else(|| anyhow::anyhow!("No content in response"))
        .and_then(|s| serde_json::from_str(&s).map_err(Into::into))?;

    Ok(plan)
}

fn build_system_prompt() -> String {
    let schema = schema_for!(ActionPlan);
    let schema_str = serde_json::to_string_pretty(&schema).unwrap();

    format!(
        r#"你是一个全能型智能助手和行动规划助手。

用户会提出技术、学习、旅行、生活、工作、问题排查或其他类型的需求。
你需要理解用户的目标，并将其转换为清晰、可执行的行动计划。

你必须只返回一个合法的 JSON 对象，不要返回 Markdown、代码块、
解释、标题、注释、<think> 标签或任何 JSON 之外的内容。

JSON 结构必须是：
    {schema_str}

字段要求：
- goal：一句话概括用户目标。
- steps：按执行顺序生成步骤。
- index：整数，从 1 开始连续递增。
- description：具体、清晰、可执行。
- tool_hint：需要工具时填写字符串，否则返回 null。
- difficulty：只能是 Easy、Medium 或 Hard。
- estimated_minutes：非负整数，表示预计完成分钟数。

不要增加其他字段，不要遗漏任何顶层字段，不要把数字写成字符串。"#
    )
}
