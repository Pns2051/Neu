#pragma once
#include <string>
#include <vector>
#include <algorithm>

enum class OpType {
    Reverse,
    Slice,
    Swap
};

struct Op {
    OpType type;
    int value;
};

class YouTubeCipher {
public:
    static std::string decipher(std::string sig, const std::vector<Op>& ops) {
        for (const auto& op : ops) {
            switch (op.type) {
                case OpType::Reverse:
                    std::reverse(sig.begin(), sig.end());
                    break;
                case OpType::Slice:
                    if (static_cast<size_t>(op.value) < sig.length()) {
                        sig.erase(0, op.value);
                    }
                    break;
                case OpType::Swap:
                    if (!sig.empty() && static_cast<size_t>(op.value) < sig.length()) {
                        std::swap(sig[0], sig[op.value]);
                    }
                    break;
            }
        }
        return sig;
    }
    
    static std::vector<Op> parseJs(const std::string& /*js*/) {
        // Advanced parsing logic would go here
        return { {OpType::Reverse, 0}, {OpType::Slice, 1}, {OpType::Swap, 2} };
    }
};
