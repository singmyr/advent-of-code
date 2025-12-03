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
        if (len > 1 && (len & 1) == 0) {
            int breakpoint = len / 2;
            string first = cur.substr(0, breakpoint);
            string second = cur.substr(breakpoint);
            if (first == second) {
                sum += i;
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