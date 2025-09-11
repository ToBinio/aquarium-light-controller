import type { EventStream } from "h3";
import type { Updates } from "~~/shared/updates";

const updateEventStreams: EventStream[] = [];

export const UpdateEventStream = {
  addStream(eventStream: EventStream) {
    updateEventStreams.push(eventStream);

    eventStream.onClosed(async () => {
      updateEventStreams.splice(updateEventStreams.indexOf(eventStream), 1);
      await eventStream.close();
    });
  },
  async sendUpdate(data: Updates) {
    const json = JSON.stringify(data);

    updateEventStreams.forEach((eventStream) => {
      eventStream.push(json);
    });
  },
};
