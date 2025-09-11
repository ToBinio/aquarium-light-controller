import type { Updates } from "~~/shared/updates";
import { UpdateEventStream } from "../utils/see";

export default defineEventHandler(async (event) => {
  const body = (await readBody(event)) as Updates;
  UpdateEventStream.sendUpdate(body);
});
