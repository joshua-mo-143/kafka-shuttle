sql:
	docker exec -it shuttle_kafka-shuttle_shared_postgres psql -U postgres -d kafka-shuttle -c 'select * from messages';
create:
	curl localhost:8000/send -H 'Content-Type: application/json' \
		-d '{"action":"Create","data":{"message_id":4,	"name":"Josh","message":"Hello world!"}}'

update:
	curl localhost:8000/send -H 'Content-Type: application/json' \
		-d '{"action":"Update","data":{"message_id":1,"name":"Josh","message":"Hey there!"}}'

delete:
	curl localhost:8000/send -H 'Content-Type: application/json' \
		-d '{"action":"Delete","data":{"message_id":1,"name":"Josh","message":"Hey there!"}}'

seed-kafka:
	./scripts/kafka.sh
