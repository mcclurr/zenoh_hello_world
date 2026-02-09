import time
import zenoh

KEY = "demo/hello"

if __name__ == "__main__":
    # Docker multicast scouting often doesn't work, so connect explicitly to the router.
    conf = zenoh.Config.from_json5("""
    {
      mode: "client",
      connect: { endpoints: ["tcp/zenoh:7447"] }
    }
    """)

    with zenoh.open(conf) as session:
        pub = session.declare_publisher(KEY)
        i = 0
        while True:
            msg = f"hello {i}"
            print(f"[PUB] putting {KEY} = {msg}")
            pub.put(msg)
            i += 1
            time.sleep(1)
