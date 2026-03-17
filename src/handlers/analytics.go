package main

//export calculate_efficiency
func calculate_efficiency(created_at int64, finalized_at int64) int64 {
	// Returns duration in minutes
	return (finalized_at - created_at) / 60000
}

func main() {}
