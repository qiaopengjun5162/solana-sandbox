use anyhow::Result;
use anyhow::anyhow;
use borsh::{BorshDeserialize, BorshSerialize};
use serde_json::Value;
use std::error::Error;

// 定义与程序相同的 GreetingEvent 结构体
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct GreetingEvent {
    pub message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder().build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let data = r#"
    {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBlock",
        "params": [
            387787294,
            {
                "encoding": "jsonParsed",
                "maxSupportedTransactionVersion": 0,
                "transactionDetails": "full",
                "rewards": false
            }
        ]
    }
    "#;

    let json: Value = serde_json::from_str(&data)?;
    let request = client
        .request(
            reqwest::Method::POST,
            "https://docs-demo.solana-devnet.quiknode.pro/",
        )
        .headers(headers)
        .json(&json);

    let response = request.send().await?;
    let status = response.status();
    let response_text = response.text().await?;
    println!("Status: {}", status);
    let body: Value = serde_json::from_str(&response_text)?;

    let target_program_id = "GGBjDqYdicSE6Qmtu6SAsueX1biM5LjbJ8R8vZvFfofA";
    // 提取 logMessages
    let log_messages = extract_log_messages(&body, target_program_id)?;

    // 打印 logMessages
    println!("logMessages: {:#?}", log_messages);

    // 解析 EVENT:GREETING 日志
    for log in &log_messages {
        if log.contains("EVENT:GREETING") {
            let event = parse_greeting_event(log)?;
            println!("Parsed GreetingEvent: {:?}", event);
        }
    }

    Ok(())
}

fn extract_log_messages(body: &Value, target_program_id: &str) -> Result<Vec<String>> {
    // 获取 transactions 数组
    let transactions = body["result"]["transactions"]
        .as_array()
        .ok_or_else(|| anyhow!("结果中未找到 transactions"))?;

    // 遍历交易，找到调用目标程序的交易
    for tx in transactions {
        // 检查指令中是否包含目标 programId
        let instructions = tx["transaction"]["message"]["instructions"]
            .as_array()
            .ok_or_else(|| anyhow!("交易中未找到指令"))?;

        let has_target_program = instructions.iter().any(|instruction| {
            instruction["programId"]
                .as_str()
                .map_or(false, |pid| pid == target_program_id)
        });

        if has_target_program {
            // 提取 logMessages
            let log_messages = tx["meta"]["logMessages"]
                .as_array()
                .ok_or_else(|| anyhow!("meta 中未找到 logMessages"))?;
            // 过滤与目标程序相关的日志
            let filtered_logs: Vec<String> = log_messages
                .iter()
                .filter_map(|log| {
                    log.as_str()
                        .filter(|s| s.contains(target_program_id) || s.contains("Program log"))
                        .map(String::from)
                })
                .collect();

            if !filtered_logs.is_empty() {
                return Ok(filtered_logs);
            }
        }
    }

    Err(anyhow!("未找到调用程序 {} 的日志", target_program_id))
}

fn parse_greeting_event(log: &str) -> Result<GreetingEvent> {
    // 提取 [26, 0, 0, 4, ...] 部分
    let start = log
        .find('[')
        .ok_or_else(|| anyhow!("无效的事件日志格式：未找到 '['"))?;
    let end = log
        .find(']')
        .ok_or_else(|| anyhow!("无效的事件日志格式：未找到 ']'"))?;
    let bytes_str = &log[start + 1..end];

    // 将字符串中的数字转换为 Vec<u8>
    let bytes: Vec<u8> = bytes_str
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u8>().map_err(|e| anyhow!("无法解析字节：{}", e)))
        .collect::<Result<Vec<u8>>>()?;

    // 使用 Borsh 解序列化
    let event =
        GreetingEvent::try_from_slice(&bytes).map_err(|e| anyhow!("Borsh 解序列化失败：{}", e))?;

    Ok(event)
}
