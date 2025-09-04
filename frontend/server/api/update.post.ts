import { Light } from "~~/shared/light";
import { UpdateEventStream } from "../utils/see";

export default defineEventHandler(async (event) => {
  const body = (await readBody(event)) as Light;
  UpdateEventStream.sendUpdate(body);
});
