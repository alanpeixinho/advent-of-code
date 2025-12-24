#include <iostream>
#include <string>
#include <deque>
#include <limits>
#include <string_view>
#include <unordered_map>
#include <unordered_set>
#include <vector>
#include <glpk.h>

using namespace std;

int parse_ligths(string_view lights_str) {
    int state = 0;
    int i;
    for (i = 1; lights_str[i] != ']'; ++i) {
        state = (state << 1) + (lights_str[i] == '#');
    }
    return state;
}

inline int ctoi(char c) {
    return c - '0';
}

vector<int> parse_buttons(string_view buttons_str, int nbits) {

    vector<int> buttons;
    int state = 0;
    int i = 0;
    while (i < buttons_str.size()) {
        if (buttons_str[i] == '(')
            state = 0;
        else {
            int cur = 0;
            while (isdigit(buttons_str[i])) {
                cur = cur * 10 + ctoi(buttons_str[i]);
                ++i;
            }
            state |= 1 << (nbits - 1 - cur);
            if (buttons_str[i] == ')')
                buttons.push_back(state);
        }
        ++i;
    }

    return buttons;
}

vector<int> parse_energy(string_view energy_str) {
    vector<int> energies;
    int i = 1;
    while (i < energy_str.size()) {
        int cur = 0;
        while (isdigit(energy_str[i])) {
            cur = cur * 10 + ctoi(energy_str[i]);
            ++i;
        }
        if (energy_str[i] == ',' || energy_str[i] == '}')
            energies.push_back(cur);
        ++i;
    }
    return energies;
}

string_view substr(string_view s, char begin, char end) {
    const size_t begin_pos = s.find_first_of(begin);
    const size_t end_pos = s.find_last_of(end);
    return { &s[begin_pos], end_pos - begin_pos + 1 };
}

tuple<int, vector<int>, vector<int>> read_input() {
    string line;
    getline(cin, line);
    if (line.empty()) {
        return {};
    }
    const string_view s { line };
    const int lights = parse_ligths(substr(s, '[', ']'));
    const vector<int> energy = parse_energy(substr(s, '{', '}'));
    const vector<int> buttons = parse_buttons(substr(s, '(', ')'), energy.size());

    return { lights, buttons, energy };
}

int bfs(int end, const vector<int>& edges) {
    const int start = 0;
    deque<pair<int, int>> q;
    unordered_set<int> visited;

    q.push_back({start, 0});

    while (!q.empty()) {
        const auto& [cur, level] = q.front(); q.pop_front();
        if (cur == end)
            return level;
        visited.insert(cur);
        for (const int edge : edges) {
            const int next = cur ^ edge;
            if (visited.find(next) == cend(visited))
                q.push_back({next, level + 1});
        }
    }

    return numeric_limits<int>::max();
}

inline bool is_bit_on(int x, int bit) {
    return (x & (1 << bit)) > 0;
}

int integer_programming_solve(const vector<int>& buttons, const vector<int>& energy) {

    /* we map the problem as optimizing
     * min b_1 + b_2 + ... b_n (sum of buttons presses)
     * subject to
     * sum b_j == energy_i (where b_j are all buttons that modify energy counter i)
     * */

    const int nconstraints = energy.size();
    const int nvariables = buttons.size();

    glp_prob* problem = glp_create_prob();
    glp_set_obj_dir(problem, GLP_MIN);

    glp_add_rows(problem, nconstraints);
    glp_add_cols(problem, nvariables);

    for (int col = 0; col < nvariables; ++col) {
        glp_set_col_kind(problem, col + 1, GLP_IV); // integer variable (no float value)
        glp_set_col_bnds(problem, col + 1, GLP_LO, 0.0, 0.0); // >=0
        glp_set_obj_coef(problem, col + 1, 1.0); // all buttons have same cost 1.0
    }

    for (int row = 0; row < nconstraints; ++row)
        glp_set_row_bnds(problem, row + 1, GLP_FX, energy[row], energy[row]); // ==energy_i

    // load matrix
    // coeficients and constraints start at index 1
    vector<int> coef_rows = { 0 };
    vector<int> coef_cols = { 0 };
    vector<double> coef_vals = { 0 };

    for (int row = 0; row < nconstraints; ++row) {
        for (int col = 0; col < nvariables; ++col) {
            if (is_bit_on(buttons[col], energy.size() - 1 - row)) {
                coef_rows.push_back(row + 1);
                coef_cols.push_back(col + 1);
                coef_vals.push_back(1.0);
            }
        }
    }

    glp_load_matrix(problem, coef_vals.size() - 1,
            coef_rows.data(), coef_cols.data(), coef_vals.data());


    glp_iocp param;
    glp_init_iocp(&param);
    param.presolve = GLP_ON;
    param.msg_lev = GLP_MSG_ERR;
    //glp_simplex(problem, &param);
    glp_intopt(problem, &param);

    const double min = glp_mip_obj_val(problem);

    glp_delete_prob(problem);
    return min;
}


int main() {
    int total = 0;
    int total_2 = 0;
    while (true) {
        const auto& [state, buttons, energy] = read_input();
        if (state == 0 && buttons.empty() && energy.empty())
            break;

        total += bfs(state, buttons);
        total_2 += integer_programming_solve(buttons, energy);
    }
    printf("Total: %d\n", total);
    printf("Total Energy counter: %d\n", total_2);
    return 0;
}
