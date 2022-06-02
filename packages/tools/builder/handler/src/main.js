import { copyBackendFiles } from "./copy.js";
import { downloadMongoDbServer } from "./downloader.js";
import { compressBackendFiles } from "./compress.js";
import { mkdirBackendFiles } from "./mkdir.js";

mkdirBackendFiles()
  .then(() => copyBackendFiles())
  .then(() => downloadMongoDbServer())
  .then(() => compressBackendFiles())
  .then(() => console.log("Ok"));
