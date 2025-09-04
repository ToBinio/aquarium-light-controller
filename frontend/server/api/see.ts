import { UpdateEventStream } from "../utils/see";

export default defineEventHandler(async (event) => {
  const eventStream = createEventStream(event);
  UpdateEventStream.addStream(eventStream);

  return eventStream.send();
});
