import time
import zenoh

KEY = "demo/hello"

def listener(sample):
    # sample.payload.to_string() works in the docs examples
    print(f"[SUB] {sample.kind} {sample.key_expr} = {sample.payload.to_string()}")

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
