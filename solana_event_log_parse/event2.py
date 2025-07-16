import asyncio
import base64
from solders.pubkey import Pubkey
from anchorpy import Program, Idl

async def parse_solana_event():
    """
    使用正确的 IDL 文件来解析 Solana 事件日志。
    """
    try:
        # 1. 加载刚刚生成的、正确的 IDL 文件
        with open("mint_program.json") as f:
            idl = Idl.from_json(f.read())
            print("✅ IDL 文件加载成功！")

        # 2. 设置您的合约程序 ID
        program_id = Pubkey.from_string("6jYBw1mAaH3aJrKEjoacBmNT43MqnTanDBUpiyMX4TN")
        
        # 3. 初始化 Anchor Program 对象
        program = Program(idl, program_id)

        # 4. 从您的日志中提取 Base64 编码的事件数据
        #    这个数据对应的是 TokensMinted 事件
        b64_event_data = "z9SAwq82QBj+3IpPa0HkkpmKfkG3WK8wlTzndXObrCaA7heJvHcd1+oQbIkb9bmKHsDnwJxWUyBnm5Pvct1JlonJ3IzxrTZ9AMqaOwAAAAA="
        
        # 5. 解码事件
        event_data_bytes = base64.b64decode(b64_event_data)
        event = program.coder.events.decode(event_data_bytes)

        if event:
            print(f"\n🎉 事件解析成功！")
            print(f"   - 事件名称 (Event Name): {event.name}")
            print(f"   - 事件数据 (Event Data):")
            # 打印事件的所有字段和值
            for field, value in event.data.__dict__.items():
                print(f"     - {field}: {value}")
        else:
            print("❌ 事件解析失败，未找到匹配的事件类型。")

    except FileNotFoundError:
        print("❌ 错误：找不到 'mint_program.json'。请确保您已将新生成的 IDL 文件复制到当前目录。")
    except Exception as e:
        print(f"❌ 解析过程中发生错误: {e}")

if __name__ == "__main__":
    asyncio.run(parse_solana_event())