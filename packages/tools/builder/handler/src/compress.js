import compressing from "compressing";
import { resolve } from "path";

export async function compressBackendFiles() {
  const root = resolve();
  console.log(resolve());
  const src = resolve(root, "dist/backend");
  const dest = resolve(root, "dist/backend.zip");
  await compressing.zip.compressDir(src, dest);
  const dest_tar = resolve(root, "dist/backend.tar");
  await compressing.tar.compressDir(src, dest_tar);
  const dest_gz = resolve(root, "dist/backend.tar.gz");
  await compressing.gzip.compressFile(dest_tar, dest_gz);
}
