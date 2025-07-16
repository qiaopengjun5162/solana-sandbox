import base64

def main():
    print("Hello from solana-event-log-parse!")
  
    # 提取 Base64 部分
    # base64_str = "Xs7iLxWdjiShzDaypJkWGPKlkF8s+ulolgSTLE9J3bPohqDdyvLQRZeWqxGnamM2a8s9ope4AjuudIfKT0qUc4bBmCuD4JXbgjt/BQAAAAADAAAAAAAAAA=="
    base64_str = "z9SAwq82QBj+3IpPa0HkkpmKfkG3WK8wlTzndXObrCaA7heJvHcd1+oQbIkb9bmKHsDnwJxWUyBnm5Pvct1JlonJ3IzxrTZ9AMqaOwAAAAA="
    decoded_bytes = base64.b64decode(base64_str)
    print(decoded_bytes.hex())  # 将解码后的字节转换为十六进制表示，便于查看


if __name__ == "__main__":
    main()
