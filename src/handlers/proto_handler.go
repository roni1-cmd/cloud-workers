package main

import (
	"github.com/golang/protobuf/proto"
)

//export serialize_meeting
func serialize_meeting(title string, duration int64) *byte {
	record := &MeetingRecord{
		Title: title,
		TotalDurationMs: duration,
	}
	data, _ := proto.Marshal(record)
	return &data[0]
}
