FROM python:3.11-slim

RUN pip install --no-cache-dir eclipse-zenoh protobuf grpcio-tools

WORKDIR /app

COPY app/pub.py app/sub.py /app/
COPY protos /app/protos

RUN python -m grpc_tools.protoc \
    -I /app/protos \
    --python_out=/app \
    /app/protos/hello.proto /app/protos/metrics.proto
