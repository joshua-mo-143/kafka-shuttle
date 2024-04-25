### Shuttle Kafka example
This repository shows how you can use Kafka with Shuttle. Note that Shuttle itself doesn't supply Kafka - you need to find a third party service yourself.

### Usage
How to use:
1) Use `cargo shuttle init --from joshua-mo-143/kafka-shuttle` and follow the prompt
2) Set up a Kafka account somewhere, get the credentials and adjust your FutureProducer/StreamConsumer creation setup as required. An example for using Upstash's Kafka can be found in `kafka.rs`. You will likely need to set the topic name as "messages" unless you change the topic name.
3) Use `cargo shuttle deploy --ad` and you're ready to go!

