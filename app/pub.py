import time
import zenoh
import random
from datetime import datetime

import hello_pb2
import metrics_pb2

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
            hello = hello_pb2.HelloMsg(
                msg="hello world",
                counter=i,
                temperature=round(random.uniform(20.0, 30.0), 2),
                timestamp=datetime.utcnow().isoformat(),
            )

            metrics = metrics_pb2.MetricsMsg(
                cpu=round(random.uniform(0, 100), 1),
                mem=round(random.uniform(0, 100), 1),
                timestamp=datetime.utcnow().isoformat(),
            )

            data1 = hello.SerializeToString()     # bytes
            data2 = metrics.SerializeToString()   # bytes

            print(f"[PUB] sending HELLO:\n{hello}")
            pub1.put(data1)

            print(f"[PUB] sending METRICS:\n{metrics}")
            pub2.put(data2)

            i += 1
            time.sleep(1)
