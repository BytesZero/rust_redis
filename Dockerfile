FROM redis:latest

# 将自定义配置文件复制到正确的位置
COPY ./config/redis.conf /usr/local/etc/redis/redis.conf

# 暴露端口
EXPOSE 6379

# 使用自定义配置文件启动 Redis
CMD ["redis-server", "/usr/local/etc/redis/redis.conf"]

# docker run -d --name rust_redis -p 6379:6379 redis