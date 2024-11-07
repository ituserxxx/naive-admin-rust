#!/bin/bash

# 配置参数
HOST="148.70.9.106"
PORT="1883"
TOPIC="test/topic"
MESSAGE="Hello, MQTT"
CLIENTS=10000000
PUBLISH_INTERVAL=100   # 每秒发布一条消息
DURATION=120          # 持续 60 秒

# 启动多个并发客户端，发布消息
for i in $(seq 1 $CLIENTS); do
  mosquitto_pub -h $HOST -p $PORT -t $TOPIC -m "$MESSAGE" -q 1 -d &
  sleep 0.01  # 确保并发启动
done

# 等待压力测试完成
sleep $DURATION
echo "Test completed!"
