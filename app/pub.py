import time
import zenoh
import json
import random
from datetime import datetime

KEY1 = "demo/hello"
KEY2 = "demo/metrics"

if __name__ == "__main__":
    conf = zenoh.Config.from_json5("""
    {
      mode: "client",
      connect: { endpoints: ["tcp/zenoh:7447"] }
    }
    """)

    with zenoh.open(conf) as session:
        pub1 = session.declare_publisher(KEY1)
        pub2 = session.declare_publisher(KEY2)

        i = 0
        while i < 100:
            payload1 = {
                "msg": "hello world",
                "counter": i,
                "temperature": round(random.uniform(20.0, 30.0), 2),
                "timestamp": datetime.utcnow().isoformat()
            }

            payload2 = {
                "cpu": round(random.uniform(0, 100), 1),
                "mem": round(random.uniform(0, 100), 1),
                "timestamp": datetime.utcnow().isoformat(),
            }

            data1 = json.dumps(payload1)
            data2 = json.dumps(payload2)

            print(f"[PUB] sending: {data1}")
            pub1.put(data1)
            print(f"[PUB] sending: {data2}")
            pub2.put(data2)

            i += 1
            time.sleep(1)
