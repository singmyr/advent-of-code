#include <cstdint>
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
using namespace std;

uint64_t calculateInvalidsSum(uint64_t min, uint64_t max) {
    uint64_t sum = 0;

    for (uint64_t i = min; i <= max; ++i) {
        string cur = to_string(i);
        int len = cur.length();
        for (int pieces = 2; pieces <= len; ++pieces) {
            if (len % pieces > 0) {
                continue;
            }

            bool is_pieces_invalid = true;
            int piece_size = len / pieces;
            string base = cur.substr(0, piece_size);
            for (int piece = 1; piece < pieces; ++piece) {
                int piece_start = piece * piece_size;
                string compare = cur.substr(piece_start, piece_size);
                if (base != compare) {
                    is_pieces_invalid = false;
                    break;
                }
            }

            if (is_pieces_invalid) {
                sum += i;
                break;
            }
        }
    }

    return sum;
}

int main() {
    ifstream file("build/input.txt");

    stringstream ss;

    string ranges[2];
    int part = 0;
    char ch;
    uint64_t sum = 0;
    while (file >> noskipws >> ch) {
        if (ch == '-' || ch == ',') {
            ranges[part] = ss.str();
            part = !part;
            ss.str("");
        } else {
            ss << ch;
        }

        if (ch == ',') {
            sum += calculateInvalidsSum(stoull(ranges[0]), stoull(ranges[1]));
        }
    }

    ranges[part] = ss.str();
    part = !part;
    ss.str("");
    sum += calculateInvalidsSum(stoull(ranges[0]), stoull(ranges[1]));

    cout << sum << endl;
    
    file.close();

    return 0;
}