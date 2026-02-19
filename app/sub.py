import time
import zenoh
import hello_pb2

KEY = "demo/hello"

def listener(sample):
    b = sample.payload.to_bytes()  # raw bytes
    msg = hello_pb2.HelloMsg()
    msg.ParseFromString(b)
    print(f"[SUB] {sample.kind} {sample.key_expr} msg={msg.msg} counter={msg.counter} temp={msg.temperature} ts={msg.timestamp}")

if __name__ == "__main__":
    conf = zenoh.Config.from_json5("""
    {
      mode: "client",
      connect: { endpoints: ["tcp/zenoh:7447"] }
    }
    """)

    with zenoh.open(conf) as session:
        session.declare_subscriber(KEY, listener)
        print(f"[SUB] listening on {KEY} ...")
        while True:
            time.sleep(3600)
