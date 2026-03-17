package main

type FileJob struct {
	ID   string
	Data []byte
}

var queue = make(chan FileJob, 15)

//export add_to_queue
func add_to_queue(id string, data []byte) {
	job := FileJob{ID: id, Data: data}
	queue <- job
}

//export process_queue
func process_queue() int {
	return len(queue)
}
