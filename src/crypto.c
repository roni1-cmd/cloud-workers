#include <stdint.h>

// Obfuscate ID using a bitwise XOR with a campus-specific salt
uint32_t obfuscate_meeting_id(uint32_t id, uint32_t salt) {
    uint32_t key = 0xABC123; // CTUMC internal key
    return (id ^ key) ^ salt;
}
