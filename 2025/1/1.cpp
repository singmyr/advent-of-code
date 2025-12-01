#include <iostream>
#include <fstream>
using namespace std;

int main() {
    ifstream file("build/input.txt");

    char direction;
    int steps;

    int count = 0;
    int position = 50;
    int change = 0;
    while (file >> direction >> steps) {
        change = direction == 'R' ? steps : (0 - steps);
        
        position += change;
        position %= 100;

        if (position == 0) {
            count++;
        }
    }

    cout << count << endl;
    
    file.close();

    return 0;
}