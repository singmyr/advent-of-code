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
        
        if (direction == 'L' && position > 0 && steps >= position) {
            count++;
        } else if (direction == 'R' && position < 0 && steps >= abs(position)) {
            count++;
        }

        position += change;

        while (position >= 100) {
            count++;
            position -= 100;
        }

        while (position <= -100) {
            count++;
            position += 100;
        }
    }

    cout << count << endl;
    
    file.close();

    return 0;
}