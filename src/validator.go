package main

import "unsafe"

//export validate_header
func validate_header(ptr *byte, length int) bool {
	// Low-level slice header creation
	buf := (*[1 << 30]byte)(unsafe.Pointer(ptr))[:length:length]
	
	// Check for PDF signature (%PDF-)
	if length > 4 && string(buf[:5]) == "%PDF-" {
		return true
	}
	return false
}

func main() {}
