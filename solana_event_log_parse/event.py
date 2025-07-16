from anchorpy import Program, Idl
from solders.pubkey import Pubkey
import base64
import json

async def parse_event():
    # with open("red_packet.json") as f:
    with open("mint_program.json") as f:
        idl = Idl.from_json(f.read())  # 直接传递文件对象
    
    # program_id = Pubkey.from_string("HqSDjxnoR35q8uRMG3LDDvbJ9Hqj4H4bWMPAsBF1hqJq")
    program_id = Pubkey.from_string("6jYBw1mAaH3aJrKEjoacBmNT43MqnTanDBUpiyMX4TN")
    program = Program(idl, program_id)
    
    # event_data = base64.b64decode("Xs7iLxWdjiShzDaypJkWGPKlkF8s+ulolgSTLE9J3bPohqDdyvLQRZeWqxGnamM2a8s9ope4AjuudIfKT0qUc4bBmCuD4JXbgjt/BQAAAAADAAAAAAAAAA==")
    event_data = base64.b64decode("z9SAwq82QBj+3IpPa0HkkpmKfkG3WK8wlTzndXObrCaA7heJvHcd1+oQbIkb9bmKHsDnwJxWUyBnm5Pvct1JlonJ3IzxrTZ9AMqaOwAAAAA=")
    event = program.coder.events.decode(event_data)
    
    print(f"解析成功！事件类型: {type(event).__name__}")
    print("事件详情:", vars(event))  # 显示所有属性

if __name__ == "__main__":
    import asyncio
    asyncio.run(parse_event())