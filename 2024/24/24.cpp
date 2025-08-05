#include <cassert>
#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <set>
#include <string_view>
#include <unistd.h>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

using namespace std;

enum GateType {
    AND,
    OR,
    XOR,
    NONE
};

GateType parse_gate_type(string_view type)
{
    if (type == "AND") {
        return GateType::AND;
    } else if (type == "OR") {
        return GateType::OR;
    } else if (type == "XOR") {
        return GateType::XOR;
    } else {
        return GateType::NONE;
    }
}

string gate_type_str(GateType type)
{
    if (type == AND) {
        return "AND";
    } else if (type == OR) {
        return "OR";
    } else if (type == XOR) {
        return "XOR";
    } else {
        return "NONE";
    }
}

struct Gate {
    int id;
    string in1, in2, out;
    GateType type;
    bool processed;
};

void print_gate(const Gate& gate) {
    printf("%s %s %s -> %s\n",
            gate.in1.c_str(),
            gate_type_str(gate.type).c_str(),
            gate.in2.c_str(),
            gate.out.c_str());
}

unordered_map<string, uint8_t> read_signals_input()
{
    char name[128];
    int val;

    char* line = new char[1024];
    size_t size;
    unordered_map<string, uint8_t> signals;
    while (true) {

        assert(getline((char**)&line, &size, stdin));
        if (line[0] == '\n')
            break;
        sscanf(line, "%127[^:]: %d", name, &val);

        signals[name] = val;
    }

    delete[] line;
    return signals;
}

vector<Gate> read_gates_input()
{
    char sin1[128], sin2[128];
    char type[4];
    char sout[128];
    char sep[4];

    vector<Gate> gates;

    int id = 1;
    while (true) {
        if (scanf("%s %s %s %s %s", sin1, type, sin2, sep, sout) != 5)
            break;
        gates.push_back(Gate {
                .id = id++,
                .in1 = sin1,
                .in2 = sin2,
                .out = sout,
                .type = parse_gate_type(type),
                .processed = false
            });
    }

    return gates;
}

int process_gate(Gate& gate, unordered_map<string, uint8_t>& signals)
{
    switch (gate.type) {
    case GateType::AND:
        signals[gate.out] = signals[gate.in1] & signals[gate.in2];
        break;
    case GateType::OR:
        signals[gate.out] = signals[gate.in1] | signals[gate.in2];
        break;
    case GateType::XOR:
        signals[gate.out] = signals[gate.in1] ^ signals[gate.in2];
        break;
    default:
        return 1;
    }

    gate.processed = true;
    return 0;
}

template <typename T, typename U>
bool inline contains(unordered_map<T, U>& umap, const T& key)
{
    return umap.find(key) != cend(umap);
}

uint64_t register_value(char reg_prefix, const unordered_map<string, uint8_t>& signals)
{
    uint64_t reg = 0;
    for (auto& [name, signal] : signals) {
        if (name[0] == reg_prefix) {
            int offset;
            sscanf(name.c_str() + 1, "%d", &offset);
            reg |= uint64_t(signal) << offset;
        }
    }
    return reg;
}

void process_all_gates(vector<Gate>& gates, unordered_map<string, uint8_t>& signals)
{
    size_t count = 0;

    for (Gate& gate : gates)
        gate.processed = false;

    while (count < gates.size()) {
        for (Gate& gate : gates) {
            if (!contains(signals, gate.in1) || !contains(signals, gate.in2))
                continue;
            if (gate.processed)
                continue;

            process_gate(gate, signals);
            count++;
        }
    }
}

string reg_name(char prefix, int num)
{
    char s[5];
    sprintf(s, "%c%02d", prefix, num);
    return string(s);
}

void set_register(char reg_prefix, uint64_t value, unordered_map<string, uint8_t>& signals)
{
    for (int i = 0; i < 64; ++i) {
        signals[reg_name(reg_prefix, i)] = (value % 2);
        value >>= 2;
    }
}

vector<string> dependencies(string_view name, const vector<Gate>& gates)
{
    unordered_set<string> deps;
    deps.insert(string(name));
    while (true) {
        const int count = deps.size();
        for (const auto& gate : gates) {
            if (deps.find(gate.out) != cend(deps)) {
                deps.insert(gate.in1);
                deps.insert(gate.in2);
            }
        }
        if (deps.size() == count)
            break;
    }
    return vector<string>(cbegin(deps), cend(deps));
}

int bitsum(uint64_t x)
{
    int count = 0;
    while (x) {
        count += x % 2;
        x >>= 1;
    }
    return count;
}

// inneficient, but ok
int find_gate(vector<Gate>& gates,
        GateType type, string_view in_a, string_view in_b,
        set<string> &swaps) {

    const int n = gates.size();
    for (int i = 0; i < n; ++i) {
        if (gates[i].type == type &&
                ((gates[i].in1 == in_a && gates[i].in2 == in_b) ||
                (gates[i].in1 == in_b && gates[i].in2 == in_a))) {
            return i;
        }
    }

    // if we cannot find the gate, look for a gate that has the same port and one of the wires
    // the different wire must have been swapped

    for (int i = 0; i < n; ++i) {
        if (gates[i].type == type &&
                (gates[i].in1 == in_a || gates[i].in2 == in_a)) {
            swaps.insert(string(in_b));
            swaps.insert(string(gates[i].in1 == in_a ? gates[i].in2 : gates[i].in1));
            return i;
        }
    }

    for (int i = 0; i < n; ++i) {
        if (gates[i].type == type &&
                (gates[i].in1 == in_b || gates[i].in2 == in_b)) {
            swaps.insert(string(in_a));
            swaps.insert(string(gates[i].in1 == in_b ? gates[i].in2 : gates[i].in1));
            return i;
        }
    }
    assert(0);
}

// the circuit implements a full ripple adder
// got a hint on this https://en.wikipedia.org/wiki/Adder_(electronics)
vector<string> check_ripple_adder(vector<Gate>& gates, int ndigits) {

    set<string> swaps;

    string prev_carry = "";
    for (uint8_t d = 0; d < ndigits; ++d) {
        const string xname = reg_name('x', d);
        const string yname = reg_name('y', d);

        string sum = gates[find_gate(gates, XOR, xname, yname, swaps)].out;
        string carry = gates[find_gate(gates, AND, xname, yname, swaps)].out;

        if (prev_carry == "") {
            prev_carry = carry;
        } else {
            string new_carry = gates[find_gate(gates, AND, prev_carry, sum, swaps)].out;
            string final_sum = gates[find_gate(gates, XOR, prev_carry, sum, swaps)].out;
            string sum_carry = gates[find_gate(gates, AND, sum, prev_carry, swaps)].out;
            string final_carry = gates[find_gate(gates, OR, sum_carry, carry, swaps)].out;
            prev_carry = final_carry;
        }
    }
    return vector<string>(cbegin(swaps), cend(swaps));
}

void print_swapped_gates(const vector<string> &swaps) {
    const int n = swaps.size();
    for (int i = 0; i < n; ++i) {
        printf("%s", swaps[i].c_str());
        if (i != n-1) {
            printf(",");
        }
    }
    printf("\n");
}

int main()
{
    unordered_map<string, uint8_t> signals_orig = read_signals_input();
    unordered_map<string, uint8_t> signals { signals_orig };
    vector<Gate> gates_orig = read_gates_input();
    vector<Gate> gates { gates_orig };

    process_all_gates(gates, signals);
    const uint64_t z = register_value('z', signals);
    printf("Part #1: Z = %lu\n", z);

    const int MAX_DIGITS = 45; //the input goes form 00 to 44 bits
    vector<string> swaps = check_ripple_adder(gates, MAX_DIGITS);
    printf("Part #2: ");
    print_swapped_gates(swaps);

    return 0;
}
