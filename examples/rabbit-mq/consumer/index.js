var amqp = require("amqplib/callback_api");

amqp.connect(
  "amqp://guest:guest@localhost:5672",
  (connectionError, connection) => {
    if (connectionError) {
      throw connectionError;
    }

    connection.createChannel((channelError, channel) => {
      if (channelError) {
        throw channelError;
      }

      const queueName = "my-cool-queue";

      channel.assertQueue(queueName, {
        durable: false,
        exclusive: false,
        autoDelete: false,
      });

      console.log(
        `[*] Waiting for messages in ${queueName}. To exit press CTRL+C`
      );

      channel.consume(
        queueName,
        (msg) => {
          console.log(`[x] Received ${msg.content.toString()}`);
        },
        {
          noAck: true,
        }
      );
    });
  }
);
