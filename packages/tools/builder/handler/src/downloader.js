import got from "got";
import { promisify } from "node:util";
import { createWriteStream } from "node:fs";
import { resolve } from "node:path";
import { pipeline as pipeline_callback } from "node:stream";

const pipeline = promisify(pipeline_callback);

export async function downloadMongoDbServer() {
  const url =
    "https://fastdl.mongodb.org/windows/mongodb-windows-x86_64-5.0.9-signed.msi";
  const dest = resolve("dist/backend/mongodb-windows-x86_64-5.0.9-signed.msi");
  const stream = got.stream(url);
  const write_stream = createWriteStream(dest);
  let show = false;
  let down = setInterval(() => (show = true), 1000);
  stream.on("downloadProgress", (prog) => {
    if (show) {
      console.log(
        `Downloading mongodb progress in: ${prog.percent.toFixed(2)}%`
      );
      show = false;
    }
  });
  stream.on("close", () => {
    console.log("Ok");
    clearInterval(down);
  });
  await pipeline(stream, write_stream);
}
