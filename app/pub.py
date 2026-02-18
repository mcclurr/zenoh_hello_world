import time
import zenoh
import json
import random
from datetime import datetime

KEY = "demo/hello"

if __name__ == "__main__":
    conf = zenoh.Config.from_json5("""
    {
      mode: "client",
      connect: { endpoints: ["tcp/zenoh:7447"] }
    }
    """)

    with zenoh.open(conf) as session:
        pub = session.declare_publisher(KEY)

        i = 0
        while i < 10:
            payload = {
                "msg": "hello world",
                "counter": i,
                "temperature": round(random.uniform(20.0, 30.0), 2),
                "timestamp": datetime.utcnow().isoformat()
            }

            data = json.dumps(payload)

            print(f"[PUB] sending: {data}")
            pub.put(data)

            i += 1
            time.sleep(1)
